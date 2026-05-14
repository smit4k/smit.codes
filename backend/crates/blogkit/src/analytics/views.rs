use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

const TOTAL_PAGE_VISITS_COUNTER: &str = "total_page_visits";

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
         SELECT ?, ?, ?, ?, datetime('now')
         WHERE NOT EXISTS (
             SELECT 1
             FROM post_views
             WHERE post_slug = ?
               AND post_type = ?
               AND viewer_ip IS ?
               AND user_agent IS ?
               AND viewed_at >= datetime('now', '-1 hour')
         )",
    )
    .bind(post_slug)
    .bind(post_type)
    .bind(viewer_ip.as_deref())
    .bind(user_agent.as_deref())
    .bind(post_slug)
    .bind(post_type)
    .bind(viewer_ip.as_deref())
    .bind(user_agent.as_deref())
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
    increment_total_page_visits(pool).await?;

    sqlx::query(
        "INSERT INTO page_views (page_path, viewer_ip, user_agent, viewed_at)
         SELECT ?, ?, ?, datetime('now')
         WHERE NOT EXISTS (
             SELECT 1
             FROM page_views
             WHERE page_path = ?
               AND viewer_ip IS ?
               AND user_agent IS ?
               AND viewed_at >= datetime('now', '-1 hour')
         )",
    )
    .bind(page_path)
    .bind(viewer_ip.as_deref())
    .bind(user_agent.as_deref())
    .bind(page_path)
    .bind(viewer_ip.as_deref())
    .bind(user_agent.as_deref())
    .execute(pool)
    .await?;

    Ok(())
}

/// Increment the site-wide page visit counter.
pub async fn increment_total_page_visits(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO site_counters (counter_name, count, updated_at)
         VALUES (?, 1, datetime('now'))
         ON CONFLICT(counter_name) DO UPDATE SET
            count = count + 1,
            updated_at = datetime('now')",
    )
    .bind(TOTAL_PAGE_VISITS_COUNTER)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get the site-wide page visit counter.
pub async fn get_total_page_visits(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "SELECT count
         FROM site_counters
         WHERE counter_name = ?",
    )
    .bind(TOTAL_PAGE_VISITS_COUNTER)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| row.get("count")).unwrap_or(0))
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
    let total_views = get_total_page_visits(pool).await?;
    let unique_visitors = sqlx::query(
        "SELECT
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
        total_views,
        unique_visitors: unique_visitors.get("unique_visitors"),
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

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("connect in-memory sqlite");
        sqlx::raw_sql(include_str!("../database/schema.sql"))
            .execute(&pool)
            .await
            .expect("initialize schema");
        pool
    }

    #[tokio::test]
    async fn records_total_page_visits_without_page_dedupe() {
        let pool = test_pool().await;

        record_page_view(
            &pool,
            "/",
            Some("127.0.0.0".to_string()),
            Some("test-agent".to_string()),
        )
        .await
        .expect("record first page view");
        record_page_view(
            &pool,
            "/",
            Some("127.0.0.0".to_string()),
            Some("test-agent".to_string()),
        )
        .await
        .expect("record second page view");

        let total_page_visits = get_total_page_visits(&pool)
            .await
            .expect("get total page visits");
        let page_views = get_page_view_count(&pool, "/")
            .await
            .expect("get page views");

        assert_eq!(total_page_visits, 2);
        assert_eq!(page_views.total_views, 1);
    }

    #[tokio::test]
    async fn site_stats_use_total_page_visit_counter() {
        let pool = test_pool().await;

        record_page_view(&pool, "/", Some("127.0.0.0".to_string()), None)
            .await
            .expect("record first page view");
        record_page_view(&pool, "/about", Some("127.0.0.0".to_string()), None)
            .await
            .expect("record second page view");

        let stats = get_site_stats(&pool).await.expect("get site stats");

        assert_eq!(stats.total_views, 2);
        assert_eq!(stats.unique_visitors, 1);
        assert_eq!(stats.pages.len(), 2);
    }
}
