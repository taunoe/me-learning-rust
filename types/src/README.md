# Basic Types

- **booleans** - bool for representing true/false
- **unsigned integers** - u8 u16 u32 u64 u128 for representing nonnegative whole numbers
- **signed integers** - i8 i16 i32 i64 i128 for representing whole numbers
- **pointer sized integers** - usize isize for representing indexes and sizes of things in memory
- **floating point** - f32 f64
- **tuple** - (value, value, ...) for passing fixed sequences of values on the stack
- **arrays** - [value, value, ...] a collection of similar elements with fixed length known at compile time
- **slices** - a collection of similar elements with length known at runtime
- **str(string slice)** - text with a length known at runtime

## Basic Type Conversion

Rust makes numeric type conversions very easy with the as keyword

## Constants

Instead of copying values like variables where they are used, constants directly replace the text identifier where they are used with their value at compile time.

Constant names are always in SCREAMING_SNAKE_CASE