//! # Units and Operations Pertaining to Mass
//! 
//! The base unit used to store mass in the `unitconverter` crate is kilograms.

use crate::macros::*;
use crate::formulas::*;

/// # Units of Mass
/// 
/// Units for measurement of mass.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
/// 2. [Astin AV, Arnold Karo H, Mueller FH. Refinement of values for the yard and the pound. Gaithersburg (MD): National Bureau of Standards; 1959 Jun. Report F.R. Doc. 59-5442.](https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf)
/// 3. [Codes for units of measure used in international trade, annex C. units of measure: code elements listed by name. Geneva (Switzerland): United Nations Economic Commission for Europe; 1996 Jun. Report TRADE/WP.4/R.1224/Add.2.](https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf)
pub enum MassUnit {
    /// Since the 2019 revision of the SI system defined as one thousandth of a kilogram, which is in turn defined by taking the fixed numerical value of the Planck constant to be 6.62607015 × 10<sup>-34</sup> when expressed in Joule-seconds, which is equal to kilogram-squaremeters per second. The meter and the second being defined from the speed of light in vacuum and the unperturbed ground state hyperfine transition frequency of the cesum 133 atom. Represented by the symbol g.<sup>1</sup>
    Gram,
    /// Defined as 0.45359237 kilograms, since the adoption of the international avoirdupois pound in 1959.<sup>2</sup> Represented by the symbol lb.<sup>3</sup>
    Pound,
}

impl MassUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                MassUnit::Gram => Formula::Multiply{ scale: 0.001f64 },
                MassUnit::Pound => Formula::Multiply{ scale: 0.453_592_37f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                MassUnit::Gram => Formula::Divide{ scale: 0.001f64 },
                MassUnit::Pound => Formula::Divide{ scale: 0.453_592_37f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                MassUnit::Gram => "gram",
                MassUnit::Pound => "pound",
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> &str {
            match self {
                MassUnit::Gram => "grams",
                MassUnit::Pound => "pounds"
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> &str {
            match self {
                MassUnit::Gram => "g",
                MassUnit::Pound => "lb",
            }
        }
    }
}

impl_measurement!(MassMeasurement, MassUnit);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(MassUnit::Gram.to_base_unit(1f64), 0.001f64);
        assert_eq!(MassUnit::Pound.to_base_unit(1f64), 0.453_592_37f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(MassUnit::Gram.from_base_unit(0.001f64), 1f64);
        assert_eq!(MassUnit::Pound.from_base_unit(0.453_592_37f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(MassUnit::Gram.name_singular(), "gram");
        assert_eq!(MassUnit::Pound.name_singular(), "pound");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(MassUnit::Gram.name_plural(), "grams");
        assert_eq!(MassUnit::Pound.name_plural(), "pounds");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(MassUnit::Gram.symbol(), "g");
        assert_eq!(MassUnit::Pound.symbol(), "lb");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = MassMeasurement::from(0.453_592_37f64, Prefix::Kilo, MassUnit::Gram);
        let test_result = test_measurement.to(Prefix::None, MassUnit::Pound);

        assert_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_mass_measurements_work() {
        let measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, MassUnit::Pound), 3f64);
    }

    #[test]
    fn subtraction_of_mass_measurements_work() {
        let measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, MassUnit::Pound), 1f64);
    }

    #[test]
    fn addition_assign_of_mass_measurements_work() {
        let mut measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, MassUnit::Pound), 3f64);
    }

    #[test]
    fn subtraction_assign_of_mass_measurements_work() {
        let mut measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, MassUnit::Pound), 1f64);
    }
}