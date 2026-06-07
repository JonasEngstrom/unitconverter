//! # Units and Operations Pertaining to Mass

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Mass
/// 
/// Units for measurement of mass.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
/// 2. National Bureau of Standards. (1959) *Refinement of Values for the Yard and the Pound*. [https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf](https://www.nist.gov/system/files/documents/2017/05/09/frn-59-5442-1959.pdf)
/// 3. United Nations Economic Commission for Europe. (1995) *Codes for Units of Measure Used in International Trade*. [https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf](https://unece.org/sites/default/files/datastore/fileadmin/DAM/trade/untdid/download/r1224a2.pdf)
pub enum MassUnit {
    /// Since the 2019 revision of the SI system defined as one thousandth of a kilogram, which is in turn defined by taking the fixed numerical value of the Planck constant to be 6.62607015 × 10<sup>-34</sup> when expressed in Joule-seconds, which is equal to kilogram-squaremeters per second. The meter and the second being defined from the speed of light in vacuum and the unperturbed ground state hyperfine transition frequency of the cesum 133 atom. Represented by the symbol g.<sup>1</sup>
    Gram,
    /// Defined as 0.45359237 kilograms, since the adoption of the international avoirdupois pound in 1959.<sup>2</sup> Represented by the symbol lb.<sup>3</sup>
    Pound,
}

impl MassUnit {
    /// Returns the factor for converting a unit into kilograms.
    /// 
    /// ```
    /// use unitconverter::mass::MassUnit;
    /// 
    /// let pound = MassUnit::Pound;
    /// 
    /// assert_eq!(pound.factor(), 0.45359237f64);
    /// ```
    pub fn factor(&self) -> f64 {
        match self {
            MassUnit::Gram => 0.001f64,
            MassUnit::Pound => 0.453_592_37f64,
        }
    }

    /// Returns the name of a unit in singular.
    /// 
    /// ```
    /// use unitconverter::mass::MassUnit;
    /// 
    /// let gram = MassUnit::Gram;
    /// 
    /// assert_eq!(gram.name_singular(), "gram");
    /// ```
    pub fn name_singular(&self) -> &str {
        match self {
            MassUnit::Gram => "gram",
            MassUnit::Pound => "pound",
        }
    }

    /// Returns the name of a unit in plural.
    /// 
    /// ```
    /// use unitconverter::mass::MassUnit;
    /// 
    /// let gram = MassUnit::Gram;
    /// 
    /// assert_eq!(gram.name_plural(), "grams");
    /// ```
    pub fn name_plural(&self) -> &str {
        match self {
            MassUnit::Gram => "grams",
            MassUnit::Pound => "pounds"
        }
    }

    /// Returns the symbol of a unit.
    /// 
    /// ```
    /// use unitconverter::mass::MassUnit;
    /// 
    /// let pound = MassUnit::Pound;
    /// 
    /// assert_eq!(pound.symbol(), "lb");
    /// ```
    pub fn symbol(&self) -> &str {
        match self {
            MassUnit::Gram => "g",
            MassUnit::Pound => "lb",
        }
    }
}

/// # Measurement of Mass
/// 
/// A measurement of mass. Stored internally as kilograms, but output as any unit the user desires.
pub struct MassMeasurement { value: f64 }

impl MassMeasurement {
    /// # Store a New Measurement of Mass
    /// 
    /// Measurements are stored using a value, a prefix, and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::mass::{ MassUnit, MassMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_pound = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
    /// 
    /// // Desired output format.
    /// assert_eq!(one_pound.to(Prefix::Kilo, MassUnit::Gram), 0.453_592_37f64);
    /// ```
    pub fn from(value: f64, prefix: Prefix, unit: MassUnit) -> Self {
        Self {
            value: value * Prefix::conversion_constant(prefix, Prefix::None) * unit.factor()
        }
    }

    /// # Convert a Previously Stored Measurement of Mass
    /// 
    /// Prefiously stored `MassMeasurement`s are converted using a prefix and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::mass::{ MassUnit, MassMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let one_pound = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
    /// 
    /// // Desired output format.
    /// assert_eq!(one_pound.to(Prefix::Kilo, MassUnit::Gram), 0.453_592_37f64);
    /// ```
    pub fn to(&self, prefix: Prefix, unit: MassUnit) -> f64 {
        self.value * Prefix::conversion_constant(Prefix::None, prefix) / unit.factor()
    }
}

impl_add_and_subtract!(MassMeasurement);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn factors_are_correct() {
        assert_eq!(MassUnit::Gram.factor(), 0.001f64);
        assert_eq!(MassUnit::Pound.factor(), 0.453_592_37f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(MassUnit::Gram.name_singular(), "gram");
        assert_eq!(MassUnit::Pound.name_singular(), "pound");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(MassUnit::Gram.name_plural(), "grams");
        assert_eq!(MassUnit::Pound.name_plural(), "pounds");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(MassUnit::Gram.symbol(), "g");
        assert_eq!(MassUnit::Pound.symbol(), "lb");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = MassMeasurement::from(0.453_592_37f64, Prefix::Kilo, MassUnit::Gram);
        let test_result = test_measurement.to(Prefix::None, MassUnit::Pound);

        assert_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_mass_measuremens_work() {
        let measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, MassUnit::Pound), 3f64);
    }

    #[test]
    fn subtraction_of_mass_measuremens_work() {
        let measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, MassUnit::Pound), 1f64);
    }

    #[test]
    fn addition_assign_of_mass_measuremens_work() {
        let mut measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, MassUnit::Pound), 3f64);
    }

    #[test]
    fn subtraction_assign_of_mass_measuremens_work() {
        let mut measurement_one = MassMeasurement::from(2f64, Prefix::None, MassUnit::Pound);
        let measurement_two = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, MassUnit::Pound), 1f64);
    }
}