---
title: "Prompter"
description: "Web UI for Claude Code sessions"
sortOrder: 10
tags: ["rust", "cli", "ai", "pty", "axum"]
img: "/images/prompter.png"
img_alt: "Prompter web interface"
links:
  - name: "GitHub"
    url: "https://github.com/MartinKavik/prompter"
---

Prompter is a Rust CLI that wraps Claude Code in a pseudo-terminal while exposing a web dashboard for monitoring and interacting with AI sessions.

It captures conversation history and makes it accessible via a real-time web interface, allowing both terminal and browser-based interaction with Claude simultaneously.

Key capabilities:

- Full PTY passthrough for transparent Claude Code usage
- Automatic session detection and transcript monitoring
- Web UI showing recent prompts and answers with live updates via SSE
- Dual input from terminal and web interface
- Proper signal forwarding for clean process management
