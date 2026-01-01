CREATE TABLE post_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_slug TEXT NOT NULL,
    post_type TEXT NOT NULL,
    viewer_ip TEXT,
    user_agent TEXT,
    viewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
