mod content;

use crate::content::parser::parse_markdown;

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

    assert_eq!(fm.title, "Test Post");
    assert!(body.contains("# Hello"));

    println!("Hello, world!");
}
