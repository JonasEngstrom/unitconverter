//! # Units and Operations Pertaining to Area
//! 
//! The base unit used to store area in the `unitconverter` crate is square meters.

use crate::macros::*;

use crate::length::{ LengthUnit, LengthMeasurement };

pub enum AreaUnit {
    Custom(LengthUnit, LengthUnit),
}

impl AreaUnit {
    doc_to_base_unit! {
        fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AreaUnit::Custom(length, width) => |x| length.to_base_unit()(1f64) * width.to_base_unit()(1f64) * x,
            }
        }
    }

    doc_from_base_unit! {
        fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
            match self {
                AreaUnit::Custom(length, width) => |x| x / (length.to_base_unit()(1f64) * width.to_base_unit()(1f64)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_unit_to_base_unit_works_correctly() {
        let custom_area_unit = AreaUnit::Custom(LengthUnit::Inch, LengthUnit::Inch);
        let one_square_inch = custom_area_unit.to_base_unit()(1f64);

        assert_eq!(one_square_inch, 0.0254f64 * 0.0254f64);
    }

    #[test]
    fn custom_unit_from_base_unit_works_correctly() {
        let custom_area_unit = AreaUnit::Custom(LengthUnit::Inch, LengthUnit::Inch);
        let one_square_meter = custom_area_unit.from_base_unit()(1f64);

        assert_eq!(one_square_meter, 1f64 / (0.0254f64 * 0.0254f64));
    }
}