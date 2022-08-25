# Common Programming Concepts

## Variables and Mutability

- By default, variables in Rust are immutable. So, when we want to declare a mutable variable, we need to explicitly tell the compiler that the following variable is a mutable one;

### Mutable variables

- Are defined by using the `mut` keyword
- Names in camel case, by convention.

### Constants

- Are defined by using the `const` keyword.
- Names in uppercase, by convention.
- May only be a constant expression, not the result of a value computed at runtime.
- Are valid for the entire program lifetime, within the scope they were declared.

### Shadowing

- Are defined by using only the `let` keyword.
- By definition, is the act of declaring a variable with the same name as a previously declared one. The first variable is then **shadowed** by the second one.
- The difference between shadowing and mutation is the with shadowing, we are creating a new variable in memory, instead of reusing it.
- Since we are creating a new variable instead of mutating an already created one, we can also use the same name for differently typed variables.

## Data Types

- Rust is a statically typed language, meaning that we must declare the types of all variables to be known at compile time.
- Sometimes many types are possible and the compiler is unable to infer the type we want to use. In that case, we need to explicitly declare the type.

### Scalar Types

- Represents a single value.

#### Integer Types

- Is a number without a fractional component.
- Rust defaults integer types to `i32`.
- It's declaration follows the `i` or `u` letter to signal wheter is a signed or unsigned integer, followed by the amount of bits it takes in space, like `i32` or `u64`.

  |  Length | Signed | Unsigned |
  |:-------:|:------:|:--------:|
  |  8-bit  |   i8   |    u8    |
  |  16-bit |   i16  |    u16   |
  |  32-bit |   i32  |    u32   |
  |  64-bit |   i64  |    u64   |
  | 128-bit |  i128  |   u128   |
  |   arch  |  isize |   usize  |

- Each signed variable can store numbers from -(2^(n-1)) to 2^n-1.
- Each unsigned variable can store numbers up to 2^n - 1.
- The `arch` length defined by `isize` and `usize` depend on the architecture of the system, like 32-bit and 64-bit systems.
- Can be defined using any integer literal method as the below table. You may also use `_` as a visual separator to make the number easier to read, like `1_000` that becomes `1000`.

  | Number literals |   Example   |
  |:---------------:|:-----------:|
  | Decimal         | 98_222      |
  | Hex             | 0xff        |
  | Octal           | 0o77        |
  | Binary          | 0b1111_0000 |
  | Byte (u8 only)  | b'A'        |

- In the case of integer overflow, the program might either `panic!` or wrap around (only with the `--release` flag), transforming 256 in `u8` for example in 0, or 257 in 1. In either case, is better to handle possible overflows.

#### Floating-Point Types (IEEE-754 compliant)

- Rust uses the `f32` and `f64`, the latter by default, to represent floating point numbers.
- `f32` has single-precision and `f64` has double precision.
- All floating points types are signed.

#### Numeric Operations

- Rust supports all the basic mathematical operations: addition `(+)` subtraction `(-),` multiplication `(*),` division `(/)` and remainder, or modulus `(%)`.

#### Boolean Type

- Specified by using the `bool` keyword.
- A Rust in boolean is pretty straightforward: `true` or `false` and are one byte in size.

#### Character Type

- Is represented by the `char` keyword, and consists in a single character, that might be ASCII, accented letters, and unicode.
- Like in C, when we declare a `char` variable, it might be wrapped only in single quotes.

### Compound Types

- Can group multiple values in one type. Rust has two compound types: `tuples` and `arrays`.

#### Tuple Type

- Is a general way to group together a number of valyes that may vary in types.
- Has a fixed length, meaning that once declared, they cannot grow or shrink in size.
- The types of all components might or might not be explicitly declared.
- Might be destructured using `(val1, val2, val3)`
- To access an element by index, we use the `.` followed by the index, like `x.1` or `y.2`.

#### Array Type

- In Rust, unlike other languages, arrays have fixed length.
- Unlike tuples, all elements in an array must have the same type.
- Allocates data in the `stack` and not in the `heap`.
- Might have the type declared or infered by the compiler.
- To access array elements, we use the square bracket notation `arr[index]`.
- When an out-of-bounds value is accessed, the compiler exits with a `panic!`.