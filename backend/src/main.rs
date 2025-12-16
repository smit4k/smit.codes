mod content;
mod utils;

use crate::content::parser::parse_markdown;
use crate::utils::read_time::estimate_read_time;

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

    let (fm, body) = parse_markdown(md).unwrap();

    let read_time = estimate_read_time(&body);
    println!("Read time: {}", read_time);
    assert_eq!(fm.title, "Test Post");
    assert!(body.contains("# Hello"));

    println!("Hello, world!");
}
