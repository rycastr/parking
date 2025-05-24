#[derive(Debug, PartialEq)]
pub enum ModelError {
    SessionAlreadyClosed,
    InvalidCheckoutDate,
}
