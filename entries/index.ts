import "../css/styles.css";

declare global {
  interface Window {
    // We want to save imported rust module (resp. its JS loader) into global scope,
    // because we will not be able to import it again (for types, enums, etc.)
    // because of side-effects like loading wasm file
    rustModule: typeof import("../crate/pkg/index");
  }
}

(async () => {
  // We have to load wasm async
  // Note: files in crate/pkg/ will be created on first build
  window.rustModule = await import("../crate/pkg/index");
})();
