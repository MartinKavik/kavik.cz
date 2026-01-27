---
title: "Boon"
description: "One language for web apps, CLIs and hardware"
sortOrder: 1
tags: ["language", "rust", "declarative", "persistent", "WIP"]
img: "/images/boon_logo.png"
img_alt: "Boon - Timeless & Playful Language"
links:
  - name: "Website"
    url: "https://boon.run"
  - name: "GitHub"
    url: "https://github.com/BoonLang/boon"
  - name: "Playground"
    url: "https://play.boon.run"
---

Boon is a declarative language where the runtime remembers values across reloads, enabling hot-reloading and state persistence without a database.

Statically typed with a Factorio-inspired visual debugger. Other inspirations described in the blog post [The Boon of Dataflow](/blog/the-boon-of-dataflow).

Currently powered by a Rust interpreter with a custom lock-free actor-based engine. Exploring Differential Dataflow as an alternative — engines switchable in the Playground.

<details>
<summary>Related work</summary>

1. **MoonZoon** — powers Boon Playground and the HTML renderer; its signals and styling influenced Boon's design
2. **UI Parser** — extracts UI metadata from web pages for UI recreation and visual testing in Boon
3. **RayBox** — 3D SDF renderer that will render Boon UIs with real depth and lighting
4. **BoonComputer** — RISC CPU on FPGA proving Boon describes hardware as naturally as software

</details>

<br>

![Boon interval example](/images/boon_interval_small.png)
