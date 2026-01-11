---
title: "RayBox"
description: "3D web UI renderer with SDF and WebGPU"
sortOrder: 5
tags: ["rust", "webgpu", "wasm", "sdf", "3d", "WIP"]
img: "/images/raybox.jpg"
img_alt: "RayBox - 3D web UI with SDF and WebGPU"
---

RayBox renders web UI as real 3D objects using Signed Distance Functions and WebGPU.

The idea is simple: buttons, inputs, and dialogs should be physical. A button has actual depth and sits closer to the camera. A dialog floats in front of content and casts real shadows. Inset effects emerge naturally from geometry instead of being faked with CSS.

The current version renders TodoMVC with 97.74% visual similarity to Chrome as a proof-of-concept. The next step is replacing flat rectangles with SDF-defined 3D shapes where shadows, highlights, and depth come from actual lighting and geometry.

Built entirely in Rust, compiles to WebAssembly. The same SDF models could eventually export to 3D printers.
