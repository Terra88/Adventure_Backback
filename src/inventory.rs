/// Adds a new item to the backpack vector.
pub fn add_item(backpack: &mut Vec<String>, item: &str) {
    backpack.push(item.to_string());
}

/// Displays all items currently in the backpack.
pub fn display_items(backpack: &[String]) {
    println!("Backpack:");
    for item in backpack {
        println!("- {item}");
    }
}