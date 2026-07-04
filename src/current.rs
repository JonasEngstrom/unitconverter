//! # Units and Operations Pertaining to Electric Current
//! 
//! The base unit used to store electric current in the `unitconverter` crate is amperes.

use crate::macros::*;

/// # Units of Electric Current
/// 
/// Units for measurement of electric current.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
pub enum CurrentUnit {
    /// Since the 2019 revision of the SI system defined by taking the fixed numerical value of the elementary charge to be 1.602176634 × 10<sup>-19</sup> when expressed in coulomb, which is equal to ampere-seconds. The second being defined from the unperturbed ground state hyperfine transition frequency of the cesum 133 atom. Represented by the symbol A.<sup>1</sup>
    Ampere,
}

impl CurrentUnit {
    doc_to_base_unit! {
        pub(crate) fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                CurrentUnit::Ampere => |x| x,
            }
        }
    }

    doc_from_base_unit! {
        pub(crate) fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                CurrentUnit::Ampere => |x| x,
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                CurrentUnit::Ampere => "ampere",
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> &str {
            match self {
                CurrentUnit::Ampere => "amperes",
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> &str {
            match self {
                CurrentUnit::Ampere => "A",
            }
        }
    }
}

impl_measurement!(CurrentMeasurement, CurrentUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(CurrentUnit::Ampere.to_base_unit()(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(CurrentUnit::Ampere.from_base_unit()(1f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(CurrentUnit::Ampere.name_singular(), "ampere");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(CurrentUnit::Ampere.name_plural(), "amperes");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(CurrentUnit::Ampere.symbol(), "A");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = CurrentMeasurement::from(100f64, Prefix::None, CurrentUnit::Ampere);
        let test_result = test_measurement.to(Prefix::Hecto, CurrentUnit::Ampere);

        assert_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_current_measurements_work() {
        let measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, CurrentUnit::Ampere), 0.18f64);
    }

    #[test]
    fn subtraction_of_current_measurements_work() {
        let measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, CurrentUnit::Ampere), 0.06f64);
    }

    #[test]
    fn addition_assign_of_current_measurements_work() {
        let mut measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CurrentUnit::Ampere), 0.18f64);
    }

    #[test]
    fn subtraction_assign_of_current_measurements_work() {
        let mut measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CurrentUnit::Ampere), 0.06f64);
    }
}