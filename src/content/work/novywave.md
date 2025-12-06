---
title: "NovyWave"
description: "Modern GTKWave alternative"
publishDate: 2024-12-15
tags: ["waveform", "viewer", "wasm", "fpga", "tauri"]
img: "/images/figma_fastwave.png"
img_alt: "NovyWave waveform viewer concept in Figma"
links:
  - name: "GitHub"
    url: "https://github.com/NovyWave/NovyWave"
---

NovyWave aims to modernize the waveform viewer experience for FPGA and hardware debugging.
The project mixes a WebAssembly front-end with a Tauri desktop shell so large capture files stay fast while the UI remains portable.

I worked on the interaction model, accessibility of dense traces, and fast zooming/panning so engineers can stay in flow instead of waiting on redraws.
