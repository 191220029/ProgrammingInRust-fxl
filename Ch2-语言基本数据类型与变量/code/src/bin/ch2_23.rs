use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90); //创建一个Alice-90的键值对

    assert_eq!(scores.get("Alice"), Some(&90));
    assert_eq!(scores.get("Marisa"), None);
}
