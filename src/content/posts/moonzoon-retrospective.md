---
title: "MoonZoon: Technical Highlights from Building a Rust Fullstack Framework"
description: "Key technical decisions and innovations from developing MoonZoon - signals over hooks, TEA vs components, and the path to Boon."
sortOrder: 3
tags: ["moonzoon", "rust", "boon", "webdev"]
---

![MoonZoon logo](/images/moonzoon-logo.png)

In early 2021, I started writing a series of technical articles documenting the development of [MoonZoon](https://github.com/MoonZoon/MoonZoon) - a Rust fullstack framework designed to simplify web development. This post summarizes the key technical insights from those articles.

## The Problem: Web Development Complexity

Modern web development has become unnecessarily complex. New developers must learn HTML, CSS, JavaScript, JSON, Git, DNS, HTTP, REST, WebSockets, SQL, and authentication - a daunting prerequisite list. Then comes the decision fatigue: React vs. Vue, MongoDB vs. PostgreSQL, serverless vs. VPS.

The deeper problem? As I wrote in [Cure for Web Development](https://dev.to/martinkavik/cure-for-web-development-nnn):

> "It works, until you try to do the first major refactor - you'll feel like you are going through a minefield."

HTML and CSS, designed for simple web pages decades ago, cannot adequately serve modern applications. We're building skyscrapers on foundations meant for cottages.

## The Architecture Debate: TEA vs Components

One of the most popular articles in the series explored frontend architecture patterns. [MoonZoon Dev News 2](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem) compared The Elm Architecture (TEA) with component-based approaches.

**TEA's strength** is that Model (the global state) provides "a single source of truth for all your data" - you can inspect the complete app state at any moment.

**TEA's weakness** is code fragmentation. Applications consist of "5 big isolated trees - Model, init, Msg, update and view" where modules spread across all of them. Inter-component communication becomes challenging: "you can't create custom subscriptions."

**Component-based approaches** attempt to solve this but introduce their own problems: excessive modularity, abstractions at incorrect levels, and heavy reliance on JavaScript compilation layers. Critically, "almost nobody really knows how to work with them effectively."

MoonZoon's Zoon took a hybrid approach using decorator-based blocks:

```rust
#[s_var]  // state variable
fn counter() -> SVar<i32> { 0 }

#[update]  // update handlers
fn increment() { counter().update(|c| c + 1); }

#[cache]  // computed/derived values
fn doubled() -> i32 { counter().inner() * 2 }

#[cmp]  // component rendering
fn root() -> Cmp {
    col![
        button![button::on_press(increment), "+"],
        text![counter()]
    ]
}
```

This naturally separates TEA's trees into standalone blocks while providing local component state through `cmp_var` - without polluting global state.

## Solving the Conditional Hooks Problem

The most popular article in the series, [MoonZoon Dev News 3](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp), tackled a fundamental problem with React-style hooks.

React's hook system relies on call order to determine identity. The framework assigns unique IDs using a counter:

```
Iteration 1: mike() -> id=1, layla_rose() -> id=2
Iteration 2: mike() -> id=1, amber() -> id=2 (wrong person's data!)
```

When conditional logic changes which functions execute, call IDs become unstable. State incorrectly associates with different components across renders. This is why React requires the "rules of hooks" - no hooks in conditionals.

![Counter button example](/images/moonzoon_counter_button.png)

**The Rust solution**: Rather than relying on indices alone, MoonZoon leveraged Rust's `#[track_caller]` attribute combined with `Location::caller()`. This identifies hook calls by their source code location:

```
Key = (call_index, file_location)
```

This approach proved more robust than React's index-based system, preventing data mismatches even with conditional rendering.

Eventually, MoonZoon moved beyond hooks entirely to a **signals-based reactive system**. Signals provide reactive data binding without requiring a Virtual DOM, eliminating the need for call counters, location tracking overhead, and rules-of-hooks compliance checks.

## Fullstack Architecture

The technical articles documented several key architectural decisions:

### Three-Tier Structure

MoonZoon applications are organized into three crates:
- **Shared** - Serializable types for frontend-backend communication (using `serde_lite` to reduce Wasm file size)
- **Moon** (backend) - Actix-based server with virtual actors
- **Zoon** (frontend) - Reactive UI with signals

![Chat example](/images/moonzoon_send_message.png)

### Live-Reload via SSE

As detailed in [MoonZoon Dev News 1](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6), the dev server uses Server-Sent Events (not WebSockets) for live-reload. The frontend receives two event types:
- **Reload Events** - Trigger page refresh after file changes
- **Backend Build ID Events** - Auto-reload when backend restarts

HTTPS runs during development to mirror production, with HTTP/2 eliminating SSE limitations.

### Warp to Actix Migration

[MoonZoon Dev News 4](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-wasm-pack-installer-57cp) documented the migration from Warp to Actix. The decision prioritized speed, async capabilities, and better Tokio integration. A key simplification: the `main` function became async, eliminating the need for separate `init` functions.

### Error Handling Innovation

The mzoon CLI adopted a two-library approach:
- **anyhow** - Enables early returns via `?` without manual error mapping
- **fehler** - Eliminates verbose `Ok(())` wrapping through the `#[throws]` macro

### Virtual Actors for Sessions

[MoonZoon Dev News 5](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) introduced virtual actors: "sessions are virtual actors managed by the Moon. Each SessionActor represents a live connection between the frontend (Zoon) and backend (Moon)." This enables broadcasting to all clients or targeting individual sessions via correlation IDs.

## Open Source Philosophy

In [Open-Source Roads](https://dev.to/martinkavik/open-source-roads-10a), I explored the sustainability problem through a road metaphor:

> "There are no main roads without side roads, no rivers without brooks and no projects without dependencies."

Just as major highways depend on supporting infrastructure, popular projects rely on lesser-known dependencies. Current dynamics create "voluntary slavery" - talented developers provide essential infrastructure while struggling financially. The article proposed OpenHope, a platform where users pay access fees distributed among chosen maintainers, with major projects allocating roughly 50% of income to supporting their dependencies.

## From MoonZoon to Boon

![Boon logo](/images/boon_logo.png)

Many ideas from MoonZoon carried forward into [Boon](/work/boon). The reactive signals architecture evolved into Boon's dataflow model. The frustration with HTML/CSS abstraction led to exploring new rendering approaches.

The core insight remained: web development is too complex because we're working at the wrong abstraction level. Both MoonZoon and Boon attempt to find that better abstraction - one that makes building web applications feel as natural as describing what you want to build.

Read more about the dataflow philosophy in [The Boon of Dataflow](/blog/the-boon-of-dataflow), or try the [Boon Playground](https://nicemaggieamvmex.boonlang.app/).

---

*These articles were originally published on [dev.to](https://dev.to/martinkavik) between February and July 2021:*

1. [Cure for Web Development](https://dev.to/martinkavik/cure-for-web-development-nnn) - February 2021
2. [Open-Source Roads](https://dev.to/martinkavik/open-source-roads-10a) - February 2021
3. [MoonZoon Dev News 1: CLI, Build Pipeline, Live-Reload, HTTPS](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6) - March 2021
4. [MoonZoon Dev News 2: Live Demo, Zoon, Examples, Architectures](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem) - April 2021
5. [MoonZoon Dev News 3: Signals, React-like Hooks, Optimizations](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp) - May 2021
6. [MoonZoon Dev News 4: Actix, Async CLI, Error Handling, Wasm-pack Installer](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-wasm-pack-installer-57cp) - June 2021
7. [MoonZoon Dev News 5: Chat Example, MoonZoon Cloud](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) - July 2021
