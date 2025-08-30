#!/usr/bin/env node

import * as fs from "node:fs/promises";
import * as path from "node:path";

async function fixupPkg() {
  const pkgDir = path.join(process.cwd(), "pkg");
  const esmDir = path.join(pkgDir, "esm");
  const cjsDir = path.join(pkgDir, "cjs");

  try {
    // Move LICENSE and README.md from `pkg/esm` to `pkg` (overwrite existing)
    const filesToMove = ["LICENSE", "README.md"];

    for (const file of filesToMove) {
      const srcPath = path.join(esmDir, file);
      const destPath = path.join(pkgDir, file);

      try {
        await fs.copyFile(srcPath, destPath);
      } catch (error) {
        console.warn(`❌ Failed to move ${file}: ${error}`);
      }
    }

    // Delete duplicates
    const filesToDelete = [...filesToMove, ".gitignore"];
    for (const file of filesToDelete) {
      try {
        await fs.unlink(path.join(esmDir, file));
      } catch (error) {
        console.warn(`❌ Failed to delete pkg/esm/${file}: ${error}`);
      }

      try {
        await fs.unlink(path.join(cjsDir, file));
      } catch (error) {
        console.warn(`❌ Failed to delete pkg/cjs/${file}: ${error}`);
      }
    }

    console.log("✅ Package fixup completed successfully!");
  } catch (error) {
    console.error("❌ Package fixup failed:", error);
    process.exit(1);
  }
}

fixupPkg();
