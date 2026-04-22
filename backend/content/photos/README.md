# Photo Posts

Create one folder per photo post under `backend/content/photos`.

Example:

```text
backend/content/photos/
  tokyo-street-walk/
    post.yaml
    cover.webp
    preview-1.webp
    preview-2.webp
    full-1.webp
    full-2.webp
```

Example manifest:

```yaml
title: Tokyo Street Walk
date: 2026-04-21
tags:
  - travel
  - street
coverImage: cover.webp
previewImages:
  - preview-1.webp
  - preview-2.webp
images:
  - src: full-1.webp
    width: 1600
    height: 1200
    alt: Neon signs after rain
  - src: full-2.webp
    width: 1200
    height: 1600
    alt: Alleyway vending machine glow
description: Night photos from a street walk through Shinjuku.
```

Image paths are resolved relative to the manifest file and served from `/assets/...`.
