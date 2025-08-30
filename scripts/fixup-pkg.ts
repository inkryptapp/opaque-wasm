#!/usr/bin/env node

import * as fs from "node:fs/promises";
import * as path from "node:path";

async function fixupPkg() {
  const scriptsDir = path.join(process.cwd(), "scripts");
  const pkgDir = path.join(process.cwd(), "pkg");
  const esmDir = path.join(pkgDir, "esm");
  const cjsDir = path.join(pkgDir, "cjs");

  try {
    // Move LICENSE, .gitignore, README.md from `pkg/esm` to `pkg` (overwrite existing)
    const filesToMove = ["LICENSE", ".gitignore", "README.md"];

    for (const file of filesToMove) {
      const srcPath = path.join(esmDir, file);
      const destPath = path.join(pkgDir, file);

      try {
        await fs.copyFile(srcPath, destPath);
      } catch (error) {
        console.warn(`❌ Failed to move ${file}: ${error}`);
      }
    }

    const filesToDelete = [...filesToMove, "package.json"];

    // Delete duplicates
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

    // Copy package.template.json to pkg/package.json
    const templatePath = path.join(scriptsDir, "package.template.json");
    const packageJsonPath = path.join(pkgDir, "package.json");

    try {
      await fs.copyFile(templatePath, packageJsonPath);
    } catch (error) {
      console.error(`❌ Failed to copy package.template.json: ${error}`);
    }

    console.log("✅ Package fixup completed successfully!");
  } catch (error) {
    console.error("❌ Package fixup failed:", error);
    process.exit(1);
  }
}

fixupPkg();
