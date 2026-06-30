/// Calculates the final price of a cart after applying an optional discount.
///
/// ```
/// use chapter11_testing::checkout_total;
///
/// assert_eq!(checkout_total(&[1_200, 800], Some("SAVE10")), 1_800);
/// ```
pub fn checkout_total(prices: &[u64], discount_code: Option<&str>) -> u64 {
    let subtotal = calculate_subtotal(prices);
    let discount = discount_percent(discount_code);

    subtotal * (100 - discount) / 100
}

fn calculate_subtotal(prices: &[u64]) -> u64 {
    prices.iter().sum()
}

fn discount_percent(code: Option<&str>) -> u64 {
    match code {
        Some("SAVE10") => 10,
        Some("SAVE20") => 20,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_subtotal_from_all_items() {
        assert_eq!(calculate_subtotal(&[1_200, 800, 500]), 2_500);
    }

    #[test]
    fn recognizes_known_discount_code() {
        assert_eq!(discount_percent(Some("SAVE20")), 20);
    }

    #[test]
    fn rejects_unknown_discount_code() {
        assert_eq!(discount_percent(Some("NOT-A-CODE")), 0);
    }
}
