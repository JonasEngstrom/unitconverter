//! # Units and Operations Pertaining to Frequency
//!
//! The base unit used to store frequency in the `unitconverter` crate is hertz.

use crate::formulas::*;
use crate::macros::*;

/// # Units of Frequency
///
/// Units for measurement of frequency.
///
/// ## References
///
/// 1. [Le Système international d'unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
pub enum FrequencyUnit {
    /// Defined as s<sup>-1</sup>.<sup>1</sup> Represented by the symbol Hz.<sup>1</sup>
    Hertz,
}

impl FrequencyUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                FrequencyUnit::Hertz => Formula::Multiply{ scale: 1f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                FrequencyUnit::Hertz => Formula::Divide{ scale: 1f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                FrequencyUnit::Hertz => "hertz".to_string(),
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                FrequencyUnit::Hertz => "hertz".to_string(),
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                FrequencyUnit::Hertz => "Hz".to_string(),
            }
        }
    }
}

impl_measurement!(FrequencyMeasurement, FrequencyUnit);
impl_multiplication_and_division!(FrequencyMeasurement, crate::time::TimeMeasurement, crate::count::CountMeasurement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(FrequencyUnit::Hertz.to_base_unit(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(FrequencyUnit::Hertz.from_base_unit(1f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(FrequencyUnit::Hertz.name_singular(), "hertz");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(FrequencyUnit::Hertz.name_plural(), "hertz");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(FrequencyUnit::Hertz.symbol(), "Hz");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = FrequencyMeasurement::from(1f64, Prefix::Kilo, FrequencyUnit::Hertz);
        let test_result = test_measurement.to(Prefix::None, FrequencyUnit::Hertz);

        assert_eq!(test_result, 1_000f64);
    }

    #[test]
    fn addition_of_frequency_measurements_work() {
        let measurement_one = FrequencyMeasurement::from(2f64, Prefix::None, FrequencyUnit::Hertz);
        let measurement_two = FrequencyMeasurement::from(1f64, Prefix::None, FrequencyUnit::Hertz);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, FrequencyUnit::Hertz), 3f64);
    }

    #[test]
    fn subtraction_of_frequency_measurements_work() {
        let measurement_one = FrequencyMeasurement::from(2f64, Prefix::None, FrequencyUnit::Hertz);
        let measurement_two = FrequencyMeasurement::from(1f64, Prefix::None, FrequencyUnit::Hertz);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, FrequencyUnit::Hertz), 1f64);
    }

    #[test]
    fn addition_assign_of_frequency_measurements_work() {
        let mut measurement_one = FrequencyMeasurement::from(2f64, Prefix::None, FrequencyUnit::Hertz);
        let measurement_two = FrequencyMeasurement::from(1f64, Prefix::None, FrequencyUnit::Hertz);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, FrequencyUnit::Hertz), 3f64);
    }

    #[test]
    fn subtraction_assign_of_frequency_measurements_work() {
        let mut measurement_one = FrequencyMeasurement::from(2f64, Prefix::None, FrequencyUnit::Hertz);
        let measurement_two = FrequencyMeasurement::from(1f64, Prefix::None, FrequencyUnit::Hertz);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, FrequencyUnit::Hertz), 1f64);
    }
}
