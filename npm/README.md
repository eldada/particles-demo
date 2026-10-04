# Ember Field

A browser page with a small fire of particles inside a slowly rotating wireframe cube.

## Requirements

- Node.js 22 or newer
- npm

## Run while developing

```bash
npm install
npm run dev
```

Open http://127.0.0.1:5173/. The page rebuilds when the source changes.

## Build a static page

```bash
npm install
npm run build
```

The site is written to `dist/`. Serve that folder with any static file server:

```bash
python3 -m http.server -d dist 5173
```

Open http://127.0.0.1:5173/. The same `dist/` folder can be hosted on GitHub Pages, Netlify, or another plain web server.

Open the built page through a server. The HTML requests `/style.css` and `/assets/main.js` from the site root, so a `file://` URL will not load them.

## Controls

The same keys are listed in the panel at the top left.

| Key | Action |
| --- | --- |
| Arrow keys | Extend the burn in that direction. Repeat a key to push it farther |
| `+` / `-` | Raise or lower the particle count |
| `F` | Firework burst inside the cube. It fades out, then returns to the fire |
| `1`–`9` | Set the wire cube size, from smallest to largest |
| `[` / `]` | Slow down or speed up the cube rotation, including reverse |
| `A` / `Z` | Zoom in or out |

On a touch screen, the Firework button triggers that burst.
