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
