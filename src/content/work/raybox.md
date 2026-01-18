---
title: "RayBox"
description: "3D (web) UI renderer with SDF and PBR"
sortOrder: 5
tags: ["rust", "slang", "wasm", "sdf", "3d", "WIP"]
img: "/images/raybox.jpg"
img_alt: "RayBox - 3D web UI with SDF and WebGPU"
---

RayBox renders UI as real 3D objects using Signed Distance Functions and Physically-Based Rendering.

The idea is simple: buttons, inputs, and dialogs should be physical:

- A button has actual depth and sits closer to the camera.
- A dialog floats in front of content and casts real shadows.
- An inset effect emerges naturally from geometry.

The renderer will use Slang as the shading language, targeting both web and native GPU backends.

**Motivation**: Building a cross-platform graphics library while exploring more intuitive GUI APIs. Future applications may include simple games and 3D printing.
