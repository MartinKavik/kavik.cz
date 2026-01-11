---
title: "MoonZoon"
description: "Rust fullstack framework"
sortOrder: 3
tags: ["rust", "fullstack", "framework", "wasm", "signals"]
img: "/images/moonzoon-logo.png"
img_alt: "MoonZoon logo"
links:
  - name: "GitHub"
    url: "https://github.com/MoonZoon/MoonZoon"
---

Fullstack Rust without HTML, CSS, or JavaScript. Wasm frontend with signals (no Virtual DOM), native Rust backend. Typed routes and `UpMsg`/`DownMsg` for real-time communication. Auto-reload with `mzoon` CLI. Also works as frontend-only for static hosting.

Boon Playground and NovyWave are built with MoonZoon.

<details>
<summary>MoonZoon history</summary>

After maintaining Seed (early Rust frontend framework), I got frustrated with VirtualDOM and Elm-style patterns. Even created Rust React Hooks — with less limitations than JS ones — but still didn't feel right. Writing HTML/CSS almost directly felt worse the more I dealt with CSS interactions and browser inconsistencies — complexity kept growing instead of shrinking.

Signals and simplified styling felt better. Typed messages simplified frontend-backend communication. That became MoonZoon in 2021. Since then I've pushed its limits: web workers with shared memory, desktop apps, animations, JS interop — and contributed to `wasm-bindgen` along the way.

</details>

![Counter button example](/images/moonzoon_counter_button.png)

![Send message example](/images/moonzoon_send_message.png)
