# Fraction

Lossless fractions and decimals; drop-in float replacement
------

[![GitHub Actions](https://github.com/dnsl48/fraction/actions/workflows/main.yml/badge.svg?branch=master)](https://github.com/dnsl48/fraction/actions/workflows/main.yml?query=branch%3Amaster) [![Documentation](https://docs.rs/fraction/badge.svg)](https://docs.rs/fraction/) [![Current Version on crates.io](https://img.shields.io/crates/v/fraction.svg)](https://crates.io/crates/fraction/) [![MIT / Apache2 License](https://img.shields.io/badge/license-MIT%20/%20Apache2-blue.svg)]()
------

# Features
 - Fraction type, representing floats as fractions
 - Decimal type, based on Fraction type with explicit precision
 - Fractions are drop-in replacement for floats with the exception for NaN == NaN && NaN < NegInfinity, thus it's hasheable and orderable.
 - Fractions are hashable and orderable, so they may be used in Sets, HashMaps and BTrees.
 - DynaInt type, which is an int dynamically growing into heap. Performs checked math and avoids stack overflows.
 - PostgreSQL integration for Numeric/Decimal type (with no extra memory allocations)
 - Juniper integration for both fractions and decimals
 - Support for generic integer conversions, such as `i8 -> u8`, `usize -> u8` and so on
 - Lossless division with infinite precision and no memory allocations

# Documentation
 Here: [![Documentation](https://docs.rs/fraction/badge.svg)](https://docs.rs/fraction/)

# Examples

## Simple use:

```rust
type F = fraction::Fraction;  // choose the type accordingly with your needs (see prelude module docs)

let two = F::from(0) + 2;   // 0 + 2 = 2
let two_third = two / 3;    // 2/3 = 0.666666[...]

assert_eq!(F::from(2), two);
assert_eq!(F::new(2u64, 3u64), two_third);

assert_eq!("2/3", format!("{}", two_third));  // print as Fraction (by default)
assert_eq!("0.6666", format!("{:.4}", two_third));  // format as decimal and print up to 4 digits after floating point
```

Decimal is implemented as a representation layer on top of Fraction.
Thus, it is also lossless and may require explicit control over "precision"
for comparison and formatting operations.
```rust
type D = fraction::Decimal;  // choose the type accordingly with your needs (see prelude module docs)

let result = D::from(0.5) / 0.3;

assert_eq!(format!("{}", result), "1.6"); // calculation result uses precision of the operands
assert_eq!(format!("{:.4}", result), "1.6666");  // explicitly passing precision to format

assert_eq!("1.6666", format!("{}", result.set_precision(4))); // the other way to set precision explicitly on Decimal
```

## Construct:

Fraction:
```rust
use std::str::FromStr;
use fraction::{Fraction, Sign};

fn main() {
    // There are several ways to construct a fraction, depending on your use case

    let f = Fraction::new(1u8, 2u8);  // constructs with numerator/denominator and normalizes the fraction (finds least common denominator)
    assert_eq!(f, Fraction::new_generic(Sign::Plus, 1i32, 2u8).unwrap());  // with numerator/denominator of different integer types
    assert_eq!(f, Fraction::from(0.5));  // convert from float (f32, f64)
    assert_eq!(f, Fraction::from_str("0.5").unwrap());  // parse a string
    assert_eq!(f, Fraction::from_str("1/2").unwrap());  // parse a string

    // Raw construct with no extra calculations.
    // Most performant, but does not look for common denominator and may lead to unexpected results
    // in following calculations. Only use if you are sure numerator/denominator are already normalized.
    assert_eq!(f, Fraction::new_raw(1u64, 2u64));
}
```

Decimal:
```rust
use std::str::FromStr;
use fraction::{Decimal, Fraction};

fn main() {
    // There are similar ways to construct Decimal. Underneath it is always represented as Fraction.
    // When constructed, Decimal preserves its precision (number of digits after floating point).
    // When two decimals are calculated, the result takes the biggest precision of both.
    // The precision is used for visual representation (formatting and printing) and for comparison of two decimals.
    // Precision is NOT used in any calculations. All calculations are lossless and implemented through Fraction.
    // To override the precision use Decimal::set_precision.

    let d = Decimal::from(1);  // from integer, precision = 0
    assert_eq!(d, Decimal::from_fraction(Fraction::from(1))); // from fraction, precision is calculated from fraction

    let d = Decimal::from(1.3);  // from float (f32, f64)
    assert_eq!(d, Decimal::from_str("1.3").unwrap());

    let d = Decimal::from(0.5);  // from float (f32, f64)
    assert_eq!(d, Decimal::from_str("1/2").unwrap());
}
```

## Format (convert to string)
Formatting works the same for both Decimal and Fraction (Decimal uses Fraction internally).
The format implementation closely follows the rust Format trait documentation.

```rust
type F = fraction::Fraction;

let result = F::from(0.7) / 0.4;
assert_eq!(format!("{}", result), "7/4");
assert_eq!(format!("{:.2}", result), "1.75");
assert_eq!(format!("{:#.3}", result), "1.750");
```

### Generic integer conversion
```rust
use fraction::{Sign, GenericFraction};

type F = GenericFraction<u32>;

let fra = F::new_generic(Sign::Plus, 1i8, 42usize).unwrap();
assert_eq!(fra, F::new(1u32, 42u32));
```

### Postgres usage notes
It is recommended to use Decimal over Fraction for PostgreSQL interactions.
When interacting with PostgreSQL, fraction type keeps the highest achievable precision up to 16383 digits after floating point.
That may lead to suboptimal performance for such values as 1/3 or 1/7.
Decimal has its own explicit precision, so there won't be accidental calculation of tens of thousands digits.

PostgreSQL uses i16 for its binary protocol, so you'll have to use at least u16
as the base type for your GenericFraction/GenericDecimal. However, it is also possible to workaround
via DynaInt<u8, _something_more_than_u8_>.

It is recommended to use `DynaInt<usize, BigUint>` so that by default you have on-stack math, and if necessary
heap memory gets allocated.

Otherwise, both types (fractions and decimals) should work transparently in accordance with Postgres crate documentation.

# Change Log

Look into the [CHANGELOG.md](CHANGELOG.md) file for details
