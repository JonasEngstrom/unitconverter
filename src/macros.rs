//! # Macros for Internal Use

/// # Implement Shared Methods for Measurement Types
/// 
/// Implements methods shared across measurement types, as well as `+`, `-`, `+=`, and `-=` operators for measurement types. For internal use.
macro_rules! impl_measurement {
    ($measurement_identifier: ident, $unit_type: ty) => {
        use std::ops::{ Add, Sub, AddAssign, SubAssign };
        use si_prefixes::Prefix;

        /// # Stores a Measurement
        /// 
        #[doc = concat!("All measurements in the `unitconverter` crate are handled in the same way. They are stored as an `f64` in the base unit of the quantity in question. The types of measurements of different quantities are kept distinct, to enable arithmetic that make sense from a physical standpoint. For more information about supported units for `", stringify!($measurement_identifier), "`s, see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page. For supported prefixes, see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation.")]
        /// 
        /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
        /// 
        /// ## General Usage of Measurement Types
        /// 
        /// ```rust
        /// use unitconverter::length::{ LengthUnit, LengthMeasurement };
        /// use si_prefixes::Prefix;
        /// 
        /// // Store 25.4 cm in a variable.
        /// let centimeters = LengthMeasurement::from(25.4f64, Prefix::Centi, LengthUnit::Meter);
        /// 
        /// // Convert the stored measurement to inches.
        /// let centimeters_in_inches = centimeters.to(Prefix::None, LengthUnit::Inch);
        /// 
        /// // Check that the conversion yields the expected 10 inches.
        /// assert_eq!(centimeters_in_inches, 10f64);
        /// ```
        pub struct $measurement_identifier { value: f64 }
        
        impl $measurement_identifier {
            #[doc = concat!("Stores a New `", stringify!($measurement_identifier), "`")]
            /// 
            #[doc = concat!("Store a new measurement from an `f64` value, an SI prefix, and a unit. For a general code example, see [*General Usage of Measurement Types*](#general-usage-of-measurement-types) above. For supported prefixes see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation, and for supported units see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page.")]
            /// 
            /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
            pub fn from(value: f64, prefix: Prefix, unit: $unit_type) -> Self {
                Self {
                    value: unit.to_base_unit(Prefix::conversion_constant(&prefix, &Prefix::None) * value)
                }
            }
    
            #[doc = concat!("Output a Stored `", stringify!($measurement_identifier), "`")]
            /// 
            #[doc = concat!("Outputs a stored measurement in a desired unit with an SI prefix. For a general code example, see [*General Usage of Measurement Types*](#general-usage-of-measurement-types) above. For supported prefixes see the [`si-prefixes`](https://crates.io/crates/si-prefixes) crate documentation, and for supported units see the [`", stringify!($unit_type), "`](", stringify!($unit_type), ") documentation page.")]
            /// 
            /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
            pub fn to(&self, prefix: Prefix, unit: $unit_type) -> f64 {
                Prefix::conversion_constant(&Prefix::None, &prefix) * unit.from_base_unit(self.value)
            }

            /// Make new measurement, by setting value directly. Only used internally for arithmetic.
            pub(crate) fn new_from_value(value: f64) -> Self {
                Self{ value }
            }

            /// Get value directly. Only used internally for arithmetic.
            pub(crate) fn get_value(&self) -> f64 {
                self.value
            }
        }

        impl Add for $measurement_identifier {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self::new_from_value(self.get_value() + other.get_value())
            }
        }

        impl Sub for $measurement_identifier {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self::new_from_value(self.get_value() - other.get_value())
            }
        }

        impl AddAssign for $measurement_identifier {
            fn add_assign(&mut self, other: Self) {
                *self = Self::new_from_value(self.get_value() + other.get_value());
            }
        }

        impl SubAssign for $measurement_identifier {
            fn sub_assign(&mut self, other: Self) {
                *self = Self::new_from_value(self.get_value() - other.get_value());
            }
        }
    }
}

pub(crate) use impl_measurement;

macro_rules! impl_square_multiplication_and_division {
    ($factor_measurement_type: ty, $product_measurement_type: ident) => {
        use std::ops::{ Mul, Div };

        impl Mul for $factor_measurement_type {
            type Output = $product_measurement_type;

            fn mul(self, other: Self) -> $product_measurement_type {
                $product_measurement_type::new_from_value(self.get_value() * other.get_value())
            }
        }

        impl Div<$factor_measurement_type> for $product_measurement_type {
            type Output = $factor_measurement_type;

            fn div(self, rhs: $factor_measurement_type) -> $factor_measurement_type {
                <$factor_measurement_type>::new_from_value(self.get_value() / rhs.get_value())
            }
        }
    }
}

pub(crate) use impl_square_multiplication_and_division;

macro_rules! impl_multiplication_and_division {
    (
        $left_factor_measurement_type: ty,
        $right_factor_measurement_type: ty,
        $product_measurement_type: ty
    ) => {
        impl std::ops::Mul<$right_factor_measurement_type> for $left_factor_measurement_type {
            type Output = $product_measurement_type;

            fn mul(self, rhs: $right_factor_measurement_type) -> $product_measurement_type {
                <$product_measurement_type>::new_from_value(
                    self.get_value() * rhs.get_value()
                )
            }
        }

        impl std::ops::Mul<$left_factor_measurement_type> for $right_factor_measurement_type {
            type Output = $product_measurement_type;

            fn mul(self, rhs: $left_factor_measurement_type) -> $product_measurement_type {
                <$product_measurement_type>::new_from_value(
                    self.get_value() * rhs.get_value()
                )
            }
        }

        impl std::ops::Div<$right_factor_measurement_type> for $product_measurement_type {
            type Output = $left_factor_measurement_type;

            fn div(self, rhs: $right_factor_measurement_type) -> $left_factor_measurement_type {
                <$left_factor_measurement_type>::new_from_value(
                    self.get_value() / rhs.get_value()
                )
            }
        }

        impl std::ops::Div<$left_factor_measurement_type> for $product_measurement_type {
            type Output = $right_factor_measurement_type;

            fn div(self, rhs: $left_factor_measurement_type) -> $right_factor_measurement_type {
                <$right_factor_measurement_type>::new_from_value(
                    self.get_value() / rhs.get_value()
                )
            }
        }
    }
}

pub(crate) use impl_multiplication_and_division;

macro_rules! doc_to_base_unit_formula {
    ($to_base_unit_formula: item) => {
        /// Returns the a formula used to convert an f64 into the base unit of the quantity.
        $to_base_unit_formula
        
        /// Returns an f64 converted from a unit into the base unit of the quantity.
        pub(crate) fn to_base_unit(&self, value_to_convert: f64) -> f64 {
            self.to_base_unit_formula().apply(value_to_convert)
        }
    }
}

pub(crate) use doc_to_base_unit_formula;

macro_rules! doc_from_base_unit_formula {
    ($from_base_unit_formula: item) => {
        /// Returns the a formula used to convert an f64 from the base unit into another unit.
        $from_base_unit_formula

        /// Returns an f64 converted from the base unit of a quantity into another unit.
        pub(crate) fn from_base_unit(&self, value_to_convert: f64) -> f64 {
            self.from_base_unit_formula().apply(value_to_convert)
        }
    }
}

pub(crate) use doc_from_base_unit_formula;

macro_rules! doc_name_singular {
    ($name_singular: item) => {
        /// Returns the name of a unit in singular.
        /// 
        /// All units in the `unitconverter` crate are handled in the same way, so below is a general example of how to use the `name_singular` method for a unit.
        /// 
        /// ```
        /// use unitconverter::temperature::TemperatureUnit;
        /// 
        /// let celsius = TemperatureUnit::Celsius;
        /// 
        /// assert_eq!(celsius.name_singular(), "degree Celsius");
        /// ```
        /// 
        /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
        $name_singular
    }
}

pub(crate) use doc_name_singular;

macro_rules! doc_name_plural {
    ($name_plural: item) => {
        /// Returns the name of a unit in plural.
        /// 
        /// All units in the `unitconverter` crate are handled in the same way, so below is a general example of how to use the `name_plural` method for a unit.
        /// 
        /// ```
        /// use unitconverter::temperature::TemperatureUnit;
        /// 
        /// let celsius = TemperatureUnit::Celsius;
        /// 
        /// assert_eq!(celsius.name_plural(), "degrees Celsius");
        /// ```
        /// 
        /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
        $name_plural
    }
}

pub(crate) use doc_name_plural;

macro_rules! doc_symbol {
    ($symbol: item) => {
        /// Returns the symbol of a unit.
        /// 
        /// All units in the `unitconverter` crate are handled in the same way, so below is a general example of how to use the `symbol` method for a unit.
        /// 
        /// ```
        /// use unitconverter::temperature::TemperatureUnit;
        /// 
        /// let celsius = TemperatureUnit::Celsius;
        /// 
        /// assert_eq!(celsius.symbol(), "°C");
        /// ```
        /// 
        /// For more information on how to use the crate, including code examples, see [the crate documentation root page](crate) or the `README.md` file in [the GitHub repo](https://github.com/JonasEngstrom/unitconverter).
        $symbol
    }
}

pub(crate) use doc_symbol;

/// # Check for Near Equality
/// 
/// Checks that two numbers are within 10<sup>-11</sup> of each other, to account for rouding errors due to floating point operations. For internal use in unit testing.
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