import * as opaqueWasm from "@inkrypt/opaque-wasm";
import { randomUUID } from "crypto";
import { Hono } from "hono";
import { deleteCookie, getCookie, setCookie } from "hono/cookie";
import { logger } from "hono/logger";
import env from "./env";
import InMemoryStore, {
  readDatabaseFile,
  writeDatabaseFile,
} from "./in-memory-store";
import {
  LoginFinishParams,
  type LoginFinishType,
  LoginStartParams,
  type LoginStartType,
  RegisterFinishParams,
  type RegisterFinishType,
  RegisterStartParams,
  type RegisterStartType,
} from "./schema";

const DB_FILE = "./sample-db.json";

async function initInMemoryStore(filePath: string): Promise<InMemoryStore> {
  if (env.DISABLE_FS) {
    return InMemoryStore.empty();
  }
  try {
    const db = readDatabaseFile(filePath);
    console.log(`✅ Database successfully initialized from file "${filePath}"`);
    return db;
  } catch (err) {
    if (err instanceof Error && "code" in err && err.code === "ENOENT") {
      console.log(
        `⚠️ no database file "${filePath}" found, initializing an empty database`
      );
    } else {
      console.error(
        `❌ failed to open database file "${filePath}", initializing empty database`,
        err
      );
    }
    const db = InMemoryStore.empty();
    return db;
  }
}

let db: InMemoryStore;

async function setUpInMemoryStore(): Promise<void> {
  const memoryDb = await initInMemoryStore(DB_FILE);

  if (!env.DISABLE_FS) {
    writeDatabaseFile(DB_FILE, memoryDb);
    memoryDb.addListener(() => {
      writeDatabaseFile(DB_FILE, memoryDb);
    });
  }
  db = memoryDb;
}

async function setupDb(): Promise<void> {
  await setUpInMemoryStore();
}

function generateSessionId(): string {
  return randomUUID();
}

await setupDb();
const app = new Hono();

app.use(logger());

app.post("/register/start", async (c) => {
  let userIdentifier: string, registrationRequest: string;
  try {
    const body = await c.req.json();
    const values: RegisterStartType = RegisterStartParams.parse(body);
    userIdentifier = values.userIdentifier;
    registrationRequest = values.registrationRequest;
  } catch (err) {
    console.error(err);
    return c.json({ error: "Invalid input values" }, 400);
  }

  const userExists = await db.hasUser(userIdentifier);
  if (userExists) {
    return c.json({ error: "user already registered" }, 400);
  }

  const { registrationResponse } = opaqueWasm.createServerRegistrationResponse({
    serverSetup: env.OPAQUE_SERVER_SETUP,
    userIdentifier,
    registrationRequest,
  });

  return c.json({ registrationResponse });
});

app.post("/register/finish", async (c) => {
  let userIdentifier: string, registrationRecord: string;
  try {
    const body = await c.req.json();
    const values: RegisterFinishType = RegisterFinishParams.parse(body);
    userIdentifier = values.userIdentifier;
    registrationRecord = values.registrationRecord;
  } catch (err) {
    console.error(err);
    return c.json({ error: "Invalid input values" }, 400);
  }

  const existingUser = await db.getUser(userIdentifier);
  if (!existingUser) {
    await db.setUser(userIdentifier, registrationRecord);
  }

  return c.text("", 200);
});

app.post("/login/start", async (c) => {
  let userIdentifier: string, startLoginRequest: string;
  try {
    const body = await c.req.json();
    const values: LoginStartType = LoginStartParams.parse(body);
    userIdentifier = values.userIdentifier;
    startLoginRequest = values.startLoginRequest;
  } catch {
    return c.json({ error: "Invalid input values" }, 400);
  }

  const registrationRecord = await db.getUser(userIdentifier);
  if (!registrationRecord) return c.json({ error: "user not registered" }, 400);

  const loginExists = await db.hasLogin(userIdentifier);
  if (loginExists) {
    return c.json({ error: "login already started" }, 400);
  }

  const { serverLoginState, loginResponse } = opaqueWasm.startServerLogin({
    serverSetup: env.OPAQUE_SERVER_SETUP,
    userIdentifier,
    registrationRecord,
    startLoginRequest,
  });

  await db.setLogin(userIdentifier, serverLoginState);

  return c.json({ loginResponse });
});

app.post("/login/finish", async (c) => {
  let userIdentifier: string, finishLoginRequest: string;
  try {
    const body = await c.req.json();
    const values: LoginFinishType = LoginFinishParams.parse(body);
    userIdentifier = values.userIdentifier;
    finishLoginRequest = values.finishLoginRequest;
  } catch (err) {
    console.error(err);
    return c.json({ error: "Invalid input values" }, 400);
  }

  const serverLoginState = await db.getLogin(userIdentifier);
  if (!serverLoginState) return c.json({ error: "login not started" }, 400);

  const { sessionKey } = opaqueWasm.finishServerLogin({
    finishLoginRequest,
    serverLoginState,
  });

  const sessionId = generateSessionId();
  await db.setSession(sessionId, { userIdentifier, sessionKey });
  await db.removeLogin(userIdentifier);

  setCookie(c, "session", sessionId, { httpOnly: true });
  return c.text("", 200);
});

app.post("/logout", async (c) => {
  const sessionId = getCookie(c, "session");
  if (!sessionId) return c.json({ error: "not authorized" }, 401);

  const session = await db.getSession(sessionId);
  if (!session) return c.json({ error: "invalid session" }, 401);

  await db.clearSession(sessionId);

  deleteCookie(c, "session");
  return c.text("", 200);
});

app.get("/restricted", async (c) => {
  const sessionId = getCookie(c, "session");
  if (!sessionId) return c.json({ error: "not authorized" }, 401);

  const session = await db.getSession(sessionId);
  if (!session) return c.json({ error: "invalid session" }, 401);

  return c.json({
    message: `👋 Hello "${session.userIdentifier}" from opaque-authenticated world!`,
  });
});

export default app;
