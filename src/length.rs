//! # Units and Operations Pertaining to Length

use si_prefixes::Prefix;

/// # Units of Length
/// 
/// Units for measurment of length.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
/// 2. National Bureau of Standards. (1959) *Refinement of Values for the Yard and the Pound*. [https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf](https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf)
pub enum LengthUnit {
    /// Defined as the length of the path travelled by light in vacuum during 1 / 299,792,458 seconds, since the 2019 revision of the SI system.<sup>1</sup>
    Meter,
    /// Defined as 0.0254 meters, since the adoption of the international yard in 1959.<sup>2</sup>
    Inch,
    /// Defined as 0.3048 meters, since the adoption of the international yard in 1959. Equal to 12 inches.<sup>2</sup>
    Foot,
    /// Defined as 0.9144 meters, since the adoption of the international yard in 1959. Equal to 3 feet.<sup>2</sup>
    Yard,
    /// Defined as 1609.344 meters, since the adoption of the international yard in 1959. Equal to 1,760 yards.<sup>2</sup>
    Mile,
}

impl LengthUnit {
    /// Returns the factor for converting a unit into meters.
    pub fn factor(&self) -> f64 {
        match self {
            LengthUnit::Meter => 1f64,
            LengthUnit::Inch => 0.0254f64,
            LengthUnit::Foot => 0.3048f64,
            LengthUnit::Yard => 0.9144f64,
            LengthUnit::Mile => 1_609.344f64,
        }
    }

    /// Returns the name of a unit in singular.
    pub fn name_singular(&self) -> &str {
        match self {
            LengthUnit::Meter => "meter",
            LengthUnit::Inch => "inch",
            LengthUnit::Foot => "foot",
            LengthUnit::Yard => "yard",
            LengthUnit::Mile => "mile",
        }
    }

    /// Returns the name of a unit in plural.
    pub fn name_plural(&self) -> &str {
        match self {
            LengthUnit::Meter => "meters",
            LengthUnit::Inch => "inches",
            LengthUnit::Foot => "feet",
            LengthUnit::Yard => "yards",
            LengthUnit::Mile => "miles",
        }
    }
}

/// # Measurement of Length
/// 
/// A mesurment of length. Stored internally as meters, but output to any unit the user desires.
pub struct LengthMeasurement { value: f64 }

impl LengthMeasurement {
    /// # Store a New Measurement of Length
    /// 
    /// Measurements are stored using a value, a prefix, and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::length::{ LengthUnit, LengthMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let twelve_centimeters = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
    /// let one_inch = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Inch);
    /// 
    /// // Desired output format.
    /// assert_eq!(twelve_centimeters.to(Prefix::Centi, LengthUnit::Meter), 12f64);
    /// assert_eq!(one_inch.to(Prefix::None, LengthUnit::Inch), 1f64);
    /// ```
    pub fn from(value: f64, prefix: Prefix, unit: LengthUnit) -> Self {
        Self {
            value: value * Prefix::conversion_constant(prefix, Prefix::None) * unit.factor()
        }
    }

    /// # Convert a Previously Stored Measurement of Length
    /// 
    /// Previously stored `LengthMeasurement`s are converted using a prefix, and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::length::{ LengthUnit, LengthMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let twelve_centimeters = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
    /// let one_inch = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Inch);
    /// 
    /// // Desired output format.
    /// assert_eq!(twelve_centimeters.to(Prefix::Centi, LengthUnit::Meter), 12f64);
    /// assert_eq!(one_inch.to(Prefix::None, LengthUnit::Inch), 1f64);
    /// ```
    pub fn to(&self, prefix: Prefix, unit: LengthUnit) -> f64 {
        self.value * Prefix::conversion_constant(Prefix::None, prefix) / unit.factor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-15f64;

    #[test]
    fn factors_are_correct() {
        assert_eq!(LengthUnit::Meter.factor(), 1f64);
        assert_eq!(LengthUnit::Inch.factor(), 0.0254f64);
        assert_eq!(LengthUnit::Foot.factor(), 0.3048f64);
        assert_eq!(LengthUnit::Yard.factor(), 0.9144f64);
        assert_eq!(LengthUnit::Mile.factor(), 1_609.344f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(LengthUnit::Meter.name_singular(), "meter");
        assert_eq!(LengthUnit::Inch.name_singular(), "inch");
        assert_eq!(LengthUnit::Foot.name_singular(), "foot");
        assert_eq!(LengthUnit::Yard.name_singular(), "yard");
        assert_eq!(LengthUnit::Mile.name_singular(), "mile");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(LengthUnit::Meter.name_plural(), "meters");
        assert_eq!(LengthUnit::Inch.name_plural(), "inches");
        assert_eq!(LengthUnit::Foot.name_plural(), "feet");
        assert_eq!(LengthUnit::Yard.name_plural(), "yards");
        assert_eq!(LengthUnit::Mile.name_plural(), "miles");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = LengthMeasurement::from(12f64, Prefix::None, LengthUnit::Inch);
        let test_result = test_measurement.to(Prefix::None, LengthUnit::Foot);
        assert!((test_result-1.0f64).abs() < EPSILON);
    }
}