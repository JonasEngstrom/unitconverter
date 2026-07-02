[![Build and Test](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/unitconverter)](https://crates.io/crates/unitconverter)
[![codecov](https://codecov.io/gh/JonasEngstrom/unitconverter/branch/main/graph/badge.svg?token=4R7RT01VHT)](https://codecov.io/gh/JonasEngstrom/unitconverter)

# Unit Converter

Unit converter is a crate for converting between different units of measurement.

## Supported Units

### Amount of Substance

The base unit of amount of substance used in the `unitconverter` crate is moles.

- Mole

### Electric Current

The base unit of electric current used in the `unitconverter` crate is amperes.

- Ampere

### Length

The base unit of length used in the `unitconverter` crate is meters.

- Foot
- Inch
- Meter
- Mile
- Yard

### Luminous Intensity

The base unit of luminous intensity used in the `unitconverter` crate is candela.

- Candela

### Mass

The base unit of mass used in the `unitconverter` crate is kilograms.

> [!NOTE]
> The conversion to and from kilograms still necessitates the use of a prefix, for consistency with how the rest of the crate is used. 

- Gram
- Pound

### Temperature

The base unit of temperature used in the `unitconverter` crate is kelvin.

- Celsius
- Delisle
- Fahrenheit
- Kelvin
- Rankine
- Réaumur
- Rømer

### Time

The base unit of time used in the `unitconverter` crate is seconds.

- Day
- Hour
- Minute
- Second