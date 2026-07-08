//! # Units and Operations Pertaining to Time
//! 
//! The base unit used to store temperature in the `unitconverter` crate is seconds.

use crate::macros::*;
use crate::formulas::*;

/// # Units of Time
/// 
/// Units for measurement of time.
/// 
/// ## References
/// 
/// 1. Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291
pub enum TimeUnit {
    /// Defined as 86,400 seconds. Equal to 24 hours. Represented by the symbol d.<sup>1</sup>
    Day,
    /// Defined as 3,600 seconds. Equal to 60 minutes. Represented by the symbol h.<sup>1</sup>
    Hour,
    /// Defined as 60 seconds. Represented by the symbol min.<sup>1</sup>
    Minute,
    /// Defined by taking the unperturbed ground-state hyperfine transition frequency of the cesium-133 atom, to be 9,192,631,770 Hz. Hertz being equal to s<sup>−1</sup>. Represented by the symbol s.<sup>1</sup>
    Second,
}

impl TimeUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                TimeUnit::Second => Formula::Multiply{ scale: 1f64 },
                TimeUnit::Minute => Formula::Multiply{ scale: 60f64 },
                TimeUnit::Hour => Formula::Multiply{ scale: 3_600f64 },
                TimeUnit::Day => Formula::Multiply{ scale: 86_400f64 },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                TimeUnit::Second => Formula::Divide{ scale: 1f64 },
                TimeUnit::Minute => Formula::Divide{ scale: 60f64 },
                TimeUnit::Hour => Formula::Divide{ scale: 3_600f64 },
                TimeUnit::Day => Formula::Divide{ scale: 86_400f64 },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                TimeUnit::Second => "second".to_string(),
                TimeUnit::Minute => "minute".to_string(),
                TimeUnit::Hour => "hour".to_string(),
                TimeUnit::Day => "day".to_string(),
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                TimeUnit::Second => "seconds".to_string(),
                TimeUnit::Minute => "minutes".to_string(),
                TimeUnit::Hour => "hours".to_string(),
                TimeUnit::Day => "days".to_string(),
            }
        }
    }

    doc_symbol! {
        pub fn symbol(&self) -> String {
            match self {
                TimeUnit::Second => "s".to_string(),
                TimeUnit::Minute => "min".to_string(),
                TimeUnit::Hour => "h".to_string(),
                TimeUnit::Day => "d".to_string(),
            }
        }
    }
}

impl_measurement!(TimeMeasurement, TimeUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(TimeUnit::Second.to_base_unit(1f64), 1f64);
        assert_eq!(TimeUnit::Minute.to_base_unit(1f64), 60f64);
        assert_eq!(TimeUnit::Hour.to_base_unit(1f64), 3_600f64);
        assert_eq!(TimeUnit::Day.to_base_unit(1f64), 86_400f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(TimeUnit::Second.from_base_unit(1f64), 1f64);
        assert_eq!(TimeUnit::Minute.from_base_unit(60f64), 1f64);
        assert_eq!(TimeUnit::Hour.from_base_unit(3_600f64), 1f64);
        assert_eq!(TimeUnit::Day.from_base_unit(86_400f64), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(TimeUnit::Second.name_singular(),"second");
        assert_eq!(TimeUnit::Minute.name_singular(),"minute");
        assert_eq!(TimeUnit::Hour.name_singular(),"hour");
        assert_eq!(TimeUnit::Day.name_singular(),"day");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(TimeUnit::Second.name_plural(),"seconds");
        assert_eq!(TimeUnit::Minute.name_plural(),"minutes");
        assert_eq!(TimeUnit::Hour.name_plural(),"hours");
        assert_eq!(TimeUnit::Day.name_plural(),"days");
    }

    #[test]
    fn symbol_are_correct() {
        assert_eq!(TimeUnit::Second.symbol(),"s");
        assert_eq!(TimeUnit::Minute.symbol(),"min");
        assert_eq!(TimeUnit::Hour.symbol(),"h");
        assert_eq!(TimeUnit::Day.symbol(),"d");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = TimeMeasurement::from(3_600f64, Prefix::None, TimeUnit::Second);
        let test_result = test_measurement.to(Prefix::None, TimeUnit::Hour);

        assert_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_time_measurements_work() {
        let measurement_one = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
        let measurement_two = TimeMeasurement::from(30f64, Prefix::None, TimeUnit::Minute);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, TimeUnit::Second), 5_400f64);
    }

    #[test]
    fn subtraction_of_time_measurements_work() {
        let measurement_one = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
        let measurement_two = TimeMeasurement::from(30f64, Prefix::None, TimeUnit::Minute);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, TimeUnit::Second), 1_800f64);
    }

    #[test]
    fn addition_assign_of_time_measurements_work() {
        let mut measurement_one = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
        let measurement_two = TimeMeasurement::from(30f64, Prefix::None, TimeUnit::Minute);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, TimeUnit::Second), 5_400f64);
    }

    #[test]
    fn subtraction_assign_of_time_measurements_work() {
        let mut measurement_one = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
        let measurement_two = TimeMeasurement::from(30f64, Prefix::None, TimeUnit::Minute);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, TimeUnit::Second), 1_800f64);
    }
}