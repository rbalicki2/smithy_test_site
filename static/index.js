const wasm = import("./smithy_test_site");

wasm.then(module => {
  module.start();
});
