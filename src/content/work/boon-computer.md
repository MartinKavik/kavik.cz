---
title: "BoonComputer"
description: "RISC CPU on FPGA — designed in Boon, running Boon"
sortOrder: 4
tags: ["boon", "risc", "fpga", "cpu", "hardware", "WIP"]
img: "/images/boon_computer.jpg"
img_alt: "BoonComputer - RISC CPU on FPGA"
---

Proving Boon can describe hardware as naturally as software — the same language for web apps and CPU architecture.

**Roadmap:**
1. Write a transpiler from Boon to Verilog for simple circuits
2. Test inside a forked DigitalJS with UART debugging and Wasm Yosys
3. Test on real hardware with peripherals like displays — for fun and confidence
4. Build SuperCounter (web app, server, CLI, FPGA) all in Boon, working together
5. Then the real RISC work begins

**Ultimate goal:** run Boon in Wasm executed on a CPU designed in Boon.

Future possibility: MRAM integration — instant resume, zero data loss, minimal standby power.

![Raspberry Pi and iCESugar FPGA](/images/raspberry_pi_and_icesugar.jpg)
