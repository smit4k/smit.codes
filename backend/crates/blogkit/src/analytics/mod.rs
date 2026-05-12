pub mod db;
pub mod handlers;
pub mod views;

pub use handlers::{analytics_read_router, analytics_router, analytics_write_router};
pub use handlers::{get_all_page_views, get_page_views, track_page_view};
pub use handlers::{get_post_views, record_post_view, AnalyticsState};

// Re-export the views functions/types
pub use views::{get_page_view_count, get_site_stats, record_page_view, PageViewCount, SiteStats};
pub use views::{get_top_viewed, get_view_count, record_view, ViewCount};

pub use db::create_pool;
