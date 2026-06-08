use gmp_rs::Bigint;

fn main() {
    let mut a = Bigint::new(0);
    let mut b = Bigint::new(1);
    for _ in 0..10000 {
        println!("{a}");

        a.add(&b);
        std::mem::swap(&mut a, &mut b);
    }
}
