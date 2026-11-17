// Runs a contract check exported by the WebAssembly fixture under Node.
//
// Usage: node run.mjs <module.wasm> [export]
//
// Exit codes:
//   0  the export returned, and at least one check has passed;
//   1  the module trapped, which is what a failed check does;
//   2  the harness could not run the export.

import { readFileSync } from 'node:fs';

const [, , path, entry = 'run'] = process.argv;

function fail(message) {
  console.error(`run.mjs: ${message}`);
  process.exit(2);
}

if (!path) {
  fail('usage: node run.mjs <module.wasm> [export]');
}

let exports;
try {
  // The module is self-contained: it has no imports.
  const { instance } = await WebAssembly.instantiate(readFileSync(path), {});
  exports = instance.exports;
} catch (error) {
  fail(`cannot instantiate ${path}: ${error.message}`);
}

if (typeof exports[entry] !== 'function') {
  fail(`${path} does not export a function named "${entry}"`);
}

let checks;
try {
  checks = exports[entry]();
} catch (error) {
  if (!(error instanceof WebAssembly.RuntimeError)) {
    fail(`unexpected error: ${error.message}`);
  }
  console.error(`${entry}: trapped after ${exports.passed()} passed checks (${error.message})`);
  process.exit(1);
}

if (!(checks > 0)) {
  fail(`no checks were executed (returned ${checks})`);
}

console.log(`${entry}: ${checks} contract checks passed on wasm32-unknown-unknown`);
