# Docker Deployment

This repo builds two separate containers:

- `backend`: Rust/Axum API, content loader, asset server, and SQLite analytics.
- `frontend`: static SvelteKit build served by nginx, with `/api/*` and `/assets/*` proxied to the backend container.

GitHub Actions publishes both images to GitHub Container Registry on pushes to `main` or `master`:

- `ghcr.io/smit4k/smit.codes-backend:latest`
- `ghcr.io/smit4k/smit.codes-frontend:latest`

## Local Build

```sh
docker compose up --build
```

The site will be available at `http://localhost:8080`.

## Ubuntu Server

Install Docker and the Compose plugin, then log in to GHCR if the packages are private:

```sh
echo "$GITHUB_TOKEN" | docker login ghcr.io -u smit4k --password-stdin
```

Create a `.env` file next to `docker-compose.prod.yml`:

```env
BACKEND_IMAGE=ghcr.io/smit4k/smit.codes-backend:latest
FRONTEND_IMAGE=ghcr.io/smit4k/smit.codes-frontend:latest
FRONTEND_PORT=8080
```

Start or update the containers:

```sh
docker compose -f docker-compose.prod.yml pull
docker compose -f docker-compose.prod.yml up -d
```

Point nginx/Caddy on the host at `http://127.0.0.1:8080`, or publish port 80 directly by setting `FRONTEND_PORT=80`.

Analytics data persists in the `backend-db` Docker volume. Content is baked into the backend image, so deploy new images after content changes.
