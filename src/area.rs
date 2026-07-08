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
impl_multiplication!(LengthMeasurement, AreaMeasurement);

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
    fn multiplication_of_area_measurements_work() {
        let length_measurement_one = LengthMeasurement::from(1f64, Prefix::Kilo, LengthUnit::Meter);
        let length_measurement_two = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Yard);
        let area_measurement = length_measurement_one * length_measurement_two;

        assert_eq!(area_measurement.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Meter)), 914.4f64);
    }

    #[test]
    fn division_of_area_measurement_with_length_measurement_works() {
        let area_measurement = AreaMeasurement::from(914.4f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Meter));
        let length_measurement_one = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Yard);
        let length_measurement_two = area_measurement / length_measurement_one;

        assert_eq!(length_measurement_two.to(Prefix::Kilo, LengthUnit::Meter), 1f64);
    }
}