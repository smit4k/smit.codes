FROM oven/bun:1 AS builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend ./
COPY backend/content ../backend/content

ENV NODE_ENV=production
RUN bun run build

FROM nginx:1.27-alpine AS runtime

COPY frontend/nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=builder /app/frontend/build /usr/share/nginx/html

ENV BACKEND_UPSTREAM=http://backend:3001

EXPOSE 80
