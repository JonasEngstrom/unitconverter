//! # Units and Operations Pertaining to Volume
//! 
//! The base unt used to store volume in the `unitconverter` crate is liters.

use crate::macros::*;
use crate::formulas::*;

use crate::length::LengthUnit;

/// # Units of Volume
/// 
/// Units for measurement of volume.
/// 
/// # References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum VolumeUnit {
    /// Defined as one cubic decimeter. Represented by the symbol l.<sup>1</sup> (Note that the liter is also represented by the symbol L or ℓ in some regions and academic disciplines. In the `unitconverter` crate the lower-case symbol is used to achieve consistency across symbols, as liter is not a unit derived from a name. Just be aware that in communication aimed at some parts of the world or certain scientific fields, the symbol may have to be changed. If you wish to read more about the topic, the [English language version](https://en.wikipedia.org/wiki/Litre#Symbol) of Wikipedia discusses the topic, [French Wikipedia](https://fr.wikipedia.org/wiki/Litre#Symbole) provides more examples of capitalized symbols of units named after people, while [Spanish Wikipedia](https://es.wikipedia.org/wiki/Litro#Símbolo) goes into how the use of typewriters led to some countries switching to an upper-case symbol for the liter.)
    Liter,
    /// The cube of any length unit supported by the `unitconverter` crate.
    Cubic(Prefix, LengthUnit),
}

impl VolumeUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                VolumeUnit::Liter => Formula::Multiply{ scale: 1f64 },
                VolumeUnit::Cubic(prefix, unit) => {
                    Formula::Multiply{
                        scale: unit
                            .to_base_unit(Prefix::conversion_constant(&prefix, &Prefix::Deci))
                            .powi(3)
                    }
                }
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                VolumeUnit::Liter => Formula::Divide{ scale: 1f64 },
                VolumeUnit::Cubic(prefix, unit) => {
                    Formula::Divide{
                        scale: unit
                            .to_base_unit(Prefix::conversion_constant(&prefix, &Prefix::Deci))
                            .powi(3)
                    }
                }
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> String {
            match self {
                VolumeUnit::Liter => "liter".to_string(),
                VolumeUnit::Cubic(prefix, unit) => {
                    let prefix_name = match prefix.name() {
                        Some(name) => name,
                        None => "",
                    };
                    format!("cubic {}{}", prefix_name, unit.name_singular())
                }
            }
        }
    }

    doc_name_plural! {
        pub fn name_plural(&self) -> String {
            match self {
                VolumeUnit::Liter => "liters".to_string(),
                VolumeUnit::Cubic(prefix, unit) => {
                    let prefix_name = match prefix.name() {
                        Some(name) => name,
                        None => "",
                    };
                    format!("cubic {}{}", prefix_name, unit.name_plural())
                }
            }
        }
    }

    doc_symbol! {
        fn symbol(&self) -> String {
            match self {
                VolumeUnit::Liter => "l".to_string(),
                VolumeUnit::Cubic(prefix, unit) => {
                    let prefix_name = match prefix.symbol() {
                        Some(name) => name,
                        None => "",
                    };
                    format!("{}{}³", prefix_name, unit.symbol())
                }
            }
        }
    }
}

impl_measurement!(VolumeMeasurement, VolumeUnit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_units_are_correct() {
        assert_eq!(VolumeUnit::Cubic(Prefix::Deci, LengthUnit::Meter).to_base_unit(1f64), 1f64);
        assert_eq!(VolumeUnit::Liter.to_base_unit(1f64), 1f64);
    }

    #[test]
    fn from_base_units_are_correct() {
        assert_eq!(VolumeUnit::Cubic(Prefix::None, LengthUnit::Meter).from_base_unit(1_000f64), 1f64);
        assert_eq!(VolumeUnit::Liter.from_base_unit(1_000f64), 1_000f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(VolumeUnit::Liter.name_singular(), "liter");
        assert_eq!(VolumeUnit::Cubic(Prefix::None, LengthUnit::Inch).name_singular(), "cubic inch");
        assert_eq!(VolumeUnit::Cubic(Prefix::Kilo, LengthUnit::Meter).name_singular(), "cubic kilometer");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(VolumeUnit::Liter.name_plural(), "liters");
        assert_eq!(VolumeUnit::Cubic(Prefix::None, LengthUnit::Inch).name_plural(), "cubic inches");
        assert_eq!(VolumeUnit::Cubic(Prefix::Kilo, LengthUnit::Meter).name_plural(), "cubic kilometers");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(VolumeUnit::Liter.symbol(), "l");
        assert_eq!(VolumeUnit::Cubic(Prefix::None, LengthUnit::Inch).symbol(), "in³");
        assert_eq!(VolumeUnit::Cubic(Prefix::Kilo, LengthUnit::Meter).symbol(), "km³");
    }

    #[test]
    fn addition_of_volume_measurements_work() {
        let measurement_one = VolumeMeasurement::from(1f64, Prefix::None, VolumeUnit::Cubic(Prefix::Deci, LengthUnit::Meter));
        let measurement_two = VolumeMeasurement::from(0.001f64, Prefix::None, VolumeUnit::Cubic(Prefix::None, LengthUnit::Meter));
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, VolumeUnit::Liter), 2f64);
    }

    #[test]
    fn substraction_of_volume_measurements_work() {
        let measurement_one = VolumeMeasurement::from(1f64, Prefix::None, VolumeUnit::Cubic(Prefix::Deci, LengthUnit::Meter));
        let measurement_two = VolumeMeasurement::from(0.001f64, Prefix::None, VolumeUnit::Cubic(Prefix::None, LengthUnit::Meter));
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, VolumeUnit::Liter), 0f64);
    }

    #[test]
    fn addition_assign_of_volume_measurements_work() {
        let mut measurement_one = VolumeMeasurement::from(1f64, Prefix::None, VolumeUnit::Cubic(Prefix::Deci, LengthUnit::Meter));
        let measurement_two = VolumeMeasurement::from(0.001f64, Prefix::None, VolumeUnit::Cubic(Prefix::None, LengthUnit::Meter));
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, VolumeUnit::Liter), 2f64);
    }

    #[test]
    fn substraction_assign_of_volume_measurements_work() {
        let mut measurement_one = VolumeMeasurement::from(1f64, Prefix::None, VolumeUnit::Cubic(Prefix::Deci, LengthUnit::Meter));
        let measurement_two = VolumeMeasurement::from(0.001f64, Prefix::None, VolumeUnit::Cubic(Prefix::None, LengthUnit::Meter));
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, VolumeUnit::Liter), 0f64);
    }
}