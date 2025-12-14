---
title: "Boon"
description: "Programming language for writing apps"
sortOrder: 1
tags: ["language", "rust", "functional", "persistent"]
img: "/images/boon_playground.png"
img_alt: "Boon playground running in the browser"
links:
  - name: "GitHub"
    url: "https://github.com/BoonLang/boon"
  - name: "Playground"
    url: "https://boon-playground.kavik.cz"
---

Boon is a declarative, statically-typed language that leans on Rust streams to keep application state coherent.
The runtime remembers values across reloads, which unlocks website hot-reloading and state persistence without bolting on a database.
It also avoids locks in the runtime so multi-threaded web front-end development stays straightforward.

The goal is to answer a few open questions:

- How simple can a useful language be?
- Can storage, threads, and protocols be made transparent to the developer?
- Can an app be stopped and later resumed without losing momentum?

The first production app written in Boon will be Persons.pro. A browser playground built with the Rust MoonZoon framework shows how the language behaves in real projects.
