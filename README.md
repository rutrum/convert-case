# `convert-case`

Rust library for converting between string cases.

```{rust}
use convert_case::ccase;

assert_eq!(
    ccase!(camel, "My_Var_Name"),
    "myVarName",
);
assert_eq!(
    ccase!(snake, "IOStream"),
    "io_stream",
);
assert_eq!(
    ccase!(snake -> title, "2020-04-16_family_photo"),
    "2020-04-16 Family Photo",
);
```

`convert-case` is highly customizable.  You can read the API documentation on [docs.rs](https://docs.rs/convert_case/) for a list of all features and read lots of examples.

## Cases

This is list of cases that `convert_case` provides out of the box.  You can always make your own custom case.

| Case | Example |
| ---- | ------- |
| Snake | `my_variable_name` |
| Constant<br />UpperSnake | `MY_VARIABLE_NAME` |
| Ada | `My_Variable_Name` |
| Kebab | `my-variable-name` |
| Cobol<br />UpperKebab | `MY-VARIABLE-NAME` |
| Train | `My-Variable-Name` |
| Flat | `myvariablename` |
| UpperFlat | `MYVARIABLENAME` |
| Pascal<br />UpperCamel | `MyVariableName` |
| Camel | `myVariableName` |
| Upper | `MY VARIABLE NAME` |
| Lower | `my variable name` |
| Title | `My Variable Name` |
| Sentence | `My variable name` |

## Additional utilities with `convert_case_extras`

Some cases and utilities that didn't feel appropriate in this library are made available in a distinct crate called [`convert_case_extras`](https://github.com/rutrum/convert-case-extras).  This crate is a demonstration of what can be built on top of the `convert_case` API.

## Command Line Utility `ccase`

The [command line utility `ccase`](https://github.com/rutrum/ccase) was made to expose the tools of the `convert_case` library to the command line.
```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat
2020-04-16 My Cat

$ ccase -t camel "convert to camel"
convertToCamel
```

## Links

| | `convert_case` | `convert_case_extras` | `ccase` |
| --- | --- | --- | --- |
| Repository | [github](https://github.com/rutrum/convert-case) | [github](https://github.com/rutrum/convert-case-extras) | [github](https://github.com/rutrum/ccase) |
| Crate | [crates.io](https://crates.io/crates/convert_case) | [crates.io](https://crates.io/crates/convert_case_extras) | [crates.io](https://crates.io/crates/ccase) |
| Documentation | [docs.rs](https://docs.rs/convert_case) | [docs.rs](https://docs.rs/convert_case_extras) | |

## Change Log

### 0.12.0: Optimizations

This release improves speed and memory efficiency.  A couple techniques were used:

* Character case checks first check ascii before doing the more expensive grapheme checks (which require heap allocations).
* Delimiter only splits get a dedicated path in the `split` method.
* The case of characters is computed upfront in a bitmask before checking individual boundary conditions in `split`.  Built-in boundaries get a custom implementation built on top of this bitmask.

| Benchmark | Before | After | Improvement |
| --- | --- | --- | --- |
| `snake_short` (`"hello_world"`) | 412 ns | 21.3 ns | −95% |
| `lower_upper` (`"lowerUpperUpper"`) | 1,631 ns | 475 ns | −71% |
| `acronym` (`"XMLRequest"`) | 1,108 ns | 418 ns | −62% |
| `camel_set` (`"getTotalLength3D"`) | 3,844 ns | 643 ns | −83% |
| `defaults_mixed` (9 boundaries) | 8,083 ns | 1,500 ns | −81% |
| `defaults_long_snake` (265 chars) | 52,281 ns | 4,730 ns | −91% |
| `unicode_cyrillic` (`"ПЕРСПЕКТИВА24"`) | 8,334 ns | 741 ns | −91% |
| Full pipeline `from_to_all` (4 words x 14 x 14) | 1,230 µs | 263 µs | −79% | 

See [CHANGELOG.md](CHANGELOG.md) for the full history.
