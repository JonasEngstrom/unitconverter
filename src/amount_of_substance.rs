//! # Units and Operations Pertaining to Amount of Substance
//! 
//! The base unit used to store amount of substance in the `unitconverter` crate is moles.

use crate::macros::*;

/// # Units of Amount of Subtance
/// 
/// Units for measurement of amount of substance.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum AmountOfSubstanceUnit {
    /// Defined as 6.02214076 × 10<sup>23</sup> elementary entities. Represented by the symbol mol.<sup>1</sup>
    Mole,
}

impl AmountOfSubstanceUnit {
    doc_to_base_unit! {
        fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AmountOfSubstanceUnit::Mole => |x| x,
            }
        }
    }

    doc_from_base_unit! {
        fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AmountOfSubstanceUnit::Mole => |x| x,
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                AmountOfSubstanceUnit::Mole => "mole"
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> &str {
            match self {
                AmountOfSubstanceUnit::Mole => "moles"
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> &str {
            match self {
                AmountOfSubstanceUnit::Mole => "mol"
            }
        }
    }
}

impl_measurement!(AmountOfSubstanceMeasurement, AmountOfSubstanceUnit);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(AmountOfSubstanceUnit::Mole.to_base_unit()(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(AmountOfSubstanceUnit::Mole.from_base_unit()(1f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(AmountOfSubstanceUnit::Mole.name_singular(), "mole");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(AmountOfSubstanceUnit::Mole.name_plural(), "moles");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(AmountOfSubstanceUnit::Mole.symbol(), "mol");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = AmountOfSubstanceMeasurement::from(1f64, Prefix::Kilo, AmountOfSubstanceUnit::Mole);
        let test_result = test_measurement.to(Prefix::None, AmountOfSubstanceUnit::Mole);

        assert_eq!(test_result, 1_000f64);
    }

    #[test]
    fn addition_of_amount_of_substance_measuremens_work() {
        let measurement_one = AmountOfSubstanceMeasurement::from(2f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let measurement_two = AmountOfSubstanceMeasurement::from(1f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, AmountOfSubstanceUnit::Mole), 3f64);
    }

    #[test]
    fn subtraction_of_amount_of_substance_measuremens_work() {
        let measurement_one = AmountOfSubstanceMeasurement::from(2f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let measurement_two = AmountOfSubstanceMeasurement::from(1f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, AmountOfSubstanceUnit::Mole), 1f64);
    }

    #[test]
    fn addition_assign_of_amount_of_substance_measuremens_work() {
        let mut measurement_one = AmountOfSubstanceMeasurement::from(2f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let measurement_two = AmountOfSubstanceMeasurement::from(1f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AmountOfSubstanceUnit::Mole), 3f64);
    }

    #[test]
    fn subtraction_assign_of_amount_of_substance_measuremens_work() {
        let mut measurement_one = AmountOfSubstanceMeasurement::from(2f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        let measurement_two = AmountOfSubstanceMeasurement::from(1f64, Prefix::None, AmountOfSubstanceUnit::Mole);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, AmountOfSubstanceUnit::Mole), 1f64);
    }
}