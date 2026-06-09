use gmp_rs::Bigint;
use std::time::Instant;

fn main() {
    let mut a = Bigint::new(0);
    let mut b = Bigint::new(1);

    let start_now = Instant::now();
    for _ in 0..3000_00 {
        a.add(&b);
        std::mem::swap(&mut a, &mut b);
    }
    let end_now = Instant::now();
    // new_now.duration_since(now)
    let elapsed_time_sec = end_now.duration_since(start_now).as_secs_f64();
    // print!("{a}\n");
    print!("elapsed time with bigint: {elapsed_time_sec}");
}
