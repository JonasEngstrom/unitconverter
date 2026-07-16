//! # Units and Operations Pertaining to Count
//! 
//! The base unit used to store count in the `unitconverter` crate is one.

use crate::macros::*;
use crate::formulas::*;

/// # Units of Count
/// 
/// Units for measurement of count.
/// 
/// ## References
/// 
/// 1. [Dozen. In: American Heritage® Dictionary of the English Language. 5th ed. Houghton Mifflin Harcourt Publishing Company; 2011.](https://www.thefreedictionary.com/dozen)
/// 2. [Lakh. In: Wikipedia. 2026.](https://en.wikipedia.org/wiki/Lakh)
/// 3. [Crore. In: Wikipedia. 2026.](https://en.wikipedia.org/wiki/Crore)
/// 4. [Myriad. In: Wikipedia. 2026.](https://en.wikipedia.org/wiki/Myriad)
/// 5. [Score. In: American Heritage® Dictionary of the English Language. 5th ed. Houghton Mifflin Harcourt Publishing Company; 2011.](https://www.thefreedictionary.com/score)
pub enum CountUnit {
    /// A count of one. Has no unit. Represented by the symbol × 1.
    One,
    /// A count of twelve. Representeted by the symbol dz in the `unitconverter` crate, but can also be represented by the symbol doz.<sup>1</sup>
    Dozen,
    /// A count of one hundred thousand. Represented by the symbol L.<sup>2</sup>
    Lakh,
    /// A count of one hundred lakh, which is equal to ten million. Represented by the symbol cr.<sup>3</sup>
    Crore,
    /// A count of ten thousand. Represented by the symbol × 10,000.
    Myriad,
    /// A count of twenty. Represented by the symbol × 20.
    Score,
}

impl CountUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                CountUnit::One => Formula::Multiply{ scale: 1f64 },
                CountUnit::Dozen => Formula::Multiply{ scale: 12f64 },
                CountUnit::Lakh => Formula::Multiply{ scale: 1e5f64 },
                CountUnit::Crore => Formula::Multiply{ scale: 1e7f64 },
                CountUnit::Myriad => Formula::Multiply{ scale: 1e4f64 },
                CountUnit::Score => Formula::Multiply{ scale: 20f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                CountUnit::One => Formula::Divide{ scale: 1f64 },
                CountUnit::Dozen => Formula::Divide{ scale: 12f64 },
                CountUnit::Lakh => Formula::Divide{ scale: 1e5f64 },
                CountUnit::Crore => Formula::Divide{ scale: 1e7f64 },
                CountUnit::Myriad => Formula::Divide{ scale: 1e4f64 },
                CountUnit::Score => Formula::Divide{ scale: 20f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                CountUnit::One => "".to_string(),
                CountUnit::Dozen => "dozen".to_string(),
                CountUnit::Lakh => "lakh".to_string(),
                CountUnit::Crore => "crore".to_string(),
                CountUnit::Myriad => "myriad".to_string(),
                CountUnit::Score => "score".to_string(),
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                CountUnit::One => "".to_string(),
                CountUnit::Dozen => "dozen".to_string(),
                CountUnit::Lakh => "lakh".to_string(),
                CountUnit::Crore => "crore".to_string(),
                CountUnit::Myriad => "myriads".to_string(),
                CountUnit::Score => "scores".to_string(),
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                CountUnit::One => "× 1".to_string(),
                CountUnit::Dozen => "doz".to_string(),
                CountUnit::Lakh => "L".to_string(),
                CountUnit::Crore => "cr".to_string(),
                CountUnit::Myriad => "× 10,000".to_string(),
                CountUnit::Score => "× 20".to_string(),
            }
        }
    }
}

impl_measurement!(CountMeasurement, CountUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(CountUnit::One.to_base_unit(1f64), 1f64);
        assert_eq!(CountUnit::Dozen.to_base_unit(1f64), 12f64);
        assert_eq!(CountUnit::Lakh.to_base_unit(1f64), 1e5f64);
        assert_eq!(CountUnit::Crore.to_base_unit(1f64), 1e7f64);
        assert_eq!(CountUnit::Myriad.to_base_unit(1f64), 1e4f64);
        assert_eq!(CountUnit::Score.to_base_unit(1f64), 20f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(CountUnit::One.from_base_unit(1f64), 1f64);
        assert_eq!(CountUnit::Dozen.from_base_unit(12f64), 1f64);
        assert_eq!(CountUnit::Lakh.from_base_unit(1e5f64), 1f64);
        assert_eq!(CountUnit::Crore.from_base_unit(1e7f64), 1f64);
        assert_eq!(CountUnit::Myriad.from_base_unit(1e4f64), 1f64);
        assert_eq!(CountUnit::Score.from_base_unit(20f64), 1f64);
    }
    
    #[test]
    fn singular_names_are_correct() {
        assert_eq!(CountUnit::One.name_singular(), "");
        assert_eq!(CountUnit::Dozen.name_singular(), "dozen");
        assert_eq!(CountUnit::Lakh.name_singular(), "lakh");
        assert_eq!(CountUnit::Crore.name_singular(), "crore");
        assert_eq!(CountUnit::Myriad.name_singular(), "myriad");
        assert_eq!(CountUnit::Score.name_singular(), "score");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(CountUnit::One.name_plural(), "");
        assert_eq!(CountUnit::Dozen.name_plural(), "dozen");
        assert_eq!(CountUnit::Lakh.name_plural(), "lakh");
        assert_eq!(CountUnit::Crore.name_plural(), "crore");
        assert_eq!(CountUnit::Myriad.name_plural(), "myriads");
        assert_eq!(CountUnit::Score.name_plural(), "scores");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(CountUnit::One.symbol(), "× 1");
        assert_eq!(CountUnit::Dozen.symbol(), "doz");
        assert_eq!(CountUnit::Lakh.symbol(), "L");
        assert_eq!(CountUnit::Crore.symbol(), "cr");
        assert_eq!(CountUnit::Myriad.symbol(), "× 10,000");
        assert_eq!(CountUnit::Score.symbol(), "× 20");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = CountMeasurement::from(1f64, Prefix::None, CountUnit::Score);
        let test_result = test_measurement.to(Prefix::None, CountUnit::One);

        assert_eq!(test_result, 20f64);
    }

    #[test]
    fn addition_of_count_measurements_works() {
        let measurement_one = CountMeasurement::from(1f64, Prefix::None, CountUnit::Lakh);
        let measurement_two = CountMeasurement::from(1f64, Prefix::None, CountUnit::Crore);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, CountUnit::One), 10_100_000f64);
    }

    #[test]
    fn subtraction_of_count_measurements_works() {
        let measurement_one = CountMeasurement::from(1f64, Prefix::None, CountUnit::Lakh);
        let measurement_two = CountMeasurement::from(1f64, Prefix::None, CountUnit::Crore);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, CountUnit::One), -9_900_000f64);
    }

    #[test]
    fn addition_assign_of_count_measurements_works() {
        let mut measurement_one = CountMeasurement::from(1f64, Prefix::None, CountUnit::Lakh);
        let measurement_two = CountMeasurement::from(1f64, Prefix::None, CountUnit::Crore);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CountUnit::One), 10_100_000f64);
    }

    #[test]
    fn subtraction_assign_of_count_measurements_works() {
        let mut measurement_one = CountMeasurement::from(1f64, Prefix::None, CountUnit::Lakh);
        let measurement_two = CountMeasurement::from(1f64, Prefix::None, CountUnit::Crore);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CountUnit::One), -9_900_000f64);
    }
}