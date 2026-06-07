//! # Units and Operations Pertaining to Length

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Length
/// 
/// Units for measurement of length.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
/// 2. National Bureau of Standards. (1959) *Refinement of Values for the Yard and the Pound*. [https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf](https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf)
/// 3. United Nations Economic Commission for Europe. (1995) *Codes for Units of Measure Used in International Trade*. [https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf](https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf)
/// 4. National Institute of Standards and Technology. (2014) *Specifications, Tolerances, and Other Technical Requirements for Weighing and Measuring Devices*. [https://web.archive.org/web/20160730042942/http://www.nist.gov/pml/wmd/pubs/upload/hb44-14-final-web.pdf](https://web.archive.org/web/20160730042942/http://www.nist.gov/pml/wmd/pubs/upload/hb44-14-final-web.pdf)
pub enum LengthUnit {
    /// Defined as the length of the path travelled by light in vacuum during 1 / 299,792,458 seconds, since the 2019 revision of the SI system. Represented by the symbol m.<sup>1</sup>
    Meter,
    /// Defined as 0.0254 meters, since the adoption of the international yard in 1959.<sup>2</sup> Represented by the symbol in.<sup>3</sup>
    Inch,
    /// Defined as 0.3048 meters, since the adoption of the international yard in 1959. Equal to 12 inches.<sup>2</sup> Represented by the symbol ft.<sup>3</sup>
    Foot,
    /// Defined as 0.9144 meters, since the adoption of the international yard in 1959. Equal to 3 feet.<sup>2</sup> Represented by the symbol yd.<sup>3</sup>
    Yard,
    /// Defined as 1609.344 meters, since the adoption of the international yard in 1959. Equal to 1,760 yards.<sup>2</sup> Represented by the symbol mi.<sup>4</sup>
    Mile,
}

impl LengthUnit {
    /// Returns the factor for converting a unit into meters.
    /// 
    /// ```
    /// use unitconverter::length::LengthUnit;
    /// 
    /// let yard = LengthUnit::Yard;
    /// 
    /// assert_eq!(yard.factor(), 0.9144f64);
    /// ```
    pub fn factor(&self) -> f64 {
        match self {
            LengthUnit::Meter => 1f64,
            LengthUnit::Inch => 0.025_4f64,
            LengthUnit::Foot => 0.304_8f64,
            LengthUnit::Yard => 0.914_4f64,
            LengthUnit::Mile => 1_609.344f64,
        }
    }

    /// Returns the name of a unit in singular.
    /// 
    /// ```
    /// use unitconverter::length::LengthUnit;
    /// 
    /// let mile = LengthUnit::Mile;
    /// 
    /// assert_eq!(mile.name_singular(), "mile");
    /// ```
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
    /// 
    /// ```
    /// use unitconverter::length::LengthUnit;
    /// 
    /// let foot = LengthUnit::Foot;
    /// 
    /// assert_eq!(foot.name_plural(), "feet");
    /// ```
    pub fn name_plural(&self) -> &str {
        match self {
            LengthUnit::Meter => "meters",
            LengthUnit::Inch => "inches",
            LengthUnit::Foot => "feet",
            LengthUnit::Yard => "yards",
            LengthUnit::Mile => "miles",
        }
    }

    /// Returns the symbol of a unit.
    /// 
    /// ```
    /// use unitconverter::length::LengthUnit;
    /// 
    /// let meter = LengthUnit::Meter;
    /// 
    /// assert_eq!(meter.symbol(), "m");
    /// ```
    pub fn symbol(&self) -> &str {
        match self {
            LengthUnit::Meter => "m",
            LengthUnit::Inch => "in",
            LengthUnit::Foot => "ft",
            LengthUnit::Yard => "yd",
            LengthUnit::Mile => "mi",
        }
    }
}

/// # Measurement of Length
/// 
/// A measurment of length. Stored internally as meters, but output as any unit the user desires.
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
    /// Previously stored `LengthMeasurement`s are converted using a prefix and a unit, as illustrated in the following examples:
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

impl_add_and_subtract!(LengthMeasurement);

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::macros::assert_almost_eq;

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
    fn symbols_are_correct() {
        assert_eq!(LengthUnit::Meter.symbol(), "m");
        assert_eq!(LengthUnit::Inch.symbol(), "in");
        assert_eq!(LengthUnit::Foot.symbol(), "ft");
        assert_eq!(LengthUnit::Yard.symbol(), "yd");
        assert_eq!(LengthUnit::Mile.symbol(), "mi");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = LengthMeasurement::from(12f64, Prefix::None, LengthUnit::Inch);
        let test_result = test_measurement.to(Prefix::None, LengthUnit::Foot);

        assert_almost_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_length_measuremens_work() {
        let measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::Centi, LengthUnit::Meter), 18f64);
    }

    #[test]
    fn subtraction_of_length_measuremens_work() {
        let measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::Centi, LengthUnit::Meter), 6f64);
    }

    #[test]
    fn addition_assign_of_length_measuremens_work() {
        let mut measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::Centi, LengthUnit::Meter), 18f64);
    }

    #[test]
    fn subtraction_assign_of_length_measuremens_work() {
        let mut measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::Centi, LengthUnit::Meter), 6f64);
    }
}