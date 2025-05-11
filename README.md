# 📰 Rustfolio — A Newspaper-Style Portfolio with Yew

**Rustfolio** is an experimental, retro-futuristic personal portfolio built using [Yew.rs](https://yew.rs/) and WebAssembly. Styled like a newspaper and powered by Rust, this site showcases projects, experimental animations, and developer logs—all wrapped in a vintage editorial layout.

---

## 🖼️ Concept

> _"Blending the aesthetic of print journalism with the interactivity of modern web apps."_

Each section looks like a newspaper article. Pages animate into view with cinematic motion using `requestAnimationFrame` and native DOM manipulation—all in pure Rust.

---

## 🛠 Tech Stack

- 🦀 **Rust + [Yew](https://yew.rs/)** — Single Page Application frontend framework in Rust
- 🧠 **WASM + [`web-sys`](https://docs.rs/web-sys)** — For low-level browser API access and DOM animation
- 🔁 **[`requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)** — Used for buttery-smooth, custom transitions
- ✨ **No JavaScript frameworks** — 100% Rust

---

## ✨ Highlights

- 🌀 **Animated Screen Transitions**  
  Custom pivot-based entrance animation using scale, translate, and rotation. Scroll is locked during the motion to preserve cinematic feel.

- 📰 **Newspaper Aesthetic**  
  Typographic layout mimicking columns, serif headlines, and editorial spacing using minimalist styles.

- 🔬 **Experimental Projects Section**  
  Showcases cutting-edge side projects like AI tools, art generators, and Rust bots alongside developer commentary in "lab log" format.

---

## 📁 Project Structure

```bash
src/
├── layouts/
│   └── screen.rs        # Animated intro component
├── components/          # Article blocks, sidebars, cards
├── utils/               # Portfolio sections (About, Projects, Blog)
├── main.rs              # Yew app entrypoint
```

## 🚀 Running Locally
First, install Trunk (Rust-based WASM bundler):

```bash
cargo install trunk
```

Then run the app:

```bash
trunk serve
```

Open your browser at http://localhost:8080


## 🔗 Live & Source
 - 🌐 Live: https://jyo561.pages.dev/


## 🙌 Credits
 - Inspired by retro newspapers,and UI Design implemented by [@rinkitadhana](https://github.com/rinkitadhana/The-Daily-Crimes).

 - Built with ❤️ and 🦀 in Rust.


