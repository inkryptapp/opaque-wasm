# 🔐 opaque-wasm: Hono Server Example

A minimal [Hono](https://hono.dev/) server set up to try `opaque-wasm` out in a backend environment.

## Prerequisites

- Node.js v22+
- `pnpm` installed globally (optional, `npm i -g pnpm`)

## Quick Start

This quick guide assumes you've already run `pnpm generate-dotenv` in the root folder, which generates the required private key for the server.

```bash
pnpm install
pnpm dev
```

Then open `http://localhost:8090` in your browser. You should see:

```
Hello Hono!
```

## Notes

- This example is intentionally minimal to keep focus on verifying your setup. Extend the routes as needed for your use case.
