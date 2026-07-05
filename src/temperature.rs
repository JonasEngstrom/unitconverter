//! # Units and Operations Pertaining to Temperature
//! 
//! The base unit used to store temperature in the `unitconverter` crate is kelvin.

use crate::macros::*;
use crate::formulas::*;

/// # Units of Temperature
/// 
/// Units for measurement of temperature.
/// 
/// ## References
/// 
/// 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)
/// 2. [Delisle scale. In: Wikipedia. 2025.](https://en.wikipedia.org/wiki/Delisle_scale)
/// 3. [Chavez Baucom I, Benham EJ, Konijnenburg J, Lee GD, Lippa KA, McGuire JT, et al. Specifications, tolerances, and other technical requirements for weighing and measuring devices—as adopted by the 110th National Council on Weights and Measures. Gaithersburg (MD): National Institute of Standards and Technology; 2026 Jan. Report NIST HB 44-2026. doi:10.6028/NIST.HB.44-2026](https://www.doi.org/10.6028/NIST.HB.44-2026)
/// 4. [Grigull U. Newton’s temperature scale and the law of cooling. Wärme- und Stoffübertragung. 1984 Dec;18(4):195–9. doi:10.1007/BF01007129](https://www.doi.org/10.1007/BF01007129)
/// 5. [Thompson A, Taylor BN. Guide for the use of the international system of units (SI). Gaithersburg (MD): National Institute of Standards and Technology; 2008 Mar. Report NIST Special Publication 811.](https://www.nist.gov/pml/special-publication-811)
/// 6. [Hofstad K. Réaumurskalaen. In: Store Norske Leksikon. Oslo (Norway); 2025.](https://snl.no/réaumurskalaen)
/// 7. [Meyer K. Ole Römer and the Thermometer. Nature. 1910 Jan 6;82(2097):296–8. doi:10.1038/082296a0](https://www.doi.org/10.1038/082296a0)
pub enum TemperatureUnit {
    /// Defined as the T - 273.15, where T is the temperature in kelvin. Represented by the symbol °C.<sup>1</sup>
    Celsius,
    /// Defined as (373.15 - T) × 3 / 2, where T is the temperature in kelvin. Represented by the symbol °De.<sup>2</sup>
    Delisle,
    /// Defined as (T - 273.15) × 1.8 + 32, where T is the temperature in kelvin. Represented by the symbol °F.<sup>3</sup>
    Fahrenheit,
    /// Defined by taking the fixed numerical value of the Boltzmann constant to be 1.380649 × 10<sup>-23</sup> when expressed in J/K. Represented by the symbol K.<sup>1</sup>
    Kelvin,
    /// Although no definitive conversion factor can be determined based on Newton’s own notes, this crate uses approximation of (T - 273.15) × 0.33, where T is the temperature in kelvin. Represented by the symbol °N.<sup>4</sup>
    Newton,
    /// Defined as T × 1.8, where T is the temperature in kelvin.<sup>5</sup> Often represented by the symbol °R, but represented by °Ra in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Rankine,
    /// Defined as (T - 273.15) × 0.8, where T is the temperature in kelvin.<sup>6</sup>  Often represented by the symbol °R, but represented by °Ré in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Réaumur,
    /// Estimated to be (T - 273.15) × 0.525 + 7.5, where T is the temperature i Kelvin.<sup>7</sup> Someteimes represented by the symbol °R, but represented by °Rø in this crate, to distinguish it from degrees in the other temperature scales starting with R.
    Rømer,
}

impl TemperatureUnit {
    doc_to_base_unit_formula! {
        fn to_base_unit_formula(&self) -> Formula {
            match self {
                TemperatureUnit::Kelvin => Formula::Affine {
                    scale: 1f64,
                    offset: 0f64,
                },
                TemperatureUnit::Celsius => Formula::Affine {
                    scale: 1f64,
                    offset: 273.15f64,
                },
                TemperatureUnit::Fahrenheit => Formula::Affine {
                    scale: 1f64 / 1.8f64,
                    offset: 273.15f64 - 32f64 / 1.8f64,
                },
                TemperatureUnit::Rankine => Formula::Affine {
                    scale: 1f64 / 1.8f64,
                    offset: 0f64,
                },
                TemperatureUnit::Réaumur => Formula::Affine {
                    scale: 1.25f64,
                    offset: 273.15f64,
                },
                TemperatureUnit::Rømer => Formula::Affine {
                    scale: 40f64 / 21f64,
                    offset: 273.15f64 - 7.5f64 * 40f64 / 21f64,
                },
                TemperatureUnit::Newton => Formula::Affine {
                    scale: 100f64 / 33f64,
                    offset: 273.15f64,
                },
                TemperatureUnit::Delisle => Formula::Affine {
                    scale: -2f64 / 3f64,
                    offset: 373.15f64,
                },
            }
        }
    }

    doc_from_base_unit_formula! {
        fn from_base_unit_formula(&self) -> Formula {
            match self {
                TemperatureUnit::Kelvin => Formula::Affine {
                    scale: 1f64,
                    offset: 0f64,
                },
                TemperatureUnit::Celsius => Formula::Affine {
                    scale: 1f64,
                    offset: -273.15f64,
                },
                TemperatureUnit::Fahrenheit => Formula::Affine {
                    scale: 1.8f64,
                    offset: 32f64 - 273.15f64 * 1.8f64,
                },
                TemperatureUnit::Rankine => Formula::Affine {
                    scale: 1.8f64,
                    offset: 0f64,
                },
                TemperatureUnit::Réaumur => Formula::Affine {
                    scale: 0.8f64,
                    offset: -273.15f64 * 0.8f64,
                },
                TemperatureUnit::Rømer => Formula::Affine {
                    scale: 0.525f64,
                    offset: 7.5f64 - 273.15f64 * 0.525f64,
                },
                TemperatureUnit::Newton => Formula::Affine {
                    scale: 0.33f64,
                    offset: -273.15f64 * 0.33f64,
                },
                TemperatureUnit::Delisle => Formula::Affine {
                    scale: -1.5f64,
                    offset: 1.5f64 * 373.15f64,
                },
            }
        }
    }

    doc_name_singular! {
        pub fn name_singular(&self) -> &str {
            match self {
                TemperatureUnit::Kelvin => "kelvin",
                TemperatureUnit::Celsius => "degree Celsius",
                TemperatureUnit::Fahrenheit => "degree Fahrenheit",
                TemperatureUnit::Rankine => "degree Rankine",
                TemperatureUnit::Réaumur => "degree Réaumur",
                TemperatureUnit::Rømer => "degree Rømer",
                TemperatureUnit::Newton => "degree Newton",
                TemperatureUnit::Delisle => "degree Delisle",
            }
        }
    }
    
    doc_name_plural! {
        pub fn name_plural(&self) -> &str {
            match self {
                TemperatureUnit::Kelvin => "kelvin",
                TemperatureUnit::Celsius => "degrees Celsius",
                TemperatureUnit::Fahrenheit => "degrees Fahrenheit",
                TemperatureUnit::Rankine => "degrees Rankine",
                TemperatureUnit::Réaumur => "degrees Réaumur",
                TemperatureUnit::Rømer => "degrees Rømer",
                TemperatureUnit::Newton => "degrees Newton",
                TemperatureUnit::Delisle => "degrees Delisle",
            }
        }
    }
    
    doc_symbol! {
        pub fn symbol(&self) -> &str {
            match self {
                TemperatureUnit::Kelvin => "K",
                TemperatureUnit::Celsius => "°C",
                TemperatureUnit::Fahrenheit => "°F",
                TemperatureUnit::Rankine => "°Ra",
                TemperatureUnit::Réaumur => "°Ré",
                TemperatureUnit::Rømer => "°Rø",
                TemperatureUnit::Newton => "°N",
                TemperatureUnit::Delisle => "°De",
            }
        }
    }
}

impl_measurement!(TemperatureMeasurement, TemperatureUnit);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::macros::assert_almost_eq;

    #[test]
    fn minus_forty_celsius_and_fahrenheit_are_equal() {
        let minus_forty_celsius = TemperatureMeasurement::from(-40f64, Prefix::None, TemperatureUnit::Celsius);

        assert_eq!(minus_forty_celsius.to(Prefix::None, TemperatureUnit::Fahrenheit), -40f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(TemperatureUnit::Kelvin.name_singular(), "kelvin");
        assert_eq!(TemperatureUnit::Celsius.name_singular(), "degree Celsius");
        assert_eq!(TemperatureUnit::Fahrenheit.name_singular(), "degree Fahrenheit");
        assert_eq!(TemperatureUnit::Rankine.name_singular(), "degree Rankine");
        assert_eq!(TemperatureUnit::Réaumur.name_singular(), "degree Réaumur");
        assert_eq!(TemperatureUnit::Rømer.name_singular(), "degree Rømer");
        assert_eq!(TemperatureUnit::Delisle.name_singular(), "degree Delisle");
        assert_eq!(TemperatureUnit::Newton.name_singular(), "degree Newton");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(TemperatureUnit::Kelvin.name_plural(), "kelvin");
        assert_eq!(TemperatureUnit::Celsius.name_plural(), "degrees Celsius");
        assert_eq!(TemperatureUnit::Fahrenheit.name_plural(), "degrees Fahrenheit");
        assert_eq!(TemperatureUnit::Rankine.name_plural(), "degrees Rankine");
        assert_eq!(TemperatureUnit::Réaumur.name_plural(), "degrees Réaumur");
        assert_eq!(TemperatureUnit::Rømer.name_plural(), "degrees Rømer");
        assert_eq!(TemperatureUnit::Delisle.name_plural(), "degrees Delisle");
        assert_eq!(TemperatureUnit::Newton.name_plural(), "degrees Newton");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(TemperatureUnit::Kelvin.symbol(), "K");
        assert_eq!(TemperatureUnit::Celsius.symbol(), "°C");
        assert_eq!(TemperatureUnit::Fahrenheit.symbol(), "°F");
        assert_eq!(TemperatureUnit::Rankine.symbol(), "°Ra");
        assert_eq!(TemperatureUnit::Réaumur.symbol(), "°Ré");
        assert_eq!(TemperatureUnit::Rømer.symbol(), "°Rø");
        assert_eq!(TemperatureUnit::Delisle.symbol(), "°De");
        assert_eq!(TemperatureUnit::Newton.symbol(), "°N");
    }

    #[test]
    fn conversion_from_kelvin_works() {
        let one_hundred_kelvin = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Kelvin);

        assert_almost_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Celsius), -1731.5f64);
        assert_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Fahrenheit), -2796.7f64);
        assert_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Kelvin), 1000f64);
        assert_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Rankine), 1800f64);
        assert_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Newton), -571.395f64);
        assert_almost_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Réaumur), -1385.2f64);
        assert_almost_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Rømer), -834.0375f64);
        assert_almost_eq!(one_hundred_kelvin.to(Prefix::Deci, TemperatureUnit::Delisle), 4097.25f64);
    }

    #[test]
    fn conversion_from_celsius_works() {
        let one_hundred_celsius = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Celsius);

        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Celsius), 1000f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Fahrenheit), 2120f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Kelvin), 3731.5f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Rankine), 6716.7f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Newton), 330f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Réaumur), 800f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Rømer), 600f64);
        assert_eq!(one_hundred_celsius.to(Prefix::Deci, TemperatureUnit::Delisle), 0f64);
    }

    #[test]
    fn conversion_from_fahrenheit_works() {
        let one_hundred_fahrenheit = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Fahrenheit);

        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Celsius), 68f64 * 5f64 / 9f64 * 10f64);
        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Fahrenheit), 1000f64);
        assert_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Kelvin), 559.67f64 * 5f64 / 9f64 * 10f64);
        assert_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Rankine), 5596.7f64);
        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Newton), 68f64 * 11f64 / 60f64 * 10f64);
        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Réaumur), 68f64 * 4f64 / 9f64 * 10f64);
        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Rømer), (68f64 * 7f64 / 24f64 + 7.5f64) * 10f64);
        assert_almost_eq!(one_hundred_fahrenheit.to(Prefix::Deci, TemperatureUnit::Delisle), 5600f64 / 6f64);
    }

    #[test]
    fn conversion_from_rankine_works() {
        let one_hundred_rankine = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Rankine);

        assert_almost_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Celsius), -391.67f64 * 5f64 / 9f64 * 10f64);
        assert_almost_eq!(one_hundred_rankine.to(Prefix::None, TemperatureUnit::Fahrenheit), -359.67f64);
        assert_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Kelvin), 5000f64 / 9f64);
        assert_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Rankine), 1000f64);
        assert_almost_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Newton), -391.67f64 * 11f64 / 60f64 * 10f64);
        assert_almost_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Réaumur), -391.67f64 * 4f64 / 9f64 * 10f64);
        assert_eq!(one_hundred_rankine.to(Prefix::Deci, TemperatureUnit::Rømer), (-391.67f64 * 7f64 / 24f64 + 7.5f64) * 10f64);
        assert_almost_eq!(one_hundred_rankine.to(Prefix::None, TemperatureUnit::Delisle), 2858.35f64 / 6f64);
        
    }

    #[test]
    fn conversion_from_réaumur_works() {
        let one_hundred_réaumur = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Réaumur);

        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Celsius), 1250f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Fahrenheit), 2570f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Kelvin), 3981.5f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Rankine), 7166.7f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Newton), 412.5f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Réaumur), 1000f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Rømer), 731.25f64);
        assert_eq!(one_hundred_réaumur.to(Prefix::Deci, TemperatureUnit::Delisle), -375f64);
    }

    #[test]
    fn conversion_from_rømer_works() {
        let one_hundred_rømer = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Rømer);

        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Celsius), (92.5f64 * 40f64 / 21f64) * 10f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Fahrenheit), (92.5f64 * 24f64 / 7f64 + 32f64) * 10f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Rømer), 1000f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Rankine), (2220f64 / 7f64 + 491.67f64) * 10f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Newton), (3700f64 / 21f64) * 3.3f64);
        assert_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Réaumur), 29600f64 / 21f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Rømer), 1000f64);
        assert_almost_eq!(one_hundred_rømer.to(Prefix::Deci, TemperatureUnit::Delisle), (373.15f64 - (3700f64 / 21f64 + 273.15f64)) * 30f64 / 2f64);
    }

    #[test]
    fn conversion_from_newton_works() {
        let one_hundred_newton = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Newton);

        assert_almost_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Celsius), 100000f64 / 33f64);
        assert_almost_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Fahrenheit), (100f64 * 60f64 / 11f64 + 32f64) * 10f64);
        assert_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Kelvin), (10000f64 /33f64 + 273.15f64) * 10f64);
        assert_almost_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Rankine), ((100f64 * 60f64 / 11f64) + 491.67f64) * 10f64);
        assert_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Newton), 1000f64);
        assert_almost_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Réaumur), 80000f64 / 33f64);
        assert_almost_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Rømer), (10000f64 / 33f64 * 21f64 / 40f64 + 7.5f64) * 10f64);
        assert_eq!(one_hundred_newton.to(Prefix::Deci, TemperatureUnit::Delisle), 1500f64 - 150000f64 / 33f64);
    }

    #[test]
    fn conversion_from_delisle_works() {
        let one_hundred_delisle = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Delisle);

        assert_almost_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Celsius), 1000f64 / 3f64);
        assert_almost_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Fahrenheit), (212f64 - 600f64 / 5f64) * 10f64);
        assert_almost_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Kelvin), (373.15f64 - 200f64 / 3f64) * 10f64);
        assert_almost_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Rankine), (671.67f64 - 600f64 / 5f64) * 10f64);
        // assert_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Newton), 1000f64);
        // assert_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Réaumur), 80000f64 / 33f64);
        // assert_almost_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Rømer), (10000f64 / 33f64 * 21f64 / 40f64 + 7.5f64) * 10f64);
        // assert_eq!(one_hundred_delisle.to(Prefix::Deci, TemperatureUnit::Delisle), 1500f64 - 150000f64 / 33f64);
    }

    #[test]
    fn addition_of_temperature_measurment_work() {
        let measurement_one = TemperatureMeasurement::from(-73.15f64, Prefix::None, TemperatureUnit::Celsius);
        let measurement_two = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Kelvin);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, TemperatureUnit::Kelvin), 300f64);
    }

    #[test]
    fn subtraction_of_temperature_measurment_work() {
        let measurement_one = TemperatureMeasurement::from(15f64, Prefix::None, TemperatureUnit::Celsius);
        let measurement_two = TemperatureMeasurement::from(2.7315f64, Prefix::Hecto, TemperatureUnit::Kelvin);
        let difference = measurement_two - measurement_one;

        assert_eq!(difference.to(Prefix::None, TemperatureUnit::Kelvin), -15f64);
    }

    #[test]
    fn addition_assign_of_temperature_measurment_work() {
        let mut measurement_one = TemperatureMeasurement::from(-73.15f64, Prefix::None, TemperatureUnit::Celsius);
        let measurement_two = TemperatureMeasurement::from(1f64, Prefix::Hecto, TemperatureUnit::Kelvin);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, TemperatureUnit::Kelvin), 300f64);
    }

    #[test]
    fn subtraction_assign_of_temperature_measurment_work() {
        let measurement_one = TemperatureMeasurement::from(15f64, Prefix::None, TemperatureUnit::Celsius);
        let mut measurement_two = TemperatureMeasurement::from(2.7315f64, Prefix::Hecto, TemperatureUnit::Kelvin);
        measurement_two -= measurement_one;

        assert_eq!(measurement_two.to(Prefix::None, TemperatureUnit::Kelvin), -15f64);
    }
}