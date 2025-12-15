---
title: "LocalSearch"
description: "Client-side full-text search library"
sortOrder: 10
tags: ["search", "full-text", "autocomplete", "rust", "wasm"]
img: "/images/localsearch.jpg"
img_alt: "LocalSearch - client-side full-text search"
links:
  - name: "GitHub"
    url: "https://github.com/Stremio/local-search"
---

LocalSearch delivers fast, client-side full-text search written in Rust and compiled to WebAssembly.
It keeps indices in the browser, which means no round-trips for autocomplete or fuzzy lookups.

I use it to keep personal projects snappy without adding another backend dependency, and to experiment with how much UX polish we can get when search feels instant.
