mod inventory;
mod summary;

fn main() {
    // 1. Create a mutable Vec<String> with at least five items (including duplicates)
    let mut backpack = vec![
        "Rope".to_string(),
        "Potion".to_string(),
        "Map".to_string(),
        "Map".to_string(),
        "Torch".to_string(),
    ];

    // 2. Add an item using the inventory module function
    inventory::add_item(&mut backpack, "Silver key");

    // 3. Display the backpack contents
    inventory::display_items(&backpack);

    // 4. Build and print the item summary
    let summary_text = summary::build_summary(&backpack);
    print!("{summary_text}");
}
