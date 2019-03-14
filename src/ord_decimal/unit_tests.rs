use super::*;
use fraction::Bounded;

#[test]
fn cmp_nan_eq_nan() {
    assert_eq!(OrdDecimal(Decimal::nan()), OrdDecimal(Decimal::nan()));
}

#[test]
fn cmp_not_nan_ne_nan() {
    assert_ne!(OrdDecimal(Decimal::from(42)), OrdDecimal(Decimal::nan()));
}

#[test]
fn cmp_inf_eq_inf() {
    assert_eq!(OrdDecimal(Decimal::infinity()), OrdDecimal(Decimal::infinity()));
}

#[test]
fn cmp_not_inf_ne_inf() {
    assert_ne!(OrdDecimal(Decimal::from(42)), OrdDecimal(Decimal::infinity()));
}

#[test]
fn cmp_neg_inf_neg_eq_inf() {
    assert_eq!(OrdDecimal(Decimal::neg_infinity()), OrdDecimal(Decimal::neg_infinity()));
}

#[test]
fn cmp_not_neg_inf_ne_neg_inf() {
    assert_ne!(OrdDecimal(Decimal::from(-42)), OrdDecimal(Decimal::neg_infinity()));
}

#[test]
fn cmp_num_eq_num() {
    assert_eq!(OrdDecimal(Decimal::from(42)), OrdDecimal(Decimal::from(42)));
}

#[test]
fn add() {
    assert_eq!(OrdDecimal::from(42), OrdDecimal::from(41) + OrdDecimal::from(1));
}

#[test]
fn checked_add_nominal() {
    assert_eq!(Some(OrdDecimal::from(42)), OrdDecimal::from(41).checked_add(&OrdDecimal::from(1)));
}

#[test]
fn checked_add_overflow() {
    assert_eq!(None, OrdDecimal::max_value().checked_add(&OrdDecimal::from(1)));
}

#[test]
fn sub() {
    assert_eq!(OrdDecimal::from(42), OrdDecimal::from(43) - OrdDecimal::from(1));
}

#[test]
fn checked_sub_nominal() {
    assert_eq!(Some(OrdDecimal::from(42)), OrdDecimal::from(43).checked_sub(&OrdDecimal::from(1)));
}

#[test]
fn checked_sub_overflow() {
    assert_eq!(None, OrdDecimal::min_value().checked_sub(&OrdDecimal::from(1)));
}

#[test]
fn mul() {
    assert_eq!(OrdDecimal::from(42), OrdDecimal::from(21) * OrdDecimal::from(2));
}

#[test]
fn checked_mul_nominal() {
    assert_eq!(Some(OrdDecimal::from(42)), OrdDecimal::from(21).checked_mul(&OrdDecimal::from(2)));
}

#[test]
fn checked_mul_overflow() {
    assert_eq!(None, OrdDecimal::max_value().checked_mul(&OrdDecimal::from(1.1)));
}

#[test]
fn div() {
    assert_eq!(OrdDecimal::from(42), OrdDecimal::from(84) / OrdDecimal::from(2));
}

#[test]
fn checked_div_nominal() {
    assert_eq!(Some(OrdDecimal::from(42)), OrdDecimal::from(84).checked_div(&OrdDecimal::from(2)));
}

#[test]
fn checked_div_overflow() {
    assert_eq!(None, OrdDecimal::min_value().checked_div(&OrdDecimal::from(0.1)));
}

