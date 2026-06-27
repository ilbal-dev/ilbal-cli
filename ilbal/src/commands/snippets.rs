use dialoguer::{Input, Select, Confirm, MultiSelect};

// --- Text input ---
let name: String = Input::new()
    .with_prompt("Project name")
    .default("my-project".into())
    .interact_text()?;

// --- Single-select dropdown ---
let databases = &["PostgreSQL", "SQLite", "MySQL"];
let selection: usize = Select::new()
    .with_prompt("Choose a database")
    .items(databases)
    .default(0)
    .interact()?;
println!("Selected: {}", databases[selection]);

// --- Yes/No confirm ---
let proceed = Confirm::new()
    .with_prompt("Proceed?")
    .default(true)
    .interact()?;

// --- Multi-select checkboxes ---
let choices = &["Feature A", "Feature B", "Feature C"];
let picks: Vec<usize> = MultiSelect::new()
    .with_prompt("Select features")
    .items(choices)
    .interact()?;
for i in picks {
    println!("Picked: {}", choices[i]);
}

// --- Fuzzy search dropdown (great for long lists) ---
let cities = &["Tokyo", "London", "New York", /* … */];
let city: usize = FuzzySelect::new()
    .with_prompt("Search for a city")
    .items(cities)
    .default(0)
    .interact()?;
