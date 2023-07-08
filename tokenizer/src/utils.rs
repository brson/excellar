pub fn require_positive(amount: i128) {
    if amount < 0 {
        panic!("amount must be positive");
    }
}
pub fn require_strictly_positive(amount: i128) {
    if amount <= 0 {
        panic!("amount must be strictly positive");
    }
}
