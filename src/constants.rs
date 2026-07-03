//! # The Seven Defining Constants of the SI System
//! 
//! The constants defning the units of the SI system<sup>1</sup>.
//! 
//! ## References
//! 
//! 1. [Le Système international d’unités, 9e édition. Sèvres (France): Bureau International des Poids et Mesures; 2025 Aug. doi:10.59161/AUEZ1291](https://www.doi.org/10.59161/AUEZ1291)


/// The unperturbed ground state hyperfine transition frequency of the cesium 133 atom in hertz. Represented by the symbol Δ<sub>VCs</sub>.<sup>1</sup>
pub const FREQUENCY_OF_CESIUM: f64 = 9_192_631_770f64;

/// The speed of light in vacuum in meters per second. Represented by the symbol c.<sup>1</sup>
pub const SPEED_OF_LIGHT: f64 = 299_792_458f64;

/// The Planck constant in joule-seconds. Represented by the symbol h.<sup>1</sup>
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34f64;

/// The elementary charge in coulomb. Represented by the symbol e.<sup>1</sup>
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19f64;

/// The Boltzmann constant in joules per kelvin. Represented by the symbol k.<sup>1</sup>
pub const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23f64;

/// The Avogadro constant in reciprocal moles. Represented by the symbol N<sub>A</sub>.<sup>1</sup>
pub const AVOGADRO_CONSTANT: f64 = 6.022_140_76e23f64;

/// The luminous efficacy of monochromatic radiation of frequency 540 × 10<sub>12</sub> hertz in lumens per watt. Represented by the symbol K<sub>cd</sub>.<sup>1</sup>
pub const LUMINOUS_EFFICACY: f64 = 683f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyperfine_transition_frequency_of_cesium_is_correct() {
        assert_eq!(FREQUENCY_OF_CESIUM, 9_192_631_770f64);
    }

    #[test]
    fn speed_of_light_is_correct() {
        assert_eq!(SPEED_OF_LIGHT, 299_792_458f64);
    }

    #[test]
    fn planck_constant_is_correct() {
        assert_eq!(PLANCK_CONSTANT, 6.626_070_15e-34f64);
    }

    #[test]
    fn elementary_charge_is_correct() {
        assert_eq!(ELEMENTARY_CHARGE, 1.602_176_634e-19f64);
    }

    #[test]
    fn boltzamnn_constant_is_correct() {
        assert_eq!(BOLTZMANN_CONSTANT, 1.380_649e-23f64);
    }

    #[test]
    fn avogadro_constant_is_correct() {
        assert_eq!(AVOGADRO_CONSTANT, 6.022_140_76e23f64);
    }

    #[test]
    fn luminous_efficacy_is_correct() {
        assert_eq!(LUMINOUS_EFFICACY, 683f64);
    }
}