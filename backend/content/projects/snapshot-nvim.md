---
title: snapshot.nvim
date: 2026-05-03
tags: [neovim, lua, rust]
links: [https://github.com/smit4k/snapshot.nvim]
description: "A Neovim plugin for taking code screenshots from the current buffer or visual selection"
---


![snapshot.nvim example](https://raw.githubusercontent.com/smit4k/snapshot.nvim/main/examples/snapshot-example.png)

I made this plugin because I liked the plugin [Polacode](https://github.com/octref/polacode) plugin from Visual Studio
Code, It allows you to create a nice aesthetic screenshot of your code with
syntax highlighting. and I noticed there werent any native plugins recreating
the functionality of Polacode inside of Neovim, so I made my own.
[snapshot.nvim](https://github.com/smit4k/snapshot.nvim) allows you to create beautiful code screenshots directly inside
of Neovim! It captures either the current visual selection or the whole buffer,
keeps the Tree-sitter syntax colors, and renders the final image with a small
Rust binary.


## Usage

Install it with your Neovim plugin manager. With `lazy.nvim`, the setup looks
like this:

```lua
return {
  "smit4k/snapshot.nvim",
  config = function()
    require("snapshot").setup({
      snapshot_dir = "~/Pictures/snapshots",
      clipboard = true,
      shadow = true,
      line_numbers = false,
    })
  end,
}
```

To capture part of a file, select the code in Visual mode and run `:Snapshot`.
To capture the whole buffer, run `:Snapshot` in Normal mode.

## How it works

The plugin is split into two parts. The Lua side talks to Neovim. The Rust side
draws the image.

That split keeps the editor integration simple while still letting the renderer
use normal image libraries. Lua is good at asking Neovim what is on screen. Rust
is better for measuring text, drawing pixels, adding shadows, rounding corners,
saving files, and copying an image to the clipboard.

### Capturing the code

The `:Snapshot` command checks the current visual marks first. If they point to
a real selection, the plugin reads only those lines from the buffer. If there is
no visual selection, it reads the whole buffer.

Character-wise selections are trimmed on the first and last line, so selecting
part of a line does what you expect instead of silently capturing the entire
line.

### Preserving colors

The plugin asks Tree-sitter for captures at each column in each captured line.
It resolves those captures through Neovim highlight groups, then merges
neighboring columns into spans before sending them to the renderer.

The payload ends up looking roughly like this:

```json
{
  "lines": [
    {
      "text": "local value = 42",
      "spans": [
        { "start": 0, "end": 5, "fg": "#c678dd" },
        { "start": 6, "end": 11, "fg": "#e06c75" }
      ]
    }
  ],
  "config": {
    "padding": 25,
    "font_size": 24,
    "shadow": true
  }
}
```

There is no screenshot scraping involved. The plugin sends text, color spans,
and configuration to the generator over stdin as JSON. This ensures the output
image is pure and clean text.

### Rendering the image

The Rust generator reads that JSON, loads JetBrains Mono from the plugin's
release directory, measures each line, and draws the code card with `imageproc`.
It applies the configured padding, line height, render scale, line numbers,
background colors, rounded corners, and outer shadow.

By default, `snapshot.nvim` uses the current `Normal` highlight group for the
card background and foreground. That makes the snapshot match the active
colorscheme unless you override the colors yourself.

When rendering finishes, the generator saves the file to `snapshot_dir` or an
explicit `output_path`. If clipboard support is enabled, it also tries to copy
the image. Clipboard failures are warnings, not fatal errors, because saving the
image still matters more.

## Binary setup

The Rust renderer lives under `generator/`. On setup, the plugin checks for a
matching release binary in `generator/target/release/`. If the binary is missing
or the stored version does not match the plugin version, it downloads the right
release artifact for the current platform.

If the download fails, it falls back to `cargo build --release`. That makes
local development less annoying and gives users a way out when a prebuilt binary
is not available.

## Things I learned

When I first started this project, I found it extremely annoying to get the
syntax highlighting from Neovim directly. It was very difficult to get clean
data directly from the Neovim API to create a screenshot.

While Tree-sitter gives us the colors and syntax information, we can't use

Another challenge I ran into was because of the fact I was using two different
languages, it made releases more difficult. While a pure Lua plugin would have
been simpler, image rendering and clipboard support would have been much more
annoying and wouldn't perform as well. For this project, using a sidecar design with Rust alongside Lua felt worth it. Using a sidecar design meant I needed a
robust way to ship the Rust renderer. While I could have the user build it
themselves, not every user has the Rust toolchain installed or wants to compile
the Rust binary just to take a fancy screenshot. Thankfully Github releases made
shipping the renderer binary manageable: I can push a tag to Github which
triggers a build run and the Neovim plugin can download the artifact from the
build for the proper architecture when it needs to.
