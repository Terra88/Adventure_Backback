use std::collections::HashMap;

/// Counts occurrences of each item using HashMap's `entry` and `or_insert`.
pub fn count_items(backpack: &[String]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for item in backpack {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

/// Builds and returns a formatted text summary of the backpack contents.
pub fn build_summary(backpack: &[String]) -> String {
    let total_items = backpack.len();
    let counts = count_items(backpack);

    let mut summary = format!("\nItem summary ({total_items} items):\n");
    for (item, count) in &counts {
        summary.push_str(&format!("{item}: {count}\n"));
    }

    summary
}