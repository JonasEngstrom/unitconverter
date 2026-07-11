//! # Units and Operations Pertaining to Plane and Phase Angles
//! 
//! The base unit used to store plane and phase angles in the `unitconverter` crate is radians.

use crate::macros::*;
use crate::formulas::*;

use std::f64::consts::PI;

/// # Units of Plane and Phase Angles
/// 
/// Units for measurement of plane and phase angles.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum AngleUnit {
    /// Defined as 2 × π per revolution. Represented by the symbol rad.<sup>1</sup>
    Radian,
    /// Defined as π / 180 radians. Represented by the symbol °.<sup>1</sup>
    Degree,
    /// Defined as 1 / 60 degrees which is equal to π / 10,800 radians. Represented by the symbol ′.<sup>1</sup>
    Minute,
    /// Defined as 1 / 60 minutes which is equal to π / 648,000 radians. Represented by the symbol ″ in the `unitconveter` crate, but can also be represented by the symbol as to signify arcseconds.<sup>1</sup>
    Second,
}

impl AngleUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                AngleUnit::Radian => Formula::Multiply{ scale: 1f64 },
                AngleUnit::Degree => Formula::Multiply{ scale: PI / 180f64 },
                AngleUnit::Minute => Formula::Multiply{ scale: PI / 10_800f64 },
                AngleUnit::Second => Formula::Multiply{ scale: PI / 648_000f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                AngleUnit::Radian => Formula::Divide{ scale: 1f64 },
                AngleUnit::Degree => Formula::Divide{ scale: PI / 180f64 },
                AngleUnit::Minute => Formula::Divide{ scale: PI / 10_800f64 },
                AngleUnit::Second => Formula::Divide{ scale: PI / 648_000f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                AngleUnit::Radian => "radian".to_string(),
                AngleUnit::Degree => "degree".to_string(),
                AngleUnit::Minute => "minute".to_string(),
                AngleUnit::Second => "second".to_string(),
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                AngleUnit::Radian => "radians".to_string(),
                AngleUnit::Degree => "degrees".to_string(),
                AngleUnit::Minute => "minutes".to_string(),
                AngleUnit::Second => "seconds".to_string(),
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                AngleUnit::Radian => "rad".to_string(),
                AngleUnit::Degree => "°".to_string(),
                AngleUnit::Minute => "′".to_string(),
                AngleUnit::Second => "″".to_string(),
            }
        }
    }
}

impl_measurement!(AngleMeasurement, AngleUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(AngleUnit::Radian.to_base_unit(1f64), 1f64);
        assert_eq!(AngleUnit::Degree.to_base_unit(1f64), PI / 180f64);
        assert_eq!(AngleUnit::Minute.to_base_unit(1f64), PI / 10_800f64);
        assert_eq!(AngleUnit::Second.to_base_unit(1f64), PI / 648_000f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(AngleUnit::Radian.from_base_unit(1f64), 1f64);
        assert_eq!(AngleUnit::Degree.from_base_unit(PI / 180f64), 1f64);
        assert_eq!(AngleUnit::Minute.from_base_unit(PI / 10_800f64), 1f64);
        assert_eq!(AngleUnit::Second.from_base_unit(PI / 648_000f64), 1f64);
    }
    
    #[test]
    fn singular_names_are_correct() {
        assert_eq!(AngleUnit::Radian.name_singular(), "radian");
        assert_eq!(AngleUnit::Degree.name_singular(), "degree");
        assert_eq!(AngleUnit::Minute.name_singular(), "minute");
        assert_eq!(AngleUnit::Second.name_singular(), "second");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(AngleUnit::Radian.name_plural(), "radians");
        assert_eq!(AngleUnit::Degree.name_plural(), "degrees");
        assert_eq!(AngleUnit::Minute.name_plural(), "minutes");
        assert_eq!(AngleUnit::Second.name_plural(), "seconds");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(AngleUnit::Radian.symbol(), "rad");
        assert_eq!(AngleUnit::Degree.symbol(), "°");
        assert_eq!(AngleUnit::Minute.symbol(), "′");
        assert_eq!(AngleUnit::Second.symbol(), "″");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = AngleMeasurement::from(PI, Prefix::None, AngleUnit::Radian);
        let test_result = test_measurement.to(Prefix::None, AngleUnit::Degree);

        assert_eq!(test_result, 180f64);
    }

    #[test]
    fn addition_of_angle_measurements_works() {
        let measurement_one = AngleMeasurement::from(1f64, Prefix::None, AngleUnit::Radian);
        let measurement_two = AngleMeasurement::from(2f64, Prefix::None, AngleUnit::Radian);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, AngleUnit::Radian), 3f64);
    }

    #[test]
    fn subtraction_of_angle_measurements_works() {
        let measurement_one = AngleMeasurement::from(1f64, Prefix::None, AngleUnit::Radian);
        let measurement_two = AngleMeasurement::from(2f64, Prefix::None, AngleUnit::Radian);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, AngleUnit::Radian), -1f64);
    }

    #[test]
    fn addition_assign_of_angle_measurements_works() {
        let mut measurement_one = AngleMeasurement::from(1f64, Prefix::None, AngleUnit::Radian);
        let measurement_two = AngleMeasurement::from(2f64, Prefix::None, AngleUnit::Radian);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AngleUnit::Radian), 3f64);
    }

    #[test]
    fn subtraction_assign_of_angle_measurements_works() {
        let mut measurement_one = AngleMeasurement::from(1f64, Prefix::None, AngleUnit::Radian);
        let measurement_two = AngleMeasurement::from(2f64, Prefix::None, AngleUnit::Radian);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AngleUnit::Radian), -1f64);
    }
}