use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCount {
    pub total_views: i64,
    pub unique_views: i64,
}

/// Record a view for a post/project by slug
pub async fn record_view(
    pool: &SqlitePool,
    post_slug: &str,
    post_type: &str,
    viewer_ip: Option<String>,
    user_agent: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO post_views (post_slug, post_type, viewer_ip, user_agent, viewed_at)
         VALUES (?, ?, ?, ?, datetime('now'))",
    )
    .bind(post_slug)
    .bind(post_type)
    .bind(viewer_ip)
    .bind(user_agent)
    .execute(pool)
    .await?;

    Ok(())
}

/// Check if a viewer IP has already viewed this post in the last 24 hours
pub async fn has_recent_view(
    pool: &SqlitePool,
    post_slug: &str,
    viewer_ip: &str,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count
         FROM post_views
         WHERE post_slug = ?
         AND viewer_ip = ?
         AND viewed_at > datetime('now', '-24 hours')",
    )
    .bind(post_slug)
    .bind(viewer_ip)
    .fetch_one(pool)
    .await?;

    let count: i64 = row.get("count");
    Ok(count > 0)
}

/// Get total and unique views for a post/project by slug
pub async fn get_view_count(pool: &SqlitePool, post_slug: &str) -> Result<ViewCount, sqlx::Error> {
    let row = sqlx::query(
        "SELECT 
            COUNT(*) as total_views,
            COUNT(DISTINCT viewer_ip) as unique_views
         FROM post_views
         WHERE post_slug = ?",
    )
    .bind(post_slug)
    .fetch_one(pool)
    .await?;

    Ok(ViewCount {
        total_views: row.get("total_views"),
        unique_views: row.get("unique_views"),
    })
}

/// Get top viewed posts/projects by post_type (slug-based)
pub async fn get_top_viewed(
    pool: &SqlitePool,
    post_type: &str,
    limit: i64,
) -> Result<Vec<(String, i64)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT 
            post_slug,
            COUNT(*) as view_count
         FROM post_views
         WHERE post_type = ?
         GROUP BY post_slug
         ORDER BY view_count DESC
         LIMIT ?",
    )
    .bind(post_type)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| (r.get("post_slug"), r.get("view_count")))
        .collect())
}
