//! # Units and Operations Pertaining to Area
//! 
//! The base unit used to store area in the `unitconverter` crate is square meters.

use crate::macros::*;
use crate::formulas::*;

use crate::length::{ LengthUnit, LengthMeasurement };

/// # Units of Area
/// 
/// Units for measurement of area.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum AreaUnit {
    /// Defined as 100 m<sup>2</sup>. Represented by the symbol a.<sup>1</sup>
    Are,
    /// The square of any length unit supported by the `unitconverter` crate.
    Square(Prefix, LengthUnit),
}

impl AreaUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                AreaUnit::Are => Formula::Multiply{ scale: 100f64 },
                AreaUnit::Square(prefix, unit) => {
                    Formula::Multiply{
                        scale: unit
                            .to_base_unit(Prefix::conversion_constant(&prefix, &Prefix::None))
                            .powi(2)
                    }
                },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                AreaUnit::Are => Formula::Divide{ scale: 100f64 },
                AreaUnit::Square(prefix, unit) => {
                    Formula::Divide{
                        scale: unit
                            .to_base_unit(Prefix::conversion_constant(&prefix, &Prefix::None))
                            .powi(2)
                    }
                },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                AreaUnit::Are => "are".to_string(),
                AreaUnit::Square(prefix, unit) => {
                    let prefix_name = match prefix.name() {
                        Some(name) => name,
                        None => "",
                    };
                    format!("square {}{}", prefix_name, unit.name_singular())
                },
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                AreaUnit::Are => "ares".to_string(),
                AreaUnit::Square(prefix, unit) => {
                    let prefix_name = match prefix.name() {
                        Some(name) => name,
                        None => "",
                    };
                    format!("square {}{}", prefix_name, unit.name_plural())
                },
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                AreaUnit::Are => "a".to_string(),
                AreaUnit::Square(prefix, unit) => {
                    let prefix_name = match prefix.symbol() {
                        Some(symbol) => symbol,
                        None => "",
                    };
                    format!("{}{}²", prefix_name, unit.symbol())
                }
            }
        }
    }
}

impl_measurement!(AreaMeasurement, AreaUnit);
impl_square_multiplication_and_division!(LengthMeasurement, AreaMeasurement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter).to_base_unit(1f64), 1e6f64);
        assert_eq!(AreaUnit::Are.to_base_unit(1f64), 1e2f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter).from_base_unit(1e6f64), 1f64);
        assert_eq!(AreaUnit::Are.from_base_unit(1e2f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(AreaUnit::Are.name_singular(), "are");
        assert_eq!(AreaUnit::Square(Prefix::None, LengthUnit::Inch).name_singular(), "square inch");
        assert_eq!(AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter).name_singular(), "square kilometer");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(AreaUnit::Are.name_plural(), "ares");
        assert_eq!(AreaUnit::Square(Prefix::None, LengthUnit::Inch).name_plural(), "square inches");
        assert_eq!(AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter).name_plural(), "square kilometers");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(AreaUnit::Are.symbol(), "a");
        assert_eq!(AreaUnit::Square(Prefix::None, LengthUnit::Inch).symbol(), "in²");
        assert_eq!(AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter).symbol(), "km²");
    }
    
    #[test]
    fn addition_of_area_measurements_work() {
        let measurement_one = AreaMeasurement::from(12f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let measurement_two = AreaMeasurement::from(6f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch)), 18f64);
    }

    #[test]
    fn subtraction_of_area_measurements_work() {
        let measurement_one = AreaMeasurement::from(12f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let measurement_two = AreaMeasurement::from(6f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch)), 6f64);
    }

        #[test]
    fn addition_assign_of_area_measurements_work() {
        let mut measurement_one = AreaMeasurement::from(12f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let measurement_two = AreaMeasurement::from(6f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch)), 18f64);
    }

    #[test]
    fn subtraction_assign_of_area_measurements_work() {
        let mut measurement_one = AreaMeasurement::from(12f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        let measurement_two = AreaMeasurement::from(6f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch));
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Inch)), 6f64);
    }
}