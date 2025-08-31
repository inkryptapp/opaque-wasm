import * as fs from "node:fs";
import * as path from "node:path";
import * as opaqueWasm from "../pkg";

async function writeDotEnv(): Promise<void> {
  const envFile = path.join(__dirname, "..", ".env");

  if (fs.existsSync(envFile) && !process.argv.includes("--force")) {
    console.log("✅ Opaque `.env` file already exists, skipping `.env` write");
    return;
  }

  const serverSetup = opaqueWasm.createServerSetup();

  const dotEnv = `# generated with \`scripts/generate-dotenv.ts\`
# example server port
PORT=8090
  
# the opaque server setup (private server key)
OPAQUE_SERVER_SETUP=${serverSetup}

# disable filesystem persistence for in-memory db
# DISABLE_FS=true
`;

  fs.writeFileSync(envFile, dotEnv);
  console.log("✅ Your opaque `.env` file is ready!");
}

writeDotEnv().catch((error) => {
  console.error("❌ Error happened while generating `.env` file:", error);
  process.exit(1);
});
