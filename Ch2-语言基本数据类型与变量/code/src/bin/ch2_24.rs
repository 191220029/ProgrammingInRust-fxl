use std::collections::HashSet;
fn main() {
    let mut books = HashSet::new();
    books.insert("The Odyssey".to_string());

    assert!(books.contains("The Odyssey"));
    assert_ne!(books.contains("The Iliad"), true);
}
