// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// I AM DONE
use std::any::Any;

fn main() {
    let mut shopping_list: Vec<Box<dyn Any>> = Vec::new();
    shopping_list.push(Box::new("milk"));
}
