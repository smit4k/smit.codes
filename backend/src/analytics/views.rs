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

// --- Page view tracking (all pages) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct PageViewCount {
    pub page_path: String,
    pub total_views: i64,
    pub unique_views: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteStats {
    pub total_views: i64,
    pub unique_visitors: i64,
    pub pages: Vec<PageViewCount>,
}

/// Record a page view for any page path
pub async fn record_page_view(
    pool: &SqlitePool,
    page_path: &str,
    viewer_ip: Option<String>,
    user_agent: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO page_views (page_path, viewer_ip, user_agent, viewed_at)
         VALUES (?, ?, ?, datetime('now'))",
    )
    .bind(page_path)
    .bind(viewer_ip)
    .bind(user_agent)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get total and unique views for a specific page path
pub async fn get_page_view_count(
    pool: &SqlitePool,
    page_path: &str,
) -> Result<PageViewCount, sqlx::Error> {
    let row = sqlx::query(
        "SELECT
            COUNT(*) as total_views,
            COUNT(DISTINCT viewer_ip) as unique_views
         FROM page_views
         WHERE page_path = ?",
    )
    .bind(page_path)
    .fetch_one(pool)
    .await?;

    Ok(PageViewCount {
        page_path: page_path.to_string(),
        total_views: row.get("total_views"),
        unique_views: row.get("unique_views"),
    })
}

/// Get site-wide stats: total views, unique visitors, and per-page breakdown
pub async fn get_site_stats(pool: &SqlitePool) -> Result<SiteStats, sqlx::Error> {
    let totals = sqlx::query(
        "SELECT
            COUNT(*) as total_views,
            COUNT(DISTINCT viewer_ip) as unique_visitors
         FROM page_views",
    )
    .fetch_one(pool)
    .await?;

    let pages = sqlx::query(
        "SELECT
            page_path,
            COUNT(*) as total_views,
            COUNT(DISTINCT viewer_ip) as unique_views
         FROM page_views
         GROUP BY page_path
         ORDER BY total_views DESC",
    )
    .fetch_all(pool)
    .await?;

    Ok(SiteStats {
        total_views: totals.get("total_views"),
        unique_visitors: totals.get("unique_visitors"),
        pages: pages
            .into_iter()
            .map(|r| PageViewCount {
                page_path: r.get("page_path"),
                total_views: r.get("total_views"),
                unique_views: r.get("unique_views"),
            })
            .collect(),
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
