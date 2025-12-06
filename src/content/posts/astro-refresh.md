---
title: "Rebuilding kavik.cz with Astro"
description: "Why I moved my personal site from a Zola theme to Astro's portfolio starter."
publishDate: 2025-12-01
tags: ["astro", "personal-site", "rewrite"]
img: "/assets/stock-3.jpg"
img_alt: "Laptop on a desk with design sketches"
---

My previous site ran on Zola with the Kita theme. It served me well, but I wanted something easier to extend with React/Vue islands, a richer design system, and a smoother content workflow for future posts.

Astro's portfolio starter gave me a modern layout and an approachable content collection API. I swapped in my projects, wired the Work section to Markdown files, and kept the focus on fast static output.

What changed for me:

- Content is now stored in `src/content`, so adding a project or blog post is a single Markdown file.
- The design stays consistent through shared components like `Hero`, `Grid`, and `CallToAction`.
- The build is still static, but I can sprinkle interactivity later without rewriting the stack.

If you spot anything broken or have ideas, let me know at martin@kavik.cz.
