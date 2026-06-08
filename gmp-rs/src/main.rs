use gmp_rs::Bigint;

fn main() {
    let mut a = Bigint::new(0);
    let mut b = Bigint::new(1);
    let mut c;
    for _ in 0..10000 {
        println!("{a}");

        c = a.clone();
        c.add(&b);
        a = b.clone();
        b = c.clone();
    }
}
