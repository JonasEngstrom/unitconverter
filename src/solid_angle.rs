//! # Units and Operations Pertaining to Solid Angles
//!
//! The base unit used to store solid angles in the `unitconverter` crate is steradian. Note that units of plane and phase angles are handled in the module [`angle.rs`][crate::angle].

use crate::formulas::*;
use crate::macros::*;

/// # Units of Solid Angles
///
/// Units for measurement of solid angles.
///
/// ## References
///
/// 1. [Le Système international d'unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
pub enum SolidAngleUnit {
    /// Defined as the solid angle subtended at the center of a sphere by an area of the surface that is equal to the squared radius. A complete sphere subtends 4π sr about its center.<sup>1</sup> Represented by the symbol sr.<sup>1</sup>
    Steradian,
}

impl SolidAngleUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                SolidAngleUnit::Steradian => Formula::Multiply{ scale: 1f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                SolidAngleUnit::Steradian => Formula::Divide{ scale: 1f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                SolidAngleUnit::Steradian => "steradian".to_string(),
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                SolidAngleUnit::Steradian => "steradians".to_string(),
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                SolidAngleUnit::Steradian => "sr".to_string(),
            }
        }
    }
}

impl_measurement!(SolidAngleMeasurement, SolidAngleUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(SolidAngleUnit::Steradian.to_base_unit(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(SolidAngleUnit::Steradian.from_base_unit(1f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(SolidAngleUnit::Steradian.name_singular(), "steradian");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(SolidAngleUnit::Steradian.name_plural(), "steradians");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(SolidAngleUnit::Steradian.symbol(), "sr");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = SolidAngleMeasurement::from(1f64, Prefix::Kilo, SolidAngleUnit::Steradian);
        let test_result = test_measurement.to(Prefix::None, SolidAngleUnit::Steradian);

        assert_eq!(test_result, 1_000f64);
    }

    #[test]
    fn addition_of_solid_angle_measurements_work() {
        let measurement_one = SolidAngleMeasurement::from(2f64, Prefix::None, SolidAngleUnit::Steradian);
        let measurement_two = SolidAngleMeasurement::from(1f64, Prefix::None, SolidAngleUnit::Steradian);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, SolidAngleUnit::Steradian), 3f64);
    }

    #[test]
    fn subtraction_of_solid_angle_measurements_work() {
        let measurement_one = SolidAngleMeasurement::from(2f64, Prefix::None, SolidAngleUnit::Steradian);
        let measurement_two = SolidAngleMeasurement::from(1f64, Prefix::None, SolidAngleUnit::Steradian);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, SolidAngleUnit::Steradian), 1f64);
    }

    #[test]
    fn addition_assign_of_solid_angle_measurements_work() {
        let mut measurement_one = SolidAngleMeasurement::from(2f64, Prefix::None, SolidAngleUnit::Steradian);
        let measurement_two = SolidAngleMeasurement::from(1f64, Prefix::None, SolidAngleUnit::Steradian);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, SolidAngleUnit::Steradian), 3f64);
    }

    #[test]
    fn subtraction_assign_of_solid_angle_measurements_work() {
        let mut measurement_one = SolidAngleMeasurement::from(2f64, Prefix::None, SolidAngleUnit::Steradian);
        let measurement_two = SolidAngleMeasurement::from(1f64, Prefix::None, SolidAngleUnit::Steradian);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, SolidAngleUnit::Steradian), 1f64);
    }
}
