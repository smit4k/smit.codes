# Backend Performance Audit (Axum + SQLite)

Date: 2026-05-11

## Scope
- Reviewed routing/middleware setup in `src/main.rs`.
- Reviewed analytics DB pool and SQLite configuration in `src/analytics/db.rs`.
- Reviewed analytics query patterns in `src/analytics/views.rs` and handler logic in `src/analytics/handlers.rs`.
- Reviewed cache/ETag behavior in `src/utils/cache.rs`.

## High-level assessment
The backend is architected for fast read-heavy workloads:
- Content is loaded once at startup, serialized, and cached in-memory by list and by slug.
- GET endpoints serve prebuilt `Bytes` payloads and conditional ETag responses.
- SQLite uses WAL + `NORMAL` synchronous mode and a connection pool.

This design is consistent with the observed latency profile (single-digit ms locally and tens of ms cross-region).

## Strengths
1. **Hot-path read efficiency (excellent)**
   - Content endpoints do not hit disk or DB per request; they return precomputed payloads.
   - Slug lookup is O(1)-style hashmap lookup.

2. **HTTP caching semantics (above average to excellent)**
   - Strong ETag generation and 304 support for unchanged resources.
   - Cache-Control requires revalidation, reducing payload transfer while keeping freshness.

3. **SQLite runtime tuning (above average)**
   - WAL mode and busy timeout improve concurrent behavior.
   - Pool sizing is configurable via env vars.

4. **Safety controls (above average)**
   - Endpoint-specific rate limiting is present and configurable.
   - Input normalization/sanitization exists for analytics paths and user-agent handling.

## Bottlenecks / Risks
1. **Linear scans in view-recording validation**
   - `record_post_view` checks slug existence via `iter().find(...)` over vectors; this is O(n).
   - Probably fine at current scale, but avoidable with a precomputed slug set/map.

2. **Analytics query growth over time**
   - `COUNT(*)` and grouped queries over ever-growing `page_views`/`post_views` can become expensive.
   - Existing composite indexes help inserts/dedup checks, but aggregate queries may still degrade without rollups/partitions.

3. **Single-region distance dominates remote p50**
   - DET -> NYC network RTT likely explains much of 35–40ms, not application compute.

4. **Potentially over-strict body limit globally**
   - `DefaultBodyLimit::max(1024)` is very low and applied globally; good for current tiny POST payloads, but could become a functional/perf issue if payloads expand.

## Ratings
- **Content read path performance:** **Excellent**
- **Analytics backend scalability (current design):** **Above Average**
- **Overall backend performance quality:** **Above Average**

## Why the measured latency is plausible
- Localhost `~<8ms` aligns with in-memory JSON + minimal handler work.
- Remote `35–40ms` from Detroit to NYC is consistent with added network RTT, TLS, and internet variability.

## Recommended improvements (priority order)
1. Precompute slug lookup sets/maps for `writing/projects/photos` in analytics-write validation path.
2. Add periodic rollups/materialized counters for `site stats` and per-page totals.
3. Add lightweight server-timing metrics per handler to split app time vs network time.
4. Consider edge caching/CDN for read endpoints if traffic grows geographically.
5. Add DB maintenance/retention strategy (e.g., archive old raw views) to cap table growth.
