fn main() {
    // vector
    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    println!("third: {}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("third: {}", third),
        None => println!("third is out of bounds"),
    }

    println!("Hello, world!");
}
