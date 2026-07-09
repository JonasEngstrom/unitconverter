[![Build and Test](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JonasEngstrom/unitconverter/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/unitconverter)](https://crates.io/crates/unitconverter)
[![codecov](https://codecov.io/gh/JonasEngstrom/unitconverter/branch/main/graph/badge.svg?token=4R7RT01VHT)](https://codecov.io/gh/JonasEngstrom/unitconverter)

# Unit Converter

Unit converter strives to employ a consistent interface for conversion between units of different dimensions. It is based around the units and the prefixes of the SI system but includes regional and historical units as well.

## Usage

This section gives a few examples on how to use the crate. For a more detailed description of the underlying design, see the [Convetions](#conventions) section below. Supported units are listed in the [Supported Units](#supported-units) section. Supported prefixes are listed in the [`si-prefixes` crate](https://crates.io/crates/si-prefixes) documentation.

Usage of the crate revolves around storing measurements of different quantities in variables. The stored measurements can then either be converted to other units or used in arithmetic operations, that may yield measurements in other dimensions—e.g. a length measurement times another length measurement becomes an area measurement.

### Storing a Measurement in a Variable

When storing a measurement, the `::from()` method is used.

```rust
use unitconverter::mass::{ MassUnit, MassMeasurement };
use si_prefixes::Prefix;

let one_pound = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);
```

### Converting a Stored Measurement to Another Unit

When converting a stored variable to another unti, the `.to()` method is used.

```rust
use unitconverter::mass::{ MassUnit, MassMeasurement };
use si_prefixes::Prefix;

let one_pound = MassMeasurement::from(1f64, Prefix::None, MassUnit::Pound);

let one_pound_in_kilograms = one_pound.to(Prefix::Kilo, MassUnit::Gram);

assert_eq!(one_pound_in_kilograms, 0.453_592_37f64);
```

### Derived Units

Some units are derived from other units. Which ones are described in the [Supported Units](#supported-units) section.

```rust
use unitconverter::length::{ LengthUnit };
use unitconverter::area::{ AreaUnit, AreaMeasurement };
use si_prefixes::Prefix;

let thousand_square_meters = AreaMeasurement::from(
    1_000f64,
    Prefix::None,
    AreaUnit::Square(Prefix::None, LengthUnit::Meter)
);

let hectares = thousand_square_meters.to(Prefix::Hecto, AreaUnit::Are);

assert_eq!(hectares, 0.1f64);
```

### Arithmetic

#### Sums and Differences

All measurements support addition and subtracts, as well as the `AddAssign` and the `SubAssign` operators.

##### Addition

```rust
use unitconverter::time::{ TimeUnit, TimeMeasurement };
use si_prefixes::Prefix;

let two_minutes = TimeMeasurement::from(2f64, Prefix::None, TimeUnit::Minute);
let sixty_seconds = TimeMeasurement::from(60f64, Prefix::None, TimeUnit::Second);

let three_minutes = two_minutes + sixty_seconds;

assert_eq!(three_minutes.to(Prefix::None, TimeUnit::Minute), 3f64);
```

```rust
use unitconverter::time::{ TimeUnit, TimeMeasurement };
use si_prefixes::Prefix;

let mut a_number_of_minutes = TimeMeasurement::from(2f64, Prefix::None, TimeUnit::Minute);
let sixty_seconds = TimeMeasurement::from(60f64, Prefix::None, TimeUnit::Second);

a_number_of_minutes += sixty_seconds;

assert_eq!(a_number_of_minutes.to(Prefix::None, TimeUnit::Minute), 3f64);
```

##### Subtraction

```rust
use unitconverter::time::{ TimeUnit, TimeMeasurement };
use si_prefixes::Prefix;

let two_minutes = TimeMeasurement::from(2f64, Prefix::None, TimeUnit::Minute);
let sixty_seconds = TimeMeasurement::from(60f64, Prefix::None, TimeUnit::Second);

let one_minute = two_minutes - sixty_seconds;

assert_eq!(one_minute.to(Prefix::None, TimeUnit::Minute), 1f64);
```

```rust
use unitconverter::time::{ TimeUnit, TimeMeasurement };
use si_prefixes::Prefix;

let mut a_number_of_minutes = TimeMeasurement::from(2f64, Prefix::None, TimeUnit::Minute);
let sixty_seconds = TimeMeasurement::from(60f64, Prefix::None, TimeUnit::Second);

a_number_of_minutes -= sixty_seconds;

assert_eq!(a_number_of_minutes.to(Prefix::None, TimeUnit::Minute), 1f64);
```

#### Products and Quotients

Some units support multiplication and division. This changes the dimension, and hence the type, of the measurement. Therefore the `AddAssign` and the `SubAssign` operators are not supported.

##### Multiplication

```rust
use unitconverter::length::{ LengthUnit, LengthMeasurement };
use unitconverter::area::{ AreaUnit, AreaMeasurement };
use si_prefixes::Prefix;

let one_hundred_meters = LengthMeasurement::from(100f64, Prefix::None, LengthUnit::Meter);
let one_tenth_of_a_kilometer = LengthMeasurement::from(0.1f64, Prefix::Kilo, LengthUnit::Meter);

let one_hectare = one_hundred_meters * one_tenth_of_a_kilometer;

assert_eq!(one_hectare.to(Prefix::Hecto, AreaUnit::Are), 1f64);
```

##### Division

```rust
use unitconverter::length::{ LengthUnit, LengthMeasurement };
use unitconverter::area::{ AreaUnit, AreaMeasurement };
use si_prefixes::Prefix;

let one_hectare = AreaMeasurement::from(1f64, Prefix::Hecto, AreaUnit::Are);
let one_tenth_of_a_kilometer = LengthMeasurement::from(0.1f64, Prefix::Kilo, LengthUnit::Meter);

let one_hundred_meters = one_hectare / one_tenth_of_a_kilometer;

assert_eq!(one_hundred_meters.to(Prefix::None, LengthUnit::Meter), 100f64);
```

## Conventions

### Unit Enums and Measurement Structs

The crate is built up around two different concepts, unit enums e.g. `LengthUnit` and measurement structs e.g. `LengthMeasurement`—prefixed with a quantity and suffixed with Unit or Measurement, as appropriate. The names are written with their initial letters capitalized (i.e. in “[camel case](https://en.wikipedia.org/wiki/Camel_case)”).

### Module Names

The module containing the [Unit Enum and Measurement Struct](#unit-enums-and-measurement-structs) pertaining to a quantity is named after the quantity in lower case with underscores separating the words (i.e. in “[snake case](https://en.wikipedia.org/wiki/Snake_case)”).

```rust
use unitconverter::luminous_intensity::{ LuminousIntensityUnit, LuminousIntensityMeasurement };
```

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

#### Double Prefixes

As the [crate convention stipulates that all units can take a prefix](#prefixes) and certain units can be derived from other units (e.g. [area as a square of a length unit](#area)) which can in turn take its own prefix, some units can end up with two prefixes in the same unit. While not customary, double prefixing means more discrete units can be handled by the crate, as in the following example.

```rust
use si_prefixes::Prefix;
use unitconverter::length::LengthUnit;
use unitconverter::area::{ AreaUnit, AreaMeasurement };

// One square meter is equal to one square meter.
let one_square_meter = AreaMeasurement::from(
    1f64,
    Prefix::None,
    AreaUnit::Square(Prefix::None, LengthUnit::Meter)
);
assert_eq!(
    one_square_meter.to(
        Prefix::None,
        AreaUnit::Square(Prefix::None, LengthUnit::Meter)
    ),
    1f64
);

// One square kilometer is equal to 1,000,000 square meters.
let one_square_kilometer = AreaMeasurement::from(
    1f64,
    Prefix::None,
    AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter)
);
assert_eq!(
    one_square_kilometer.to(
        Prefix::None,
        AreaUnit::Square(Prefix::None, LengthUnit::Meter)
    ),
    1_000_000f64
);

// One kilo square meter is equal to 1,000 square meters.
let one_kilo_square_meter = AreaMeasurement::from(
    1f64,
    Prefix::Kilo,
    AreaUnit::Square(Prefix::None, LengthUnit::Meter)
);
assert_eq!(
    one_kilo_square_meter.to(
        Prefix::None,
        AreaUnit::Square(Prefix::None, LengthUnit::Meter)
    ),
    1_000f64
);

// One kilo square kilometer is equal to 1,000,000,000 square meters.
let one_kilo_square_kilometer = AreaMeasurement::from(
    1f64,
    Prefix::Kilo,
    AreaUnit::Square(Prefix::Kilo, LengthUnit::Meter)
);
assert_eq!(
    one_kilo_square_kilometer.to(
        Prefix::None,
        AreaUnit::Square(Prefix::None, LengthUnit::Meter)
    ),
    1_000_000_000f64
);
```

### Numeric Types Used

Numbers used in the crate are stored as `f64`.

> [!WARNING]
> As the crate uses floating point arithmetic, small rounding errors can occur. Although it is not always necessary, rounding results to the desired number of significant digits can be a good idea. No unit tests in the crate tolerate errors exceeding 10<sup>-11</sup>.

### Pluralization

Units are referred to by their name in singular form, e.g. foot instead of feet.

### Spelling

If a term is spelled differently in different parts of the anglophone world, American spelling is used, e.g. meter instead of metre. This is not a comment on what should be considered correct terminolgy but merely a way to achieve consistency and reduce duplicate terms in the code.

## Supported Units

### Amount of Substance

The base unit of amount of substance used in the `unitconverter` crate is moles.

- Mole

### Area

- Are
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

### Volume

- Liter
- The cube of any [`LengthUnit`](#length) including a prefix.
    ```rust
    use unitconverter::volume::VolumeUnit;
    use unitconverter::length::LengthUnit;
    use si_prefixes::Prefix;

    // Createing a custom cubic unit uses the VolumeUnit enum’s Cubic variant.
    let cubic_kilometer_unit = VolumeUnit::Cubic(Prefix::Kilo, LengthUnit::Meter);
    let cubic_inch_unit = VolumeUnit::Cubic(Prefix::None, LengthUnit::Inch);
    ```