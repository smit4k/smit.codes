CREATE TABLE IF NOT EXISTS post_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_slug TEXT NOT NULL,
    post_type TEXT NOT NULL,
    viewer_ip TEXT,
    user_agent TEXT,
    viewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS page_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    page_path TEXT NOT NULL,
    viewer_ip TEXT,
    user_agent TEXT,
    viewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS site_counters (
    counter_name TEXT PRIMARY KEY,
    count INTEGER NOT NULL DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO site_counters (counter_name, count)
VALUES ('total_page_visits', 0);

CREATE INDEX IF NOT EXISTS idx_post_views_lookup
ON post_views (post_slug, post_type, viewer_ip, user_agent, viewed_at);

CREATE INDEX IF NOT EXISTS idx_page_views_lookup
ON page_views (page_path, viewer_ip, user_agent, viewed_at);
