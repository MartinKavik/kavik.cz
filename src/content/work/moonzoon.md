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

[Boon Playground](https://play.boon.run/) and [NovyWave](https://github.com/NovyWave/NovyWave) are built with MoonZoon. It also served as the foundation for a [Norwegian bachelor thesis](https://nva.sikt.no/registration/0198eb412387-3623524c-66da-4402-8866-164edb5c80ea).

<details>
<summary>MoonZoon history</summary>

After maintaining Seed (early Rust frontend framework), I got frustrated with VirtualDOM and Elm-style patterns. Even created Rust React Hooks — with less limitations than JS ones — but still didn't feel right. Writing HTML/CSS almost directly felt worse the more I dealt with CSS interactions and browser inconsistencies — complexity kept growing instead of shrinking.

Signals and simplified styling felt better. Typed messages simplified frontend-backend communication. That became MoonZoon in 2021. Since then I've pushed its limits: web workers with shared memory, desktop apps, animations, JS interop — and contributed to `wasm-bindgen` along the way.

Read more in [MoonZoon: Foundations](/blog/moonzoon-foundations-2021) and [MoonZoon: 2021–2025](/blog/moonzoon-2021-2025).

</details>

![Counter button example](/images/moonzoon_counter_button.png)

![Send message example](/images/moonzoon_send_message.png)
