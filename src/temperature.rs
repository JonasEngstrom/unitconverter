//! # Units and Operations Pertaining to Temperature

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Temperature
/// 
/// Units for measurement of temperature.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum TemperatureUnit {
    /// Defined by taking the fixed numerical value of the Boltzmann constant to be 1.380649 × 10<sup>-23</sup> when expressed in J/K. Represented by the symbol K.<sup>1</sup>
    Kelvin,
    Celsius,
    Fahrenheit,
    Rankine,
    Réaumur,
    Rømer,
}

impl TemperatureUnit {
    fn to_kelvin(&self) -> impl FnOnce(f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => |x| x,
            TemperatureUnit::Celsius => |x| x + 273.15f64,
            TemperatureUnit::Fahrenheit => |x| (x - 32f64) / 1.8f64 + 273.15f64,
            TemperatureUnit::Rankine => |x| x / 1.8f64,
            TemperatureUnit::Réaumur => |x| 1.25f64 * x + 273.15f64,
            TemperatureUnit::Rømer => |x| (x - 7.5f64) * 40f64 / 21f64 + 273.15f64,
        }
    }

    fn from_kelvin(&self) -> impl FnOnce(f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => |x| x,
            TemperatureUnit::Celsius => |x| x - 273.15f64,
            TemperatureUnit::Fahrenheit => |x| (x - 273.15f64) * 1.8f64 + 32f64,
            TemperatureUnit::Rankine => |x| x * 1.8f64,
            TemperatureUnit::Réaumur => |x| (x - 273.15f64) * 0.8f64,
            TemperatureUnit::Rømer => |x| (x - 273.15f64) * 0.525f64 + 7.5f64,
        }
    }

    pub fn name_singular(&self) -> &str {
        match self {
            TemperatureUnit::Kelvin => "kelvin",
            TemperatureUnit::Celsius => "degree Celsius",
            TemperatureUnit::Fahrenheit => "degree Fahrenheit",
            TemperatureUnit::Rankine => "degree Rankine",
            TemperatureUnit::Réaumur => "degree Réaumur",
            TemperatureUnit::Rømer => "degree Rømer",
        }
    }
    
    pub fn name_plural(&self) -> &str {
        match self {
            TemperatureUnit::Kelvin => "kelvin",
            TemperatureUnit::Celsius => "degrees Celsius",
            TemperatureUnit::Fahrenheit => "degrees Fahrenheit",
            TemperatureUnit::Rankine => "degrees Rankine",
            TemperatureUnit::Réaumur => "degrees Réaumur",
            TemperatureUnit::Rømer => "degrees Rømer",
        }
    }
    
    pub fn symbol(&self) -> &str {
        match self {
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Rankine => "°Ra",
            TemperatureUnit::Réaumur => "°Ré",
            TemperatureUnit::Rømer => "°Rø",
        }
    }
}

pub struct TemperatureMeasurement { value: f64 }

impl TemperatureMeasurement {
    pub fn from(value: f64, prefix: Prefix, unit: TemperatureUnit) -> Self {
        Self {
            value: Prefix::conversion_constant(prefix, Prefix::None) * unit.to_kelvin()(value)
        }
    }

    pub fn to(&self, prefix: Prefix, unit: TemperatureUnit) -> f64 {
        Prefix::conversion_constant(Prefix::None, prefix) * unit.from_kelvin()(self.value)
    }
}

impl_add_and_subtract!(TemperatureMeasurement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minus_forty_celsius_and_fahrenheit_are_equal() {
        let minus_forty_celsius = TemperatureMeasurement::from(-40f64, Prefix::None, TemperatureUnit::Celsius);

        assert_eq!(minus_forty_celsius.to(Prefix::None, TemperatureUnit::Fahrenheit), -40f64);
    }
}