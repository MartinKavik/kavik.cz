---
title: "UI Parser"
description: "Extract UI metadata from HTML+CSS pages"
sortOrder: 12
tags: ["rust", "chrome-extension", "cdp", "typescript", "mcp"]
img: "/images/ui_parser.png"
img_alt: "UI Parser overlay visualization"
---

UI Parser extracts visual metadata from web pages for framework-agnostic recreation. It captures element positions, styles, and layout relationships so designs can be recreated in systems like Boon without reverse-engineering logic.

A Chrome extension uses the DevTools Protocol for pixel-perfect element bounds, rendering an interactive canvas overlay with color-coded container types. The Rust backend coordinates via WebSocket and integrates with Claude Code through MCP.

Key capabilities:

- CDP-based element detection with accurate bounds for text and images
- Container type inference (Row, Column, Grid, Button, Link, Stack)
- Computed CSS extraction including flexbox, grid, colors, and fonts
- Pseudo-element and pseudo-class state capture
- Interactive overlay with hover labels and element selection
