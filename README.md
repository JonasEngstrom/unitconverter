[![Build and Test](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/unitconverter)](https://crates.io/crates/unitconverter)
[![codecov](https://codecov.io/gh/JonasEngstrom/unitconverter/branch/main/graph/badge.svg?token=4R7RT01VHT)](https://codecov.io/gh/JonasEngstrom/unitconverter)

# Unit Converter

Unit converter strives to employ a consistent interface for conversion between units of different dimensions. It is based around the units and the prefixes of the SI system but includes regional and historical units as well.

## Conventions

### Units Enums and Measurements Structs

The crate is built up around two different concepts, unit enums e.g. `LengthUnit` and measurement structs e.g. `LengthMeasurement`—prefixed with a quantity and suffixed with Unit or Measurement, as appropriate.

### Measurement Struct `from` and `to` Methods

Conversion of measurements between diffrent units is handled by the `from` and `to` methods of the measurement structs, with unit enums providing data on conversion factors internally. The conceptual model used is that a measurment is converted from one unit into another.

```rust
use unitconverter::mass::{ MassUnit, MassMeasurement };
use si_prefixes::Prefix;

// To convert five kilograms FROM kilograms TO pounds,
// the measurement is first stored in a variable, using the FROM method
let five_kilograms = MassMeasurement::from(5f64, Prefix::Kilo, MassUnit::Gram);

// and then converted into pounds using the TO method.
let measurement_in_pounds = five_kilograms.to(Prefix::None, MassUnit::Pound);
```

### Prefixes

All units can take a prefix, no prefixed units come as base units, and if no prefix is to be used, this has to be explicitly stated in calculations. The supported prefixes are listed in the [`si-prefixes` crate](https://crates.io/crates/si-prefixes) documentation.

```rust
use unitconverter::length::{ LengthUnit, LengthMeasurement };
use si_prefixes::Prefix;

// Even unit conventionally not used with prefixes can use them.
let one_kiloyard = LengthMeasurement::from(1f64, Prefix::Kilo, LengthUnit::Yard);

// No prefixed units come included. If you want a kilometer, you have to create it.
let one_kilometer = LengthMeasurement::from(1f64, Prefix::Kilo, LengthUnit::Meter);

// If you do not wish to use a prefix, this has to be explicitly stated.
let one_inch = LengthMeasurement::from(1f64, Prefix::None, LengthUnit::Inch);
```

### Numeric Types Used

Numbers used in the crate are stored as `f64`.

> [!WARNING]
> As the crate uses floating point arithmetic, small rounding errors can occur. Although it is not always necessary, rounding results to the desired number of significant digits can be a good idea. No unit tests in the crate tolerate errors exceeding 10<sup>-11</sup>.

### Pluralization

Units are referred to in their singular form, e.g. foot instead of feet.

### Spelling

If a term is spelled differently in different parts of the anglophone world, American spelling is used, e.g. meter instead of metre. This is not a comment on what should be considered correct terminolgy but merely a way to achieve consistency and reduce duplicate terms in the code.

## Supported Units

### Amount of Substance

The base unit of amount of substance used in the `unitconverter` crate is moles.

- Mole

### Area

- The square of any [`LengthUnit`](#length) including a prefix.
    ```rust
    use unitconverter::area::AreaUnit;
    use unitconverter::length::LengthUnit;
    use si_prefixes::Prefix;

    // Creating a custom square unit uses the AreaUnit enum’s Square variant.
    let square_kilometer_unit = AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter);
    let square_inch_unit = AreaUnit::Square(Prefix::None, LengthUnit::Inch);
    ```

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