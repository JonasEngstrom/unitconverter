//! # Units and Operations Pertaining to Area
//! 
//! The base unit used to store area in the `unitconverter` crate is square meters.

use crate::macros::*;

use si_prefixes::Prefix;
use crate::length::LengthUnit;

pub enum AreaUnit {
    Square(Prefix, LengthUnit),
}

impl AreaUnit {
    doc_to_base_unit! {
        fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AreaUnit::Square(prefix, unit) => {
                    let factor = unit
                        .to_base_unit()(Prefix::conversion_constant(&prefix, &Prefix::None))
                        .powi(2);

                    move |x| factor * x
                },
            }
        }
    }

    doc_from_base_unit! {
        fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AreaUnit::Square(prefix, unit) => {
                    let factor = unit
                        .to_base_unit()(Prefix::conversion_constant(&prefix, &Prefix::None))
                        .powi(2);
                    
                    move |x| x / factor
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_meter_to_base_unit_works() {
        let square_meter_unit = AreaUnit::Square(Prefix::None, LengthUnit::Meter);
        let one_square_meter = square_meter_unit.to_base_unit()(1f64);

        assert_eq!(one_square_meter, 1f64);
    }

    #[test]
    fn square_kilometer_to_base_unit_works() {
        let square_kilometer_unit = AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter);
        let one_square_kilometer = square_kilometer_unit.to_base_unit()(1f64);

        assert_eq!(one_square_kilometer, 1_000_000f64);
    }

    #[test]
    fn square_centimeter_to_base_unit_works() {
        let square_centimeter_unit = AreaUnit::Square(Prefix::Centi, LengthUnit::Meter);
        let one_square_centimeter = square_centimeter_unit.to_base_unit()(1f64);

        assert_eq!(one_square_centimeter, 0.0001f64);
    }

    #[test]
    fn square_foot_to_base_unit_works() {
        let square_foot_unit = AreaUnit::Square(Prefix::None, LengthUnit::Foot);
        let one_square_foot = square_foot_unit.to_base_unit()(1f64);

        assert_eq!(one_square_foot, 0.092_903_04f64);
    }

    #[test]
    fn square_meter_from_base_unit_works() {
        let square_meter_unit = AreaUnit::Square(Prefix::None, LengthUnit::Meter);
        let one_square_meter = square_meter_unit.from_base_unit()(1f64);

        assert_eq!(one_square_meter, 1f64);
    }

    #[test]
    fn square_kilometer_from_base_unit_works() {
        let square_kilometer_unit = AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter);
        let one_square_kilometer = square_kilometer_unit.from_base_unit()(1_000_000f64);

        assert_eq!(one_square_kilometer, 1f64);
    }

    #[test]
    fn square_centimeter_from_base_unit_works() {
        let square_centimeter_unit = AreaUnit::Square(Prefix::Centi, LengthUnit::Meter);
        let one_square_centimeter = square_centimeter_unit.from_base_unit()(1f64);

        assert_eq!(one_square_centimeter, 10_000f64);
    }

    #[test]
    fn square_foot_from_base_unit_works() {
        let square_foot_unit = AreaUnit::Square(Prefix::None, LengthUnit::Foot);
        let one_square_foot = square_foot_unit.from_base_unit()(0.092_903_04f64);

        assert_eq!(one_square_foot, 1f64);
    }
}