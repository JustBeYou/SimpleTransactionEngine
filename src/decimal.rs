/**
 * Simple implementation of fixed point numbers. Overflows are not handled
 * as we assume that no one would have assets close to 2^63.
 */
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decimal<const PRECISION: u32> {
    n: i64,
}

impl<const PRECISION: u32> Serialize for Decimal<PRECISION> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("Decimal", &Into::<f64>::into(*self))
    }
}

impl<'de, const PRECISION: u32> Deserialize<'de> for Decimal<PRECISION> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if let Ok(n) = s.parse::<f64>() {
            Ok(Decimal::from(n))
        } else {
            Ok(Decimal::zero())
        }
    }
}

impl<const PRECISION: u32> Decimal<PRECISION> {
    pub const fn zero() -> Self {
        Self { n: 0 }
    }
}

const fn ten_pow(n: u32) -> i64 {
    (10 as i64).pow(n)
}

impl<const PRECISION: u32> From<f64> for Decimal<PRECISION> {
    fn from(n: f64) -> Decimal<PRECISION> {
        Self {
            n: (n * ten_pow(PRECISION) as f64) as i64,
        }
    }
}

impl<const PRECISION: u32> From<i64> for Decimal<PRECISION> {
    fn from(n: i64) -> Decimal<PRECISION> {
        Self {
            n: n * ten_pow(PRECISION),
        }
    }
}

impl<const PRECISION: u32> Into<f64> for Decimal<PRECISION> {
    fn into(self) -> f64 {
        self.n as f64 / ten_pow(PRECISION) as f64
    }
}

impl<const PRECISION: u32> Display for Decimal<PRECISION> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.n / ten_pow(PRECISION),
            self.n % ten_pow(PRECISION)
        )
    }
}

#[macro_export]
macro_rules! basic_op {
    ( $name:ident, $op:ident, $method:ident) => {
        impl<const PRECISION: u32> $op for $name<PRECISION> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    n: self.n.$method(rhs.n),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! basic_assign_op {
    ( $name:ident, $op:ident, $method:ident) => {
        impl<const PRECISION: u32> $op for $name<PRECISION> {
            fn $method(&mut self, rhs: Self) {
                self.n.$method(rhs.n);
            }
        }
    };
}

basic_op!(Decimal, Add, add);
basic_op!(Decimal, Sub, sub);
basic_assign_op!(Decimal, AddAssign, add_assign);
basic_assign_op!(Decimal, SubAssign, sub_assign);

#[cfg(test)]
mod tests {
    use super::Decimal;

    #[test]
    fn into_float() {
        assert_eq!(Into::<f64>::into(Decimal::<3>::from(10.12345)), 10.123);
        assert_eq!(Into::<f64>::into(Decimal::<3>::from(-10.12345)), -10.123);
    }

    #[test]
    fn ops_precision() {
        let mut res =
            Decimal::<3>::from(1.250) + Decimal::<3>::from(3.350) - Decimal::<3>::from(2.150);
        res += Decimal::<3>::from(1);
        assert_eq!(Into::<f64>::into(res), 3.450);
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", Decimal::<3>::from(1.2349)), "1.234");
    }
}
