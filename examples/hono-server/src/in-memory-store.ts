import { readFileSync } from "node:fs";
import { writeFile } from "node:fs/promises";

const MILLISECONDS_PER_DAY =
  24 /*hours*/ * 60 /*minutes*/ * 60 /*seconds*/ * 1000; /*milliseconds*/

interface SessionData {
  userIdentifier: string;
  sessionKey: string;
}

interface LoginEntry {
  value: string;
  timestamp: number;
}

interface SessionEntry extends SessionData {
  expiresAt: number;
}

export default class InMemoryStore {
  private users: Record<string, string>;
  private logins: Record<string, LoginEntry>;
  private listeners: (() => void)[];
  private sessions: Record<string, SessionEntry>;

  constructor(
    users: Record<string, string>,
    logins: Record<string, LoginEntry>
  ) {
    this.users = users;
    this.logins = logins;
    this.listeners = [];
    this.sessions = {};
  }

  addListener(listener: () => void): () => void {
    this.listeners.push(listener);
    return () => {
      const index = this.listeners.indexOf(listener);
      if (index !== -1) {
        this.listeners.splice(index, 1);
      }
    };
  }

  private _notifyListeners(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }

  static empty(): InMemoryStore {
    return new InMemoryStore({}, {});
  }

  stringify(): string {
    return JSON.stringify(
      {
        logins: this.logins,
        users: this.users,
      },
      null,
      2
    );
  }

  async getUser(name: string): Promise<string | null> {
    return this.users[name] || null;
  }

  async hasUser(name: string): Promise<boolean> {
    return this.users[name] != null;
  }

  async getLogin(name: string): Promise<string | null> {
    const hasLogin = await this.hasLogin(name);
    return hasLogin ? this.logins[name].value : null;
  }

  async hasLogin(name: string): Promise<boolean> {
    const login = this.logins[name];
    if (login == null) return false;
    const now = Date.now();
    const elapsed = now - login.timestamp;
    return elapsed < 2000;
  }

  async setUser(name: string, value: string): Promise<void> {
    this.users[name] = value;
    this._notifyListeners();
  }

  async setLogin(name: string, value: string): Promise<void> {
    this.logins[name] = { value, timestamp: Date.now() };
    this._notifyListeners();
  }

  async removeLogin(name: string): Promise<void> {
    delete this.logins[name];
    this._notifyListeners();
  }

  async getSession(id: string): Promise<SessionData | null> {
    const session = this.sessions[id];
    if (session == null) return null;
    const { expiresAt, ...sessionData } = session;
    if (expiresAt < Date.now()) {
      await this.clearSession(id);
      return null;
    }
    return sessionData;
  }

  async setSession(
    id: string,
    session: SessionData,
    lifetimeInDays: number = 14
  ): Promise<void> {
    const expiresAt = Date.now() + lifetimeInDays * MILLISECONDS_PER_DAY;
    this.sessions[id] = { ...session, expiresAt };
  }

  async clearSession(id: string): Promise<void> {
    delete this.sessions[id];
  }
}

export function readDatabaseFile(filePath: string): InMemoryStore {
  const json = readFileSync(filePath, "utf-8");
  const data = JSON.parse(json);
  const db = new InMemoryStore(data.users, data.logins);
  return db;
}

export function writeDatabaseFile(
  filePath: string,
  db: InMemoryStore
): Promise<void> {
  const data = db.stringify();
  return writeFile(filePath, data);
}
