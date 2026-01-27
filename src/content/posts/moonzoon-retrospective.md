---
title: "MoonZoon: Retrospective"
description: "What I learned building MoonZoon - signals over hooks, TEA vs components, and the road to Boon."
sortOrder: 3
tags: ["moonzoon", "rust", "wasm", "signals", "boon"]
---

![MoonZoon logo](/images/moonzoon-logo.png)

In early 2021, I started writing a series of technical articles documenting the development of [MoonZoon](https://github.com/MoonZoon/MoonZoon) - a Rust fullstack framework where frontend and backend share one language, one type system, and work together from the start.

This post summarizes the key technical insights from those articles.

## The Problem: Web Development Complexity

Modern web development has become unnecessarily complex. New developers must learn HTML, CSS, JavaScript, JSON, Git, DNS, HTTP, REST, WebSockets, SQL, and authentication - a daunting prerequisite list. Then comes the decision fatigue: React vs. Vue, MongoDB vs. PostgreSQL, serverless vs. VPS.

The deeper problem? As I wrote in [Cure for Web Development](https://dev.to/martinkavik/cure-for-web-development-nnn):

> "It works, until you try to do the first major refactor - you'll feel like you are going through a minefield."

HTML and CSS, designed for simple web pages decades ago, cannot adequately serve modern applications.

![Cure for Web Development](/images/cure_for_web_development.jpg)

## The Elm Architecture vs Components

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

*Note: This was an early API design influenced by my work on [Seed](https://github.com/seed-rs/seed), a Rust frontend framework I maintained before MoonZoon. The current Zoon API evolved differently, with fewer macros and a builder-pattern approach.*

This naturally separates TEA's trees into standalone blocks while providing local component state through `cmp_var` - without polluting global state.

## Rust Hooks > React Hooks

The most popular article in the series, [MoonZoon Dev News 3](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp), tackled a fundamental problem with React-style hooks.

React's hook system relies on call order to determine identity. The framework assigns unique IDs using a counter. Consider two render cycles where a conditional changes which hooks are called:

```
Render 1: mike() -> id=1, layla_rose() -> id=2
Render 2: mike() -> id=1, amber()      -> id=2  // wrong!
```

When conditional logic changes which functions execute, call IDs become unstable. State incorrectly associates with different components across renders. This is why React requires the "rules of hooks" - no hooks in conditionals.

**The Rust solution**: Rather than relying on indices alone, MoonZoon leveraged Rust's `#[track_caller]` attribute combined with `Location::caller()`. This identifies hook calls by their source code location:

```
Key = (call_index, file_location)
```

This approach proved more robust than React's index-based system, preventing data mismatches even with conditional rendering.

Eventually, MoonZoon moved beyond hooks entirely to a **signals-based reactive system**. Signals provide reactive data binding without requiring a Virtual DOM, eliminating the need for call counters, location tracking overhead, and rules-of-hooks compliance checks.

<aside class="note">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="24" height="24" fill="currentColor"><path d="M176,232a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h80A8,8,0,0,1,176,232Zm40-128a87.55,87.55,0,0,1-33.64,69.21A16.24,16.24,0,0,0,176,186v6a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16v-6a16,16,0,0,0-6.23-12.66A87.59,87.59,0,0,1,40,104.49C39.74,56.83,78.26,17.14,125.88,16A88,88,0,0,1,216,104Zm-16,0a72,72,0,0,0-73.74-72c-39,.92-70.47,33.39-70.26,72.39a71.65,71.65,0,0,0,27.64,56.3A32,32,0,0,1,96,186v6h64v-6a32.15,32.15,0,0,1,12.47-25.35A71.65,71.65,0,0,0,200,104Zm-16.11-9.34a57.6,57.6,0,0,0-46.56-46.55,8,8,0,0,0-2.66,15.78c16.57,2.79,30.63,16.85,33.44,33.45A8,8,0,0,0,176,104a9,9,0,0,0,1.35-.11A8,8,0,0,0,183.89,94.66Z"/></svg>

This identity problem extends beyond hooks. List rendering in frameworks like React still requires explicit `key` props to track elements across renders - without them, reordering breaks and performance suffers. Elm and Seed had the same issue. MoonZoon's signal vectors (`MutableVec`) solved both problems by transmitting only differences at the data level, making keys unnecessary.

</aside>

The performance difference was dramatic. Using the [Dominator](https://github.com/Pauan/rust-dominator) library under the hood, Zoon could update thousands of rows basically immediately, while other Rust frameworks needed to re-render entire pages.

## Fullstack Architecture

The architecture looked like this:

### Three-Tier Structure

MoonZoon applications are organized into three crates:
- **Shared** - Serializable types for frontend-backend communication (using `serde_lite` to reduce Wasm file size)
- **Moon** (backend) - Actix-based server with virtual actors, inspired by Orleans, Phoenix, and Meteor
- **Zoon** (frontend) - Reactive UI with signals, drawing from elm-ui, Dominator, and Svelte

![Chat example](/images/moonzoon_send_message.png)

### Live-Reload via SSE

As detailed in [MoonZoon Dev News 1](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6), the dev server uses Server-Sent Events (not WebSockets) for live-reload. The frontend receives two event types:
- **Reload Events** - Trigger page refresh after file changes
- **Backend Build ID Events** - Auto-reload when backend restarts

HTTPS runs during development to mirror production, with HTTP/2 eliminating SSE limitations.

### Warp to Actix Migration

[MoonZoon Dev News 4](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-wasm-pack-installer-57cp) documented the migration from Warp to Actix. I wanted speed, async capabilities, and better Tokio integration. A key simplification: the `main` function became async, eliminating the need for separate `init` functions.

### Cleaner Error Handling

The mzoon CLI adopted a two-library approach:
- **anyhow** - Enables early returns via `?` without manual error mapping
- **fehler** - Eliminates verbose `Ok(())` wrapping through the `#[throws]` macro

### Virtual Actors for Sessions

[MoonZoon Dev News 5](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) introduced virtual actors: "sessions are virtual actors managed by the Moon. Each SessionActor represents a live connection between the frontend (Zoon) and backend (Moon)." This enables broadcasting to all clients or targeting individual sessions via correlation IDs.

## Hard-Won Lessons

Building a fullstack Rust/WASM framework surfaced challenges that aren't obvious until you hit them.

### WASM File Size: Death by Dependencies

One of the most surprising findings was how dramatically dependencies affect WASM bundle size. Using a simple counter example:

| Configuration | Optimized | Brotli |
|:------------------------------|----------:|-------:|
| Base app | 33 KB | 14 KB |
| + `1.2.to_string()` | 52 KB | 21 KB |
| + `url` crate | 338 KB | 113 KB |
| + `url` + `regex` crates | 928 KB | 236 KB |

A single float formatting call adds ~19 KB. The `regex` crate alone adds ~590 KB due to Unicode tables. These numbers shaped MoonZoon's dependency choices:

- **serde_lite** instead of full serde
- **js_sys::RegExp** instead of the `regex` crate
- **web_sys::Url** instead of the `url` crate
- **ufmt** as an alternative to `std::fmt`

<aside class="note">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="24" height="24" fill="currentColor"><path d="M176,232a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h80A8,8,0,0,1,176,232Zm40-128a87.55,87.55,0,0,1-33.64,69.21A16.24,16.24,0,0,0,176,186v6a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16v-6a16,16,0,0,0-6.23-12.66A87.59,87.59,0,0,1,40,104.49C39.74,56.83,78.26,17.14,125.88,16A88,88,0,0,1,216,104Zm-16,0a72,72,0,0,0-73.74-72c-39,.92-70.47,33.39-70.26,72.39a71.65,71.65,0,0,0,27.64,56.3A32,32,0,0,1,96,186v6h64v-6a32.15,32.15,0,0,1,12.47-25.35A71.65,71.65,0,0,0,200,104Zm-16.11-9.34a57.6,57.6,0,0,0-46.56-46.55,8,8,0,0,0-2.66,15.78c16.57,2.79,30.63,16.85,33.44,33.45A8,8,0,0,0,176,104a9,9,0,0,0,1.35-.11A8,8,0,0,0,183.89,94.66Z"/></svg>

These alternatives were hidden behind Cargo features, but they're no longer used. In practice, `serde` is everywhere - Rust's orphan rule (you can't implement foreign traits for foreign types) makes using alternatives nearly impossible when your dependencies expect serde. Similarly, `ufmt` wasn't a long-term solution because any regular float formatting in dependencies pulls in the standard formatting library anyway.

</aside>

The Cargo.toml optimization settings that worked best:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3  # or 's' for size

[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-O4']  # or '-Os' for size
```

<aside class="note">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="24" height="24" fill="currentColor"><path d="M176,232a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h80A8,8,0,0,1,176,232Zm40-128a87.55,87.55,0,0,1-33.64,69.21A16.24,16.24,0,0,0,176,186v6a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16v-6a16,16,0,0,0-6.23-12.66A87.59,87.59,0,0,1,40,104.49C39.74,56.83,78.26,17.14,125.88,16A88,88,0,0,1,216,104Zm-16,0a72,72,0,0,0-73.74-72c-39,.92-70.47,33.39-70.26,72.39a71.65,71.65,0,0,0,27.64,56.3A32,32,0,0,1,96,186v6h64v-6a32.15,32.15,0,0,1,12.47-25.35A71.65,71.65,0,0,0,200,104Zm-16.11-9.34a57.6,57.6,0,0,0-46.56-46.55,8,8,0,0,0-2.66,15.78c16.57,2.79,30.63,16.85,33.44,33.45A8,8,0,0,0,176,104a9,9,0,0,0,1.35-.11A8,8,0,0,0,183.89,94.66Z"/></svg>

**The generics paradox**: Analysis with [Twiggy](https://github.com/nicholasjng/nicholasjng.github.io) showed that most code bloat from generics came from `futures-signals`. The intuitive fix - replacing generics with other constructs - made apps both slower AND bigger. Sometimes the compiler knows best.

</aside>

### Compression Pipeline Gotchas

MoonZoon's CLI generates both Gzip and Brotli variants of all assets. This uncovered several issues:

**The async-compression bug**: During testing, the `async-compression` crate produced 9 KB output instead of the expected 16 KB. The fix was ensuring `.flush()` is always called after `.write_all()` - a subtle bug that would have shipped broken WASM files.

**Browser quirks**: Firefox only supports Brotli over HTTPS, while Chrome accepts both Gzip and Brotli on HTTP. This meant serving both compression formats and selecting based on `Accept-Encoding`.

**HTTP/2 necessity**: Browsers limit HTTP/1.x to 6 concurrent connections per domain. Since SSE holds a persistent connection, you quickly run out. HTTP/2 multiplexes everything over a single connection - but browsers only support HTTP/2 over HTTPS. This meant running HTTPS even in development.

### Build Tooling Challenges

The mzoon CLI faced several unexpected hurdles:

**File watcher debouncing**: The `notify` crate (5.0.0-pre) lacked built-in debouncing. A custom implementation using task abort-and-reset patterns was needed - new file changes cancel pending rebuilds and restart the timer.

**Directory traversal without recursion**: Rust lacks tail-call optimization, so recursive directory walking risks stack overflow on deep trees. The solution used `futures::stream::unfold()` for iterative, non-blocking traversal.

**Macro conflicts**: Combining `async-trait` with `fehler`'s `#[throws]` macro proved "deadly for the compiler" - requiring explicit `Result<()>` annotations to resolve return-type ambiguities.

**wasm-pack auto-install**: The mzoon CLI auto-downloads wasm-pack from GitHub releases. Strict platform matching (e.g., requiring `linux-gnu` exactly) broke on environments like Heroku. The fix was relaxed matching - any Linux can run the `musl` binary.

<aside class="note">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="24" height="24" fill="currentColor"><path d="M176,232a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h80A8,8,0,0,1,176,232Zm40-128a87.55,87.55,0,0,1-33.64,69.21A16.24,16.24,0,0,0,176,186v6a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16v-6a16,16,0,0,0-6.23-12.66A87.59,87.59,0,0,1,40,104.49C39.74,56.83,78.26,17.14,125.88,16A88,88,0,0,1,216,104Zm-16,0a72,72,0,0,0-73.74-72c-39,.92-70.47,33.39-70.26,72.39a71.65,71.65,0,0,0,27.64,56.3A32,32,0,0,1,96,186v6h64v-6a32.15,32.15,0,0,1,12.47-25.35A71.65,71.65,0,0,0,200,104Zm-16.11-9.34a57.6,57.6,0,0,0-46.56-46.55,8,8,0,0,0-2.66,15.78c16.57,2.79,30.63,16.85,33.44,33.45A8,8,0,0,0,176,104a9,9,0,0,0,1.35-.11A8,8,0,0,0,183.89,94.66Z"/></svg>

Later, wasm-pack was replaced with standalone `wasm-bindgen-cli` for more flexibility - we didn't need wasm-pack features like generating npm packages.

</aside>

### SSE Reliability

Server-Sent Events for live-reload seemed simple but had edge cases:

- **10-second ping interval** to detect stale connections
- **Firefox permanently closes SSE** after backend restart - requiring the `ReconnectingEventSource` library for automatic reconnection
- **Certificate serial numbers** must be unique per generation, or Firefox throws `SEC_ERROR_REUSED_ISSUER_AND_SERIAL`

## From MoonZoon to Boon

![Boon logo](/images/boon_logo.png)

Many ideas from MoonZoon carried forward into [Boon](/#boon). The reactive signals architecture evolved into Boon's dataflow model. The frustration with HTML/CSS abstraction led to exploring new rendering approaches.

The WASM size optimization pain points should be far less painful in Boon. Problems like serde alternatives and formatting libraries become concerns for the Rust runtime and Boon extension maintainers - not Boon users. The burden shifts from every app developer fighting dependency bloat to a small team optimizing the platform once.

The core insight remained: web development is too complex because we're working at the wrong abstraction level. Both MoonZoon and Boon attempt to find that better abstraction - one that makes building web applications feel as natural as describing what you want to build.

Learn more at [boon.run](https://boon.run/). For the ideas that shaped it, read [The Boon of Dataflow](/blog/the-boon-of-dataflow).

---

*These articles were originally published on [dev.to](https://dev.to/martinkavik) between February and July 2021:*

1. [Cure for Web Development](https://dev.to/martinkavik/cure-for-web-development-nnn) - February 2021
2. [MoonZoon Dev News 1: CLI, Build Pipeline, Live-Reload, HTTPS](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6) - March 2021
3. [MoonZoon Dev News 2: Live Demo, Zoon, Examples, Architectures](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem) - April 2021
4. [MoonZoon Dev News 3: Signals, React-like Hooks, Optimizations](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp) - May 2021
5. [MoonZoon Dev News 4: Actix, Async CLI, Error Handling, Wasm-pack Installer](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-wasm-pack-installer-57cp) - June 2021
6. [MoonZoon Dev News 5: Chat Example, MoonZoon Cloud](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) - July 2021

*Press & interviews:*

- [Console 114: Interview](https://console.substack.com/p/console-114) - July 2022
- [MoonZoon Overview](https://blog.abor.dev/p/moonzoon) - Abor Dev
- [Full-Stack Rust Framework](https://devm.io/rust/rust-framework-moonzoon) - DevMio
