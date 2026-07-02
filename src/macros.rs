//! # Macros for Internal Use

/// # Implement Shared Methods for Measurement Types
/// 
/// Implements methods shared across measurement types, as well as `+`, `-`, `+=`, and `-=` operators for measurement types. For internal use.
macro_rules! impl_measurement {
    ($measurement_identifier: ident, $unit_type: ty) => {
        use std::ops::{ Add, Sub, AddAssign, SubAssign };
        use si_prefixes::Prefix;

        /// # Store a Measurement
        /// 
        #[doc = concat!("All measurements in the `unitconverter` crate are handled in the same way. They are stored as an `f64` in the base unit of the quantity in question. The types of measurements of different quantities are kept distinct, to enable arithmetic that make sense from a physical standpoint. For more information about supported units for `", stringify!($measurement_identifier), "`s, see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page. For supported prefixes, see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation.")]
        /// 
        /// ## General Usage of Measurement Types
        /// 
        /// ```rust
        /// use unitconverter::length::{ LengthUnit, LengthMeasurement };
        /// use si_prefixes::Prefix;
        /// 
        /// // Store 24.5 cm in a variable.
        /// let ten_centimeters = LengthMeasurement::from(24.5f64, Prefix::Centi, LengthUnit::Meter);
        /// 
        /// // Convert the stored measurement to inches.
        /// let ten_centimeters_in_inches = ten_centimeters.to(Prefix::None, LengthUnit::Inch)
        /// 
        /// // Check that the conversion yields the expected 10 inches.
        /// assert_eq!(ten_centimeters_in_inches, 10f64);
        /// ```
        pub struct $measurement_identifier { value: f64 }
        
        impl $measurement_identifier {
            #[doc = concat!("# Store a New `", stringify!($measurement_identifier), "`")]
            /// 
            #[doc = concat!("Store a new measurement from an `f64` value, an SI prefix, and a unit. For a general code example, see [*General Usage of Measurement Types*](#general-usage-of-measurement-types) above. For supported prefixes see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation, and for supported units see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page.")]
            pub fn from(value: f64, prefix: Prefix, unit: $unit_type) -> Self {
                Self {
                    value: unit.to_base_unit()(Prefix::conversion_constant(prefix, Prefix::None) * value)
                }
            }
    
            #[doc = concat!("# Output a Stored `", stringify!($measurement_identifier), "`")]
            /// 
            #[doc = concat!("Output a stored measurement in a desired unit with an SI prefix. For a general code example, see [*General Usage of Measurement Types*](#general-usage-of-measurement-types) above. For supported prefixes see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation, and for supported units see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page.")]
            pub fn to(&self, prefix: Prefix, unit: $unit_type) -> f64 {
                Prefix::conversion_constant(Prefix::None, prefix) * unit.from_base_unit()(self.value)
            }
        }

        impl Add for $measurement_identifier {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    value: self.value + other.value,
                }
            }
        }

        impl Sub for $measurement_identifier {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    value: self.value - other.value,
                }
            }
        }

        impl AddAssign for $measurement_identifier {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    value: self.value + other.value,
                };
            }
        }

        impl SubAssign for $measurement_identifier {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    value: self.value - other.value,
                };
            }
        }
    }
}

pub(crate) use impl_measurement;

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
        {
            const EPSILON: f64 = 1e-11f64;
            assert!(($left-$right).abs() < EPSILON);
        }
    }
}

#[cfg(test)]
pub(crate) use assert_almost_eq;