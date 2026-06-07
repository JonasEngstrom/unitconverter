//! # Units and Operations Pertaining to Electric Current

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Electric Current
/// 
/// Units for measurement of electric current.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum CurrentUnit {
    /// Since the 2019 revision of the SI system defined by taking the fixed numerical value of the elementary charge to be 1.602176634 × 10<sup>-19</sup> when expressed in coulomb, which is equal to ampere-seconds. The second being defined from the unperturbed ground state hyperfine transition frequency of the cesum 133 atom. Represented by the symbol A.<sup>1</sup>
    Ampere,
}

impl CurrentUnit {
    /// Returns the factor for converting a unit into amperes.
    /// 
    /// ```
    /// use unitconverter::current::CurrentUnit;
    /// 
    /// let ampere = CurrentUnit::Ampere;
    /// 
    /// assert_eq!(ampere.factor(), 1f64);
    /// ```
    pub fn factor(&self) -> f64 {
        match self {
            CurrentUnit::Ampere => 1f64,
        }
    }

    /// Returns the name of a unit in singular.
    /// 
    /// ```
    /// use unitconverter::current::CurrentUnit;
    /// 
    /// let ampere = CurrentUnit::Ampere;
    /// 
    /// assert_eq!(ampere.name_singular(), "ampere");
    /// ```
    pub fn name_singular(&self) -> &str {
        match self {
            CurrentUnit::Ampere => "ampere",
        }
    }

    /// Returns the name of a unit in plural.
    /// 
    /// ```
    /// use unitconverter::current::CurrentUnit;
    /// 
    /// let ampere = CurrentUnit::Ampere;
    /// 
    /// assert_eq!(ampere.name_plural(), "amperes");
    /// ```
    pub fn name_plural(&self) -> &str {
        match self {
            CurrentUnit::Ampere => "amperes",
        }
    }

    /// Returns the symbol of a unit.
    /// 
    /// ```
    /// use unitconverter::current::CurrentUnit;
    /// 
    /// let ampere = CurrentUnit::Ampere;
    /// 
    /// assert_eq!(ampere.symbol(), "A");
    /// ```
    pub fn symbol(&self) -> &str {
        match self {
            CurrentUnit::Ampere => "A",
        }
    }
}

/// # Measurement of Electric Current
/// 
/// A measurement of electric current. Stored internally as amperes, but output as any unit the user desires.
pub struct CurrentMeasurement { value: f64 }

impl CurrentMeasurement {
    /// # Store a New Measurement of Electric Current
    /// 
    /// Measurements are stored using a value, a prefix, and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::current::{ CurrentUnit, CurrentMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let kiloampere = CurrentMeasurement::from(1f64, Prefix::Kilo, CurrentUnit::Ampere);
    /// 
    /// // Desired output format.
    /// assert_eq!(kiloampere.to(Prefix::None, CurrentUnit::Ampere), 1_000f64);
    /// ```
    pub fn from(value: f64, prefix: Prefix, unit: CurrentUnit) -> Self {
        Self {
            value: value * Prefix::conversion_constant(prefix, Prefix::None) * unit.factor()
        }
    }

    /// # Convert a Previously Stored Measurement of Electric Current
    /// 
    /// Measurements are stored using a value, a prefix, and a unit, as illustrated in the following examples:
    /// 
    /// ```
    /// use unitconverter::current::{ CurrentUnit, CurrentMeasurement };
    /// use si_prefixes::Prefix;
    /// 
    /// // Desired input format.
    /// let kiloampere = CurrentMeasurement::from(1f64, Prefix::Kilo, CurrentUnit::Ampere);
    /// 
    /// // Desired output format.
    /// assert_eq!(kiloampere.to(Prefix::None, CurrentUnit::Ampere), 1_000f64);
    /// ```
    pub fn to(&self, prefix: Prefix, unit: CurrentUnit) -> f64 {
        self.value * Prefix::conversion_constant(Prefix::None, prefix) / unit.factor()
    }
}

impl_add_and_subtract!(CurrentMeasurement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_are_correct() {
        assert_eq!(CurrentUnit::Ampere.factor(), 1f64);
    }

    #[test]
    fn singular_names_are_correct() {
        assert_eq!(CurrentUnit::Ampere.name_singular(), "ampere");
    }

    #[test]
    fn plural_names_are_correct() {
        assert_eq!(CurrentUnit::Ampere.name_plural(), "amperes");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(CurrentUnit::Ampere.symbol(), "A");
    }

    #[test]
    fn conversion_works() {
        let test_measurement = CurrentMeasurement::from(100f64, Prefix::None, CurrentUnit::Ampere);
        let test_result = test_measurement.to(Prefix::Hecto, CurrentUnit::Ampere);

        assert_eq!(test_result, 1f64);
    }

    #[test]
    fn addition_of_current_measuremens_work() {
        let measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        let sum = measurement_one + measurement_two;

        assert_eq!(sum.to(Prefix::None, CurrentUnit::Ampere), 0.18f64);
    }

    #[test]
    fn subtraction_of_current_measuremens_work() {
        let measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        let difference = measurement_one - measurement_two;

        assert_eq!(difference.to(Prefix::None, CurrentUnit::Ampere), 0.06f64);
    }

    #[test]
    fn addition_assign_of_current_measuremens_work() {
        let mut measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        measurement_one += measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CurrentUnit::Ampere), 0.18f64);
    }

    #[test]
    fn subtraction_assign_of_current_measuremens_work() {
        let mut measurement_one = CurrentMeasurement::from(12f64, Prefix::Centi, CurrentUnit::Ampere);
        let measurement_two = CurrentMeasurement::from(60f64, Prefix::Milli, CurrentUnit::Ampere);
        measurement_one -= measurement_two;

        assert_eq!(measurement_one.to(Prefix::None, CurrentUnit::Ampere), 0.06f64);
    }
}