fn main() {
    // ownership_example_01();
    // ownership_example_02();
    // ownership_reference();
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);
}

fn ownership_reference() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn ownership_example_02() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn ownership_example_01() {
    // 1. the string “Ferris” has been allocated on the heap. It is owned by first.
    let first = String::from("Ferris");
    // 2. the function add_suffix(first) has been called.
    // This moves ownership of the string from first to name.
    // The string data is not copied, but the pointer to the data is copied.
    let full = add_suffix(first);
    // 4. the frame for add_suffix is gone.
    // This function returned name, transferring ownership of the string to full.
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    // 3. the function name.push_str(" Jr.") resizes the string’s heap allocation.
    // This does three things.
    // First, it creates a new larger allocation.
    // Second, it writes “Ferris Jr.” into the new allocation.
    // Third, it frees the original heap memory.
    // first now points to deallocated memory.
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) {
    // note the ampersands
    println!("{} {}!", g1, g2);
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}
