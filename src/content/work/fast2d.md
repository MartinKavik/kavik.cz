---
title: "Fast2D"
description: "Draw shapes and text using Canvas or wgpu"
publishDate: 2024-07-12
tags: ["2d", "graphics", "canvas", "wgpu", "webgpu", "wasm"]
img: "/images/figma_neobrutalism.png"
img_alt: "Neobrutalist UI concept used for Fast2D demos"
links:
  - name: "GitHub"
    url: "https://github.com/NovyWave/Fast2D"
---

Fast2D is a drawing library that targets both the Canvas Web API and GPU backends such as wgpu/WebGPU.
It grew out of experiments where I needed crisp text and shapes in the browser without sacrificing performance once animations kicked in.

The library is written in Rust and compiled to WebAssembly, allowing the same code to ship to the browser or native targets with minimal changes.
