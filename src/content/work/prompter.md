---
title: "Prompter"
description: "Web UI for Claude Code sessions"
sortOrder: 10
tags: ["rust", "cli", "ai", "pty", "axum", "htmx"]
img: "/images/prompter.png"
img_alt: "Prompter web interface"
---

Prompter is a Rust CLI that wraps Claude Code in a pseudo-terminal while exposing a web dashboard for monitoring and interacting with AI sessions.

Why I built it:

- **Mobile AI access**: Tunnel the web interface to your phone and interact with your CLI AI agent from anywhere - on a tram, in a waiting room, or away from your desk
- **Agent supervisor**: Automatically nudges idle AI agents and protects critical files via content hashing - preventing agents from "cheating" by modifying deterministic tests meant to verify their own work
- **Audit trail**: Stores complete prompt history for compliance, auditing, and bureaucratic requirements
