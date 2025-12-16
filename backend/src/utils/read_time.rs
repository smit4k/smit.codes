pub fn estimate_read_time(md: &str) -> u32 {
    let human_readable_text: String = md
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    let word_count = human_readable_text.split_whitespace().count();

    (word_count as u32 / 200).max(1)
}
