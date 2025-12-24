pub mod db;
pub mod handlers;
pub mod views;

pub use handlers::{get_post_views, record_post_view};

// Re-export the views functions/types
pub use views::{get_top_viewed, get_view_count, record_view, ViewCount};

pub use db::create_pool;
