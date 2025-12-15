---
title: "Building the Boon playground"
description: "Notes on experimenting with a Rust-inspired language for web apps."
sortOrder: 1
tags: ["boon", "rust", "wasm"]
img: "/images/boon_playground.png"
img_alt: "Boon playground UI"
---

Most of my experiments start as tiny Rust prototypes. Boon grew from the question: how far can we push a declarative language that keeps runtime values around, even after a hot reload?

I used the MoonZoon framework to build the browser playground. It let me focus on the language and interactive editor instead of scaffolding another frontend stack.

The current iteration keeps state across reloads, leans on streams for data flow, and compiles to WebAssembly. Next steps are better developer tooling and shipping the first production app, Persons.pro, on top of it.
