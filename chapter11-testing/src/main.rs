use chapter11_testing::checkout_total;

fn main() {
    let total = checkout_total(&[1_200, 800], Some("SAVE10"));
    println!("Checkout total: {total} cents");
}
