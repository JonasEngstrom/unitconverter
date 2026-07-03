//! # Units and Operations Pertaining to Length
//! 
//! The base unit used to store lengthe in the `unitconverter` is meters.

use crate::macros::*;

/// # Units of Length
/// 
/// Units for measurement of length.
/// 
/// ## References
/// 
/// 1. [Astin AV, Arnold Karo H, Mueller FH. Refinement of values for the yard and the pound. Gaithersburg (MD): National Bureau of Standards; 1959 Jun. Report F.R. Doc. 59-5442.](https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf)
/// 2. [Codes for units of measure used in international trade, annex C. units of measure: code elements listed by name. Geneva (Switzerland): United Nations Economic Commission for Europe; 1996 Jun. Report TRADE/WP.4/R.1224/Add.2.](https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf)
/// 3. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
/// 4. [Chavez Baucom I, Benham EJ, Konijnenburg J, Lee GD, Lippa KA, McGuire JT, et al. Specifications, tolerances, and other technical requirements for weighing and measuring devices—as adopted by the 110th National Council on Weights and Measures. Gaithersburg (MD): National Institute of Standards and Technology; 2026 Jan. Report NIST HB 44-2026. doi:10.6028/NIST.HB.44-2026](https://www.doi.org/10.6028/NIST.HB.44-2026)
pub enum LengthUnit {
    /// Defined as 0.3048 meters, since the adoption of the international yard in 1959. Equal to 12 inches.<sup>1</sup> Represented by the symbol ft.<sup>2</sup>
    Foot,
    /// Defined as 0.0254 meters, since the adoption of the international yard in 1959.<sup>1</sup> Represented by the symbol in.<sup>2</sup>
    Inch,
    /// Defined as the length of the path travelled by light in vacuum during 1 / 299,792,458 seconds, since the 2019 revision of the SI system. Represented by the symbol m.<sup>3</sup>
    Meter,
    /// Defined as 1609.344 meters, since the adoption of the international yard in 1959. Equal to 1,760 yards.<sup>1</sup> Represented by the symbol mi.<sup>4</sup>
    Mile,
    /// Defined as 0.9144 meters, since the adoption of the international yard in 1959. Equal to 3 feet.<sup>1</sup> Represented by the symbol yd.<sup>2</sup>
    Yard,
}

impl LengthUnit {
    doc_to_base_unit! {
        pub fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                LengthUnit::Meter => |x| x,
                LengthUnit::Inch => |x| 0.025_4f64 * x,
                LengthUnit::Foot => |x| 0.304_8f64 * x,
                LengthUnit::Yard => |x| 0.914_4f64 * x,
                LengthUnit::Mile => |x| 1_609.344f64 * x,
            }
        }
    }

    doc_from_base_unit! {
        pub fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                LengthUnit::Meter => |x| x,
                LengthUnit::Inch => |x| x / 0.025_4f64,
                LengthUnit::Foot => |x| x / 0.304_8f64,
                LengthUnit::Yard => |x| x / 0.914_4f64,
                LengthUnit::Mile => |x| x / 1_609.344f64,
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                LengthUnit::Meter => "meter",
                LengthUnit::Inch => "inch",
                LengthUnit::Foot => "foot",
                LengthUnit::Yard => "yard",
                LengthUnit::Mile => "mile",
            }
        }
    }

    doc_name_plural! {
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

    doc_symbol! {
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
}

impl_measurement!(LengthMeasurement, LengthUnit);

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::macros::assert_almost_eq;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(LengthUnit::Meter.to_base_unit()(1f64), 1f64);
        assert_eq!(LengthUnit::Inch.to_base_unit()(1f64), 0.0254f64);
        assert_eq!(LengthUnit::Foot.to_base_unit()(1f64), 0.3048f64);
        assert_eq!(LengthUnit::Yard.to_base_unit()(1f64), 0.9144f64);
        assert_eq!(LengthUnit::Mile.to_base_unit()(1f64), 1_609.344f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(LengthUnit::Meter.from_base_unit()(1f64), 1f64);
        assert_eq!(LengthUnit::Inch.from_base_unit()(0.0254f64), 1f64);
        assert_eq!(LengthUnit::Foot.from_base_unit()(0.3048f64), 1f64);
        assert_eq!(LengthUnit::Yard.from_base_unit()(0.9144f64), 1f64);
        assert_eq!(LengthUnit::Mile.from_base_unit()(1_609.344f64), 1f64);
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
    fn addition_of_length_measurements_work() {
        let measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::Centi, LengthUnit::Meter), 18f64);
    }

    #[test]
    fn subtraction_of_length_measurements_work() {
        let measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::Centi, LengthUnit::Meter), 6f64);
    }

    #[test]
    fn addition_assign_of_length_measurements_work() {
        let mut measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::Centi, LengthUnit::Meter), 18f64);
    }

    #[test]
    fn subtraction_assign_of_length_measurements_work() {
        let mut measurement_one = LengthMeasurement::from(12f64, Prefix::Centi, LengthUnit::Meter);
        let measurement_two = LengthMeasurement::from(60f64, Prefix::Milli, LengthUnit::Meter);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::Centi, LengthUnit::Meter), 6f64);
    }
}