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
/// 2. National Institute of Standards and Technology. (2026) *Specifications, Tolerances, and Other Technical Requirements for Weighing and Measuring Devices* [https://doi.org/10.6028/NIST.HB.44-2026](https://doi.org/10.6028/NIST.HB.44-2026)
/// 3. National Institute of Standards and Technology. (2008) *NIST Special Publication 811, Guide for the Use of the International System of Units (SI)* [https://www.nist.gov/pml/special-publication-811](https://www.nist.gov/pml/special-publication-811
/// 4. Hofstad, Knut. (2025) *Réaumurskalaen* in *Store norske leksikon* [https://snl.no/r%C3%A9aumurskalaen](https://snl.no/r%C3%A9aumurskalaen)
/// 5. Meyer, K. Ole. Nature 82, 296–298 (1910). *Römer and the Thermometer* [https://doi.org/10.1038/082296a0](https://doi.org/10.1038/082296a0)
pub enum TemperatureUnit {
    /// Defined by taking the fixed numerical value of the Boltzmann constant to be 1.380649 × 10<sup>-23</sup> when expressed in J/K. Represented by the symbol K.<sup>1</sup>
    Kelvin,
    /// Defined as the T - 273.15, where T is the temperature in kelvin. Represented by the symbol °C.<sup>1</sup>
    Celsius,
    /// Defined as (T - 273.15) × 1.8 + 32, where T is the temperature in kelvin. Represented by the symbol °F.<sup>2</sup>
    Fahrenheit,
    /// Defined as T / 1.8, where T is the temperature in kelvin.<sup>3</sup> Often represented by the symbol °R, but represented by °Ra in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Rankine,
    /// Defined as 1.25 × T + 273.15, where T is the temperature in kelvin.<sup>4</sup>  Often represented by the symbol °R, but represented by °Ré in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Réaumur,
    /// Estimated to be (T - 7.5) × 40 / 21 + 273.15, where T is the temperature i Kelvin.<sup>5</sup> Someteimes represented by the symbol °R, but represented by °Rø in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Rømer,
}

impl TemperatureUnit {
    /// Returns a closure converting a unit into the base unit of temperature used in the crate, kelvin.
    fn to_base_unit(&self) -> impl FnOnce(f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => |x| x,
            TemperatureUnit::Celsius => |x| x + 273.15f64,
            TemperatureUnit::Fahrenheit => |x| (x - 32f64) / 1.8f64 + 273.15f64,
            TemperatureUnit::Rankine => |x| x / 1.8f64,
            TemperatureUnit::Réaumur => |x| 1.25f64 * x + 273.15f64,
            TemperatureUnit::Rømer => |x| (x - 7.5f64) * 40f64 / 21f64 + 273.15f64,
        }
    }

    /// Returns a closure converting from the base unit of temperature used in the crate, kelvin, to a given temperature unit.
    fn from_base_unit(&self) -> impl FnOnce(f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => |x| x,
            TemperatureUnit::Celsius => |x| x - 273.15f64,
            TemperatureUnit::Fahrenheit => |x| (x - 273.15f64) * 1.8f64 + 32f64,
            TemperatureUnit::Rankine => |x| x * 1.8f64,
            TemperatureUnit::Réaumur => |x| (x - 273.15f64) * 0.8f64,
            TemperatureUnit::Rømer => |x| (x - 273.15f64) * 0.525f64 + 7.5f64,
        }
    }

    /// Returns the name of a unit in singular.
    /// 
    /// ```
    /// use unitconverter::temperature::TemperatureUnit;
    /// 
    /// let celsius = TemperatureUnit::Celsius;
    /// 
    /// assert_eq!(celsius.name_singular(), "degree Celsius");
    /// ```
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
    
    /// Returns the name of a unit in plural.
    /// 
    /// ```
    /// use unitconverter::temperature::TemperatureUnit;
    /// 
    /// let celsius = TemperatureUnit::Celsius;
    /// 
    /// assert_eq!(celsius.name_plural(), "degrees Celsius");
    /// ```
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
    
    /// Returns the symbol of a unit.
    /// 
    /// ```
    /// use unitconverter::temperature::TemperatureUnit;
    /// 
    /// let celsius = TemperatureUnit::Celsius;
    /// 
    /// assert_eq!(celsius.symbol(), "°C");
    /// ```
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

/// # Measurement of Temperature
/// 
/// A measurement of temperature. Stored internally as kelvin, but output as any unit the user desires.
pub struct TemperatureMeasurement { value: f64 }

impl TemperatureMeasurement {
    /// # Store a New Measurement of Length
    /// 
    /// Measurements are stored using av value, a prefix, and a unit.
    /// 
    /// ```
    /// use unitconverter::temperature::{ TemperatureUnit, TemperatureMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_hundred_celsius = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Celsius);
    /// 
    /// assert_eq!(one_hundred_celsius.to(Prefix::None, TemperatureUnit::Kelvin), 373.15f64);
    /// ```
    pub fn from(value: f64, prefix: Prefix, unit: TemperatureUnit) -> Self {
        Self {
            value: unit.to_base_unit()(Prefix::conversion_constant(prefix, Prefix::None) * value)
        }
    }

    /// # Convert a Previously Stored Measurement of Length
    /// 
    /// Previously stored `TemperatureMeasurement`s are convdrted using a prefix and a unit.
    /// 
    /// ```
    /// use unitconverter::temperature::{ TemperatureUnit, TemperatureMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_hundred_celsius = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Celsius);
    /// 
    /// assert_eq!(one_hundred_celsius.to(Prefix::None, TemperatureUnit::Kelvin), 373.15f64);
    /// ```
    pub fn to(&self, prefix: Prefix, unit: TemperatureUnit) -> f64 {
        Prefix::conversion_constant(Prefix::None, prefix) * unit.from_base_unit()(self.value)
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