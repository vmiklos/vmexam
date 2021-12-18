#define WASM_EXPORT __attribute__((visibility("default"))) extern "C"

WASM_EXPORT int sum(int a, int b) { return a + b; }
