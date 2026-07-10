#![cfg(test)]

use si_prefixes::Prefix;

use unitconverter::length::{ LengthUnit, LengthMeasurement };
use unitconverter::area::{ AreaUnit, AreaMeasurement };

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