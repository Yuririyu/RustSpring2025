fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone(); // clone original string
    cloned.push_str("World!");  // append to the cloned string
    cloned // return cloned
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}