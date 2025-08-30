#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

async function mergePackageJson(): Promise<void> {
  try {
    const metaPackagePath = resolve("build/package.json");
    const targetPackagePath = resolve("pkg/package.json");

    const [buildPackageContent, pkgPackageContent] = await Promise.all([
      readFile(metaPackagePath, "utf-8"),
      readFile(targetPackagePath, "utf-8"),
    ]);

    const buildPackage = JSON.parse(buildPackageContent);
    const pkgPackage = JSON.parse(pkgPackageContent);

    // keep the ordering of properties
    const mergedPackage = { ...buildPackage };
    for (const [k, v] of Object.entries(pkgPackage)) {
      if (!Object.hasOwn(mergedPackage, k)) {
        mergedPackage[k] = v;
      }
    }

    await writeFile(
      targetPackagePath,
      `${JSON.stringify(mergedPackage, null, 2)}\n`,
    );

    console.log(
      "✅ Successfully merged `build/package.json` into `pkg/package.json`.",
    );
  } catch (error) {
    console.error(
      "❌ Failed to merge package files (build/package.json → pkg/package.json):",
      error,
    );
    process.exit(1);
  }
}

mergePackageJson();
