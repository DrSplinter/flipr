use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Place {
    x: Decimal,
    y: Decimal,
}
