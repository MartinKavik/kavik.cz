# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Version Control

This repo uses **jj** (Jujutsu), not Git. Use `jj` commands for all version control operations.

## Commands

- `npm run dev` — start the local dev server on `http://localhost:4321`
- `npm run build` — create a production build in `dist/`
- `npm run preview` — preview the production build locally

## Architecture

This is an Astro static site for Martin Kavík's portfolio at kavik.cz.

### Content Collections (Astro v5)

Two content collections are defined in `src/content.config.ts`:

- **work** (`src/content/work/*.md`) — Portfolio entries with fields: title, description, publishDate, tags, img, img_alt, optional links array
- **posts** (`src/content/posts/*.md`) — Blog posts with fields: title, description, publishDate, tags, optional img/img_alt

### Routing

- Static pages: `src/pages/index.astro`, `about.astro`, `work.astro`, `blog.astro`, `404.astro`
- Dynamic routes: `src/pages/work/[...slug].astro` and `src/pages/blog/[...slug].astro` for content entries

### Layout

All pages use `src/layouts/BaseLayout.astro` which includes:
- `MainHead.astro` — meta tags, fonts, global styles
- `Nav.astro` — navigation with theme toggle
- `Footer.astro`

### Styling

- Global CSS in `src/styles/global.css`
- Component-scoped styles via `<style>` tags in `.astro` files
- Dark/light theme via `.theme-dark` class on root element
- Background images served from `public/assets/backgrounds/`

### Public Assets

- `public/assets/` — background images and SVGs
- `public/images/` — project images and portraits

## Browser Automation Tools

The `tools/` directory contains Rust-based browser automation for visual debugging. Start these in order:

```bash
# Terminal 1: WebSocket server
cd tools && cargo run --release -- server start --port 9223

# Terminal 2: Launch Chromium with extension
cd tools && cargo run --release -- browser launch --ws-port 9223
```

### CLI Commands

```bash
cargo run --release -- exec --port 9223 screenshot -o /tmp/screenshot.png
cargo run --release -- exec --port 9223 resize --preset mobile   # 375x667
cargo run --release -- exec --port 9223 resize --preset tablet   # 768x1024
cargo run --release -- exec --port 9223 resize --preset desktop  # 1024x768
cargo run --release -- exec --port 9223 resize --preset large    # 1440x900
cargo run --release -- exec --port 9223 resize --width 800 --height 600
cargo run --release -- exec --port 9223 navigate --url "http://localhost:4321/about"
cargo run --release -- exec --port 9223 status
```

Port 9223 is used because 9222 is reserved for boon extension.
