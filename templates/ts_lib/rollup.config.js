import esbuild from "rollup-plugin-esbuild";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import { glob } from "glob";
import path from "node:path";
import { fileURLToPath } from "node:url";

/** @type {import('rollup').RollupOptions} */
export default {
  input: Object.fromEntries(
    glob
      .sync("./src/**/*.ts")
      .map((file) => [
        path.relative(
          "src",
          file.slice(0, file.length - path.extname(file).length)
        ),
        fileURLToPath(new URL(file, import.meta.url)),
      ])
  ),
  output: [
    {
      format: "es",
      entryFileNames: "[name].mjs",
      dir: "esm",
      preserveModules: true,
      sourcemap: true,
    },
    {
      format: "cjs",
      entryFileNames: "[name].cjs",
      dir: "cjs",
      preserveModules: true,
      sourcemap: true,
    },
  ],
  plugins: [nodeResolve(), esbuild({ tsconfig: "tsconfig.json" })],
};
