//! # Macros for Internal Use

/// # Implement Addition and Subtraction for Measurement Types
/// 
/// Implements `+`, `-`, `+=`, and `-=` operators for measurement types defined in the crate. For internal use.
macro_rules! impl_add_and_subtract {
    ($type: ty) => {
        use std::ops::{ Add, Sub, AddAssign, SubAssign };
        
        impl Add for $type {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    value: self.value + other.value,
                }
            }
        }

        impl Sub for $type {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    value: self.value - other.value,
                }
            }
        }

        impl AddAssign for $type {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    value: self.value + other.value,
                };
            }
        }

        impl SubAssign for $type {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    value: self.value - other.value,
                };
            }
        }
    }
}

pub(crate) use impl_add_and_subtract;

/// # Check for Near Equality
/// 
/// Checks that two numbers are within 10<sup>-15</sup> of each other, to account for rouding errors due to floating point operations. For internal use.
#[cfg(test)]
macro_rules! assert_almost_eq {
    ($left: expr, $right: expr) => {
        const EPSILON: f64 = 1e-15f64;
        assert!(($left-$right).abs() < EPSILON);
    }
}

#[cfg(test)]
pub(crate) use assert_almost_eq;