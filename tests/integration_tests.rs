#![cfg(test)]

use si_prefixes::Prefix;

use unitconverter::length::{ LengthUnit, LengthMeasurement };
use unitconverter::area::{ AreaUnit, AreaMeasurement };
use unitconverter::volume::VolumeUnit;

/// # Check for Near Equality
/// 
/// Checks that two numbers are within 10<sup>-11</sup> of each other, to account for rouding errors due to floating point operations. For internal use in testing. (Copied from macros.rs.)
macro_rules! assert_almost_eq {
    ($left: expr, $right: expr) => {
        {
            const EPSILON: f64 = 1e-11f64;
            assert!(($left-$right).abs() < EPSILON);
        }
    }
}

#[test]
fn multiplication_of_length_measurements_work() {
    let length_measurement_one = LengthMeasurement::from(1f64, Prefix::Kilo, LengthUnit::Meter);
    let length_measurement_two = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Yard);
    let area_measurement = length_measurement_one * length_measurement_two;

    assert_eq!(area_measurement.to(Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Meter)), 914.4f64);
}

#[test]
fn division_of_area_measurement_with_length_measurement_works() {
    let area_measurement = AreaMeasurement::from(914.4f64, Prefix::None, AreaUnit::Square(Prefix::None, LengthUnit::Meter));
    let length_measurement_one = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Yard);
    let length_measurement_two = area_measurement / length_measurement_one;

    assert_eq!(length_measurement_two.to(Prefix::Kilo, LengthUnit::Meter), 1f64);
}

#[test]
fn multiplication_of_length_and_area_measurments_work() {
    let length_measurement = LengthMeasurement::from(1f64, Prefix::Deci, LengthUnit::Meter);
    let area_measurement = AreaMeasurement::from(1f64, Prefix::None, AreaUnit::Square(Prefix::Deci, LengthUnit::Meter));
    let volume_measurement = length_measurement * area_measurement;

    assert_almost_eq!(volume_measurement.to(Prefix::None, VolumeUnit::Liter), 1f64);
}

#[test]
fn multiplication_of_area_and_length_measurments_work() {
    let length_measurement = LengthMeasurement::from(1f64, Prefix::Deci, LengthUnit::Meter);
    let area_measurement = AreaMeasurement::from(1f64, Prefix::None, AreaUnit::Square(Prefix::Deci, LengthUnit::Meter));
    let volume_measurement = area_measurement * length_measurement;

    assert_almost_eq!(volume_measurement.to(Prefix::None, VolumeUnit::Liter), 1f64);
}