---
title: "UI Parser"
description: "Extract UI metadata from web pages"
sortOrder: 11
tags: ["rust", "extension", "cdp", "typescript", "mcp", "WIP"]
img: "/images/ui_parser.png"
img_alt: "UI Parser overlay visualization"
---

UI Parser extracts visual metadata from web pages for UI recreation in Boon or other languages, and for integration tests that verify rendered output.

What it captures:

- Element bounds and positions
- Computed styles
- Text and pseudo-element sizes
- Actually used fonts
- Layout relationships

A Chromium TypeScript extension adds a new tab to DevTools, similar to the standard Elements panel.

![UI Parser architecture](/images/ui-parser-architecture.png)
