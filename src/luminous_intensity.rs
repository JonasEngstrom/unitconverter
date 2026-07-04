//! # Units and Operations Pertaining to Luminous Intensity
//! 
//! The base unit used to store luminous intensity in the `unitconverter` crate is candela.

use crate::macros::*;

/// # Units of Luminous Intensity
/// 
/// Units for measurement of luminous intensity.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
pub enum LuminousIntensityUnit {
    /// Defined by taking the fixed numerical value of the luminous efficacy of monochromatic radiation of frequency 540 × 10<sup>12</sup> Hz to be 683 when expressed in the unit lm W<sup>-1</sup>. Represented by the symbol cd.<sup>1</sup>
    Candela,
}

impl LuminousIntensityUnit {
    doc_to_base_unit! {
        pub(crate) fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                LuminousIntensityUnit::Candela => |x| x,
            }
        }
    }

    doc_from_base_unit! {
        pub(crate) fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                LuminousIntensityUnit::Candela => |x| x,
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                LuminousIntensityUnit::Candela => "candela"
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> &str {
            match self {
                LuminousIntensityUnit::Candela => "candela"
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> &str {
            match self {
                LuminousIntensityUnit::Candela => "cd"
            }
        }
    }
}

impl_measurement!(LuminousIntensityMeasurement, LuminousIntensityUnit);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(LuminousIntensityUnit::Candela.to_base_unit()(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(LuminousIntensityUnit::Candela.from_base_unit()(1f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(LuminousIntensityUnit::Candela.name_singular(), "candela");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(LuminousIntensityUnit::Candela.name_plural(), "candela");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(LuminousIntensityUnit::Candela.symbol(), "cd");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = LuminousIntensityMeasurement::from(1f64, Prefix::Kilo, LuminousIntensityUnit::Candela);
        let test_result = test_measurement.to(Prefix::None, LuminousIntensityUnit::Candela);

        assert_eq!(test_result, 1_000f64);
    }

    #[test]
    fn addition_of_luminous_intensity_measurements_work() {
        let measurement_one = LuminousIntensityMeasurement::from(2f64, Prefix::None, LuminousIntensityUnit::Candela);
        let measurement_two = LuminousIntensityMeasurement::from(1f64, Prefix::None, LuminousIntensityUnit::Candela);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, LuminousIntensityUnit::Candela), 3f64);
    }

    #[test]
    fn subtraction_of_luminous_intensity_measurements_work() {
        let measurement_one = LuminousIntensityMeasurement::from(2f64, Prefix::None, LuminousIntensityUnit::Candela);
        let measurement_two = LuminousIntensityMeasurement::from(1f64, Prefix::None, LuminousIntensityUnit::Candela);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, LuminousIntensityUnit::Candela), 1f64);
    }

    #[test]
    fn addition_assign_of_luminous_intensity_measurements_work() {
        let mut measurement_one = LuminousIntensityMeasurement::from(2f64, Prefix::None, LuminousIntensityUnit::Candela);
        let measurement_two = LuminousIntensityMeasurement::from(1f64, Prefix::None, LuminousIntensityUnit::Candela);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, LuminousIntensityUnit::Candela), 3f64);
    }

    #[test]
    fn subtraction_assign_of_luminous_intensity_measurements_work() {
        let mut measurement_one = LuminousIntensityMeasurement::from(2f64, Prefix::None, LuminousIntensityUnit::Candela);
        let measurement_two = LuminousIntensityMeasurement::from(1f64, Prefix::None, LuminousIntensityUnit::Candela);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, LuminousIntensityUnit::Candela), 1f64);
    }
}