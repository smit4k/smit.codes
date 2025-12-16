mod content;
mod utils;

use crate::content::loader::create_content_item;

fn main() {
    let md = r#"
---
title: Test Post
date: 2025-01-01
tags: [rust, svelte]
---

# Hello

This is a test.
"#;

    let item = create_content_item(md, "test-post").unwrap();

    println!("Title: {}", item.frontmatter.title);
    println!("Read time: {} min", item.read_time);
    println!("Markdown body:\n{}", item.markdown);
}
