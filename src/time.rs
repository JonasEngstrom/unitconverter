//! # Units and Operations Pertaining to Time

use si_prefixes::Prefix;

use crate::macros::impl_add_and_subtract;

/// # Units of Time
/// 
/// Units for measurement of time.
/// 
/// ## References
/// 
/// 1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)
pub enum TimeUnit {
    /// Defined by taking the unperturbed ground-state hyperfine transition frequency of the cesium-133 atom, to be 9,192,631,770 Hz. Hertz being equal to s<sup>−1</sup>. Represented by the symbol s.<sup>1</sup>
    Second,
    /// Defined as 60 seconds. Represented by the symbol min.<sup>1</sup>
    Minute,
    /// Defined as 3,600 seconds. Equal to 60 minutes. Represented by the symbol h.<sup>1</sup>
    Hour,
    /// Defined as 86,400 seconds. Equal to 24 hours. Represented by the symbol d.<sup>1</sup>
    Day,
}

impl TimeUnit {
    /// Returns the factor fo converting a unit into seconds.
    pub fn factor(&self) -> f64 {
        match self {
            TimeUnit::Second => 1f64,
            TimeUnit::Minute => 60f64,
            TimeUnit::Hour => 3_600f64,
            TimeUnit::Day => 86_400f64,
        }
    }

    /// Returns the name of a unit in singular.
    pub fn name_singular(&self) -> &str {
        match self {
            TimeUnit::Second => "second",
            TimeUnit::Minute => "minute",
            TimeUnit::Hour => "hour",
            TimeUnit::Day => "day",
        }
    }

    /// Returns the name of a unit in plural.
    pub fn name_plural(&self) -> &str {
        match self {
            TimeUnit::Second => "seconds",
            TimeUnit::Minute => "minutes",
            TimeUnit::Hour => "hours",
            TimeUnit::Day => "days",
        }
    }

    /// Returns the symbol of a unit.
    pub fn symbol(&self) -> &str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "h",
            TimeUnit::Day => "d",
        }
    }
}