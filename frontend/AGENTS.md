# Repository Guidelines

## Project Structure & Module Organization

This is a SvelteKit frontend for `smit.codes`. Route pages and endpoint handlers live in
`src/routes` using `+page.svelte`, `+page.ts`, and `+server.ts`. Shared utilities, types, SEO
helpers, markdown rendering, and reusable components live in `src/lib`. Static assets such as fonts,
icons, and `robots.txt` belong in `static`. Build output is generated in `build` and should not be
edited by hand.

## Build, Test, and Development Commands

Use Bun for package scripts:

- `bun run dev`: start the local Vite development server.
- `bun run build`: create a production build.
- `bun run preview`: serve the production build locally for inspection.
- `bun run check`: run SvelteKit sync and TypeScript/Svelte diagnostics.
- `bun run check:watch`: run diagnostics continuously while developing.
- `bun run lint`: check formatting with Prettier.
- `bun run format`: format the repository with Prettier.
- `bun run build:all`: build the parent `lambdavim` artifact, then this frontend.

## Coding Style & Naming Conventions

Prettier is the source of truth: tabs, single quotes, no trailing commas, and a 100-character print
width, with `prettier-plugin-svelte` for `.svelte` files. Keep route directories lowercase for public
URL paths, except existing compatibility routes such as `WritingPost`, `PhotoPost`, and
`ProjectPost`. Name reusable Svelte components in PascalCase under
`src/lib/components`, and keep utility modules in lowercase TypeScript files such as `date.ts` or
`markdown.ts`.

## Testing Guidelines

There is no dedicated unit or browser test runner configured yet. Treat `bun run check`,
`bun run lint`, and `bun run build` as required validation before submitting changes. When adding
tests, colocate focused tests near the code they cover or add a clear `tests` directory, and document
the new command in `package.json`.

## Commit & Pull Request Guidelines

Recent history uses short, imperative subjects, sometimes with Conventional Commit prefixes:
`feat: add writing heading permalinks`, `fix(content): resolve case-insensitive markdown assets`, and
`Add local image support to backend content loader`. Prefer `type(scope): summary` when a clear type
applies, and keep summaries specific.

Pull requests should describe the user-facing change, list validation commands, link related issues,
and include screenshots for visual changes. Call out content, routing, SEO, or deployment changes
because they affect generated pages and feeds.

## Security & Configuration Tips

Do not commit secrets or machine-specific configuration. Keep public, cacheable assets in `static`,
and update SEO helpers in `src/lib/seo.ts` or site metadata in `src/lib/site.ts` rather than
duplicating constants across routes.
