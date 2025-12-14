---
title: "RayBox"
description: "WebGPU renderer for web UI"
sortOrder: 4
tags: ["rust", "webgpu", "wasm", "rendering", "canvas"]
img: "/images/raybox-screenshot.png"
img_alt: "RayBox rendering TodoMVC in WebGPU"
---

RayBox is a proof-of-concept WebGPU renderer that draws web UI elements entirely on the GPU.
It renders TodoMVC with 97.74% visual similarity to Chrome's native rendering, demonstrating that GPU-accelerated web UIs are practical.

The hybrid Canvas2D + WebGPU approach keeps text rendering simple while pushing rectangles, borders, and backgrounds to the GPU.
100% Rust, compiles to WebAssembly, and idles at less than 5% CPU.

Technical highlights:

- Instanced rendering for rectangles and borders in a single draw call
- Text rendered to Canvas2D, then uploaded as GPU textures
- On-demand rendering instead of continuous animation loops
- Complete Rust toolchain for development, testing, and screenshots
