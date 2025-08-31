import { config } from "dotenv";
import { expand } from "dotenv-expand";
import type { ZodError } from "zod";
import { z } from "zod";

expand(config({ path: "../../.env" }));

const EnvSchema = z.object({
  PORT: z.coerce.number().default(8090),
  OPAQUE_SERVER_SETUP: z.base64url(),
  DISABLE_FS: z.boolean().default(false),
});

export type env = z.infer<typeof EnvSchema>;
let env: env;

try {
  // biome-ignore lint/style/noProcessEnv: this is the only place where we should access env vars directly
  env = EnvSchema.parse(process.env) satisfies env;
} catch (e) {
  const error = e as ZodError;
  console.error("❌ Invalid env:");
  console.error(error.flatten().fieldErrors);
  process.exit(1);
}

export default env;
