CREATE TABLE IF NOT EXISTS post_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    post_type TEXT NOT NULL CHECK(post_type IN ('post', 'project')),
    viewer_ip TEXT,
    user_agent TEXT,
    viewed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_post_views_post_id ON post_views(post_id);
CREATE INDEX IF NOT EXISTS idx_post_views_post_type ON post_views(post_type);
CREATE INDEX IF NOT EXISTS idx_post_views_viewed_at ON post_views(viewed_at);
CREATE INDEX IF NOT EXISTS idx_post_views_ip_post ON post_views(viewer_ip, post_id, viewed_at);
