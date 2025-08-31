import initOpaqueWasm, {
  createServerRegistrationResponse,
  createServerSetup,
  finishClientLogin,
  finishClientRegistration,
  finishServerLogin,
  startClientLogin,
  startClientRegistration,
  startServerLogin,
} from "@inkrypt/opaque-wasm";

const API_PREFIX = "/api";

await initOpaqueWasm();

const form = document.getElementById("form") as HTMLFormElement;
const runDemoBtn = document.getElementById("run-demo") as HTMLButtonElement;
const demoLoader = document.getElementById("demo-loader") as HTMLSpanElement;

async function request<T>(
  method: string,
  path: string,
  body?: T
): Promise<Response> {
  console.log(`➡️ ${method} ${API_PREFIX}${path}`, body);
  const res = await fetch(`${API_PREFIX}${path}`, {
    method,
    body: body && JSON.stringify(body),
    headers: { "Content-Type": "application/json" },
  });

  if (!res.ok) {
    const { error } = await res.json();
    console.log(error);
    throw new Error(error);
  }
  return res;
}

async function register(
  userIdentifier: string,
  password: string
): Promise<boolean> {
  const { clientRegistrationState, registrationRequest } =
    startClientRegistration({ password });
  const { registrationResponse } = await request("POST", `/register/start`, {
    userIdentifier,
    registrationRequest,
  }).then((res) => res.json());

  console.log("registrationResponse", registrationResponse);
  const { registrationRecord } = finishClientRegistration({
    clientRegistrationState,
    registrationResponse,
    password,
  });

  const res = await request("POST", `/register/finish`, {
    userIdentifier,
    registrationRecord,
  });
  console.log("finish successful", res.ok);
  return res.ok;
}

async function login(
  userIdentifier: string,
  password: string
): Promise<string | null> {
  const { clientLoginState, startLoginRequest } = startClientLogin({
    password,
  });

  const { loginResponse } = await request("POST", "/login/start", {
    userIdentifier,
    startLoginRequest,
  }).then((res) => res.json());

  const loginResult = finishClientLogin({
    clientLoginState,
    loginResponse,
    password,
  });

  if (!loginResult) {
    return null;
  }
  const { sessionKey, finishLoginRequest } = loginResult;
  const res = await request("POST", "/login/finish", {
    userIdentifier,
    finishLoginRequest,
  });
  return res.ok ? sessionKey : null;
}

form.addEventListener("submit", async (e: SubmitEvent) => {
  e.preventDefault();

  const formData = new FormData(e.target as HTMLFormElement);
  const username = formData.get("username") as string;
  const password = formData.get("password") as string;
  const action = e.submitter ? (e.submitter as HTMLButtonElement).name : "";

  try {
    switch (action) {
      case "login": {
        const sessionKey = await login(username, password);
        if (sessionKey) {
          alert(
            `User "${username}" logged in successfully; sessionKey = ${sessionKey}`
          );
        } else {
          alert(`User "${username}" login failed`);
        }
        break;
      }

      case "register": {
        const ok = await register(username, password);
        if (ok) {
          alert(`User "${username}" registered successfully`);
        } else {
          alert(`Failed to register user "${username}"`);
        }
        break;
      }

      default:
        console.error("Unsupported form action");
        break;
    }
  } catch (err) {
    console.error(err);
    alert(err);
  }
});

runDemoBtn.addEventListener("click", () => {
  const serverSetup = createServerSetup();
  const userIdentifier = "john.doe@example.com";
  const password = "_P4ssw0rd123!";
  runLocalClientServerDemo(serverSetup, userIdentifier, password);
});

function runLocalClientServerDemo(
  serverSetup: string,
  userIdentifier: string,
  password: string
) {
  demoLoader.removeAttribute("hidden");
  const t1 = performance.now();

  console.log("############################################");
  console.log("#                                          #");
  console.log("#   Running Demo Registration/Login Flow   #");
  console.log("#                                          #");
  console.log("############################################");

  console.log({ serverSetup, username: userIdentifier, password });

  console.log();
  console.log("startClientRegistration");
  console.log("-----------------------");
  const { clientRegistrationState, registrationRequest } =
    startClientRegistration({ password });

  console.log({ clientRegistrationState, registrationRequest });

  console.log();
  console.log("createServerRegistrationResponse");
  console.log("-----------------------");
  const { registrationResponse } = createServerRegistrationResponse({
    serverSetup,
    registrationRequest,
    userIdentifier: userIdentifier,
  });

  console.log({ registrationResponse });

  console.log();
  console.log("finishClientRegistration");
  console.log("------------------------");
  const {
    registrationRecord,
    exportKey: clientRegExportKey,
    serverStaticPublicKey: clientRegServerStaticPublicKey,
  } = finishClientRegistration({
    password,
    clientRegistrationState,
    registrationResponse,
  });

  console.log({
    clientRegExportKey,
    clientRegServerStaticPublicKey,
    registrationRecord,
  });

  console.log();
  console.log("startClientLogin");
  console.log("----------------");
  const { clientLoginState, startLoginRequest } = startClientLogin({
    password,
  });

  console.log({ clientLoginState, startLoginRequest });

  console.log();
  console.log("startServerLogin");
  console.log("----------------");
  const { loginResponse, serverLoginState } = startServerLogin({
    userIdentifier: userIdentifier,
    registrationRecord,
    serverSetup,
    startLoginRequest,
  });

  console.log({ loginResponse, serverLoginState });

  console.log();
  console.log("finishClientLogin");
  console.log("-----------------");
  const loginResult = finishClientLogin({
    clientLoginState,
    loginResponse,
    password,
  });

  if (!loginResult) {
    console.log("Login failed: `loginResult` is missing");
    return;
  }

  const {
    finishLoginRequest,
    exportKey: clientLoginExportKey,
    serverStaticPublicKey: clientLoginServerStaticPublicKey,
    sessionKey: clientSessionKey,
  } = loginResult;

  console.log({
    clientLoginExportKey,
    clientSessionKey,
    clientLoginServerStaticPublicKey,
    finishLoginRequest,
  });

  console.log();
  console.log("finishServerLogin");
  console.log("-----------------");
  const { sessionKey: serverSessionKey } = finishServerLogin({
    finishLoginRequest,
    serverLoginState,
  });

  console.log({ serverSessionKey });

  console.log(
    `✅ Done in ${Number((performance.now() - t1) / 1000).toFixed(2)}s`
  );

  demoLoader.setAttribute("hidden", "");
}
