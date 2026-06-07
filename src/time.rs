//! # Units and Operations Pertaining to Time

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Time
/// 
/// Units for measurement of time.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum TimeUnit {
    /// Defined by taking the unperturbed ground-state hyperfine transition frequency of the cesium-133 atom, to be 9,192,631,770 Hz. Hertz being equal to s<sup>−1</sup>. Represented by the symbol s.<sup>1</sup>
    Second,
    /// Defined as 60 seconds. Represented by the symbol min.<sup>1</sup>
    Minute,
    /// Defined as 3,600 seconds. Equal to 60 minutes. Represented by the symbol h.<sup>1</sup>
    Hour,
    /// Defined as 86,400 seconds. Equal to 24 hours. Represented by the symbol d.<sup>1</sup>
    Day,
}

impl TimeUnit {
    /// Returns the factor fo converting a unit into seconds.
    /// 
    /// ```
    /// use unitconverter::time::TimeUnit;
    /// 
    /// let hour = TimeUnit::Hour;
    /// 
    /// assert_eq!(hour.factor(), 3_600f64);
    /// ```
    pub fn factor(&self) -> f64 {
        match self {
            TimeUnit::Second => 1f64,
            TimeUnit::Minute => 60f64,
            TimeUnit::Hour => 3_600f64,
            TimeUnit::Day => 86_400f64,
        }
    }

    /// Returns the name of a unit in singular.
    /// 
    /// ```
    /// use unitconverter::time::TimeUnit;
    /// 
    /// let minute = TimeUnit::Minute;
    /// 
    /// assert_eq!(minute.name_singular(), "minute");
    /// ```
    pub fn name_singular(&self) -> &str {
        match self {
            TimeUnit::Second => "second",
            TimeUnit::Minute => "minute",
            TimeUnit::Hour => "hour",
            TimeUnit::Day => "day",
        }
    }

    /// Returns the name of a unit in plural.
    /// 
    /// ```
    /// use unitconverter::time::TimeUnit;
    /// 
    /// let day = TimeUnit::Day;
    /// 
    /// assert_eq!(day.name_plural(), "days");
    /// ```
    pub fn name_plural(&self) -> &str {
        match self {
            TimeUnit::Second => "seconds",
            TimeUnit::Minute => "minutes",
            TimeUnit::Hour => "hours",
            TimeUnit::Day => "days",
        }
    }

    /// Returns the symbol of a unit.
    /// 
    /// ```
    /// use unitconverter::time::TimeUnit;
    /// 
    /// let second = TimeUnit::Second;
    /// 
    /// assert_eq!(second.symbol(), "s");
    /// ```
    pub fn symbol(&self) -> &str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "h",
            TimeUnit::Day => "d",
        }
    }
}

/// # Measurement of Time
/// 
/// A measurement of time. Stored internally as seconds, but output as any unit the user desires.
pub struct TimeMeasurement { value: f64 }

impl TimeMeasurement {
    /// # Store a New Measurement of Time
    /// 
    /// Measurements are stored using a value, a prefix, and a unit, as illustrated in the following example:
    /// 
    /// ```
    /// use unitconverter::time::{ TimeUnit, TimeMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_hour = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
    /// 
    /// // Desired output format.
    /// assert_eq!(one_hour.to(Prefix::None, TimeUnit::Second), 3_600f64);
    /// ```
    pub fn from(value: f64, prefix: Prefix, unit: TimeUnit) -> Self {
        Self {
            value: value * Prefix::conversion_constant(prefix, Prefix::None) * unit.factor()
        }
    }

    /// # Convert a Previously Stored Measurement of Time
    /// 
    /// Previously stored `TimeMeasurement`s are converted using a prefix and a unit, as illustrated in the following example:
    /// 
    /// ```
    /// use unitconverter::time::{ TimeUnit, TimeMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_hour = TimeMeasurement::from(1f64, Prefix::None, TimeUnit::Hour);
    /// 
    /// // Desired output format.
    /// assert_eq!(one_hour.to(Prefix::None, TimeUnit::Second), 3_600f64);
    /// ```
    pub fn to(&self, prefix: Prefix, unit: TimeUnit) -> f64 {
        self.value * Prefix::conversion_constant(Prefix::None, prefix) / unit.factor()
    }
}

impl_add_and_subtract!(TimeMeasurement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_are_correct() {
        assert_eq!(TimeUnit::Second.factor(), 1f64);
        assert_eq!(TimeUnit::Minute.factor(), 60f64);
        assert_eq!(TimeUnit::Hour.factor(), 3_600f64);
        assert_eq!(TimeUnit::Day.factor(), 86_400f64);
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