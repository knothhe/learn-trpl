use chapter11_testing::checkout_total;

mod common;

#[test]
fn customer_checks_out_without_a_discount() {
    let cart = common::sample_cart();

    assert_eq!(checkout_total(&cart, None), 2_000);
}

#[test]
fn customer_checks_out_with_a_discount() {
    let cart = common::sample_cart();

    assert_eq!(checkout_total(&cart, Some("SAVE10")), 1_800);
}
