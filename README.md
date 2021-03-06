# Personal website [kavik.cz](https://kavik.cz/)

[![Build Status](https://travis-ci.org/MartinKavik/kavik.cz.svg?branch=master)](https://travis-ci.org/MartinKavik/kavik.cz)
[![Netlify Status](https://api.netlify.com/api/v1/badges/19540504-c4ba-49cf-842d-62ee48bf437c/deploy-status)](https://app.netlify.com/sites/kavik/deploys)

- Written in **Rust** framework [Seed](https://github.com/David-OConnor/seed).
- Prerendered. (Prerendering is a part of build pipeline, see `build:prerender` in `package.json`),
- Based on [MartinKavik/seed-quickstart-webpack](https://github.com/MartinKavik/seed-quickstart-webpack).

<p align="center">
  <img src="/design/web/logo.svg" width="128" title="Martin Kavík logo">
</p>

## Fork it, modify it and use it as your own website!

---

## Quickstart

1. Make sure you have [Rust](https://www.rust-lang.org), [Yarn](https://yarnpkg.com) and [Node.js](https://nodejs.org) installed.
    - `rustc -V` should output something like `rustc 1.38.0 (625451e37 2019-09-23)`
    - `yarn -v` should output something like `1.17.3`
    - `node -v` should output something like `v10.16.3`
1. Fork it / Clone it / Download it.
1. `yarn`
1. `yarn start`
1. [localhost:8000](http://localhost:8000)

---

## Roadmap

- Experiment with lightweight custom layout engine to make code simpler and more readable. (It will be a part of the [Hellweb project](https://github.com/MartinKavik/hellweb-pain)).
- Write simple cookieless analytics. (Probably a part of the [Hellweb project](https://github.com/MartinKavik/hellweb-pain), too).

---

Other Seed projects: [MartinKavik/awesome-seed-rs](https://github.com/MartinKavik/awesome-seed-rs)
