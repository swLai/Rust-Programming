// Simple Expense Tracker - Demonstrating Control Structures in Rust
// This example covers: if/else, match, while loops, for loops, break, and continue

fn main() {
    // Store expenses as a vector of tuples: (category, amount)
    let mut expenses: Vec<(String, f64)> = Vec::new();
    let mut running = true;

    println!("========================================");
    println!("   Welcome to Simple Expense Tracker   ");
    println!("========================================\n");

    // Main program loop using while
    while running {
        println!("\n--- Main Menu ---");
        println!("1. Add Expense");
        println!("2. View All Expenses");
        println!("3. View Summary by Category");
        println!("4. Find Expenses Above Amount");
        println!("5. Exit");
        println!("-----------------");

        // Read user choice
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue; // Skip to next iteration
            }
        };

        // Match expression for menu selection
        match choice {
            1 => {
                // Add expense
                println!("\n--- Add New Expense ---");
                println!("Select category:");
                println!("1. Food");
                println!("2. Transport");
                println!("3. Entertainment");
                println!("4. Utilities");
                println!("5. Other");

                let mut cat_choice = String::new();
                std::io::stdin()
                    .read_line(&mut cat_choice)
                    .expect("Failed to read input");
                let cat_choice: u32 = cat_choice.trim().parse().unwrap_or(0);

                // Match with multiple arms for category selection
                let category = match cat_choice {
                    1 => "Food",
                    2 => "Transport",
                    3 => "Entertainment",
                    4 => "Utilities",
                    5 => "Other",
                    _ => {
                        println!("Invalid category!");
                        continue;
                    }
                };

                println!("Enter amount: ");
                let mut amount_str = String::new();
                std::io::stdin()
                    .read_line(&mut amount_str)
                    .expect("Failed to read input");
                let amount: f64 = amount_str.trim().parse().unwrap_or(0.0);

                // Nested if for validation
                if amount > 0.0 {
                    if amount > 10000.0 {
                        println!("Warning: This is a large expense!");
                    }
                    expenses.push((category.to_string(), amount));
                    println!("Expense added: {} - ${:.2}", category, amount);
                } else {
                    println!("Invalid amount! Must be greater than 0.");
                }
            }

            2 => {
                // View all expenses using for loop
                println!("\n--- All Expenses ---");
                if expenses.is_empty() {
                    println!("No expenses recorded yet.");
                } else {
                    let mut total = 0.0;
                    // For loop with index using range
                    for i in 0..expenses.len() {
                        println!(
                            "{}. {} - ${:.2}",
                            i + 1,
                            expenses[i].0,
                            expenses[i].1
                        );
                        total += expenses[i].1;
                    }
                    println!("-----------------");
                    println!("Total: ${:.2}", total);
                }
            }

            3 => {
                // Category summary using for loop with iter()
                println!("\n--- Summary by Category ---");

                let categories = ["Food", "Transport", "Entertainment", "Utilities", "Other"];

                // For loop iterating through categories
                for category in categories.iter() {
                    let mut cat_total = 0.0;
                    let mut cat_count = 0;

                    // Nested for loop to sum expenses per category
                    for expense in expenses.iter() {
                        if expense.0 == *category {
                            cat_total += expense.1;
                            cat_count += 1;
                        }
                    }

                    // If else to only show categories with expenses
                    if cat_count > 0 {
                        println!("{}: ${:.2} ({} items)", category, cat_total, cat_count);
                    }
                }

                // If let style - calculating grand total
                let grand_total = if expenses.is_empty() {
                    0.0
                } else {
                    let mut sum = 0.0;
                    for expense in &expenses {
                        sum += expense.1;
                    }
                    sum
                };
                println!("-----------------");
                println!("Grand Total: ${:.2}", grand_total);
            }

            4 => {
                // Find expenses above a threshold
                println!("\n--- Find Expenses Above Amount ---");
                println!("Enter minimum amount: ");

                let mut threshold_str = String::new();
                std::io::stdin()
                    .read_line(&mut threshold_str)
                    .expect("Failed to read input");
                let threshold: f64 = threshold_str.trim().parse().unwrap_or(0.0);

                println!("\nExpenses above ${:.2}:", threshold);
                let mut found = false;

                // For loop with continue to skip non-matching items
                for expense in expenses.iter() {
                    if expense.1 <= threshold {
                        continue; // Skip expenses below threshold
                    }

                    // Categorize expense size using if else if ladder
                    let size_label = if expense.1 >= 500.0 {
                        "Large"
                    } else if expense.1 >= 100.0 {
                        "Medium"
                    } else {
                        "Small"
                    };

                    println!("  {} - ${:.2} [{}]", expense.0, expense.1, size_label);
                    found = true;
                }

                if !found {
                    println!("  No expenses found above ${:.2}", threshold);
                }
            }

            5 => {
                // Exit using break concept (setting flag to exit while loop)
                println!("\nThank you for using Expense Tracker!");

                // Show final statistics before exiting
                if !expenses.is_empty() {
                    // Using loop with break to find highest expense
                    let mut highest = 0.0;
                    let mut highest_cat = String::new();

                    for expense in &expenses {
                        if expense.1 > highest {
                            highest = expense.1;
                            highest_cat = expense.0.clone();
                        }
                    }

                    println!("Your highest expense was: {} - ${:.2}", highest_cat, highest);
                    println!("Total expenses recorded: {}", expenses.len());
                }

                running = false; // This will exit the while loop
            }

            // Default case using range pattern
            6..=100 => {
                println!("Option {} is not available. Please choose 1-5.", choice);
            }

            _ => {
                println!("Invalid option! Please choose 1-5.");
            }
        }
    }

    println!("Goodbye!");
}
