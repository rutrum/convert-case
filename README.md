# Convert Case

Converts to and from various cases.

## Rust Library `convert_case`

Convert case was written in Rust and is ready to be used inline with your rust code as a library.
```rs
use convert_case::{Case, Casing};

assert_eq!("ronnieJamesDio", "Ronnie_James_dio".to_case(Case::Camel));
assert_eq!("io_stream", "IOStream".to_case(Case::Snake));
assert_eq!(
    "2020-04-16 My Cat Cali",
    "2020-04-16_my_cat_cali".from_case(Case::Snake).to_case(Case::Title)
);
```
You can read the API documentation on [docs.rs](https://docs.rs/convert_case/) for a list of all features and read lots of examples.

## Command Line Utility `ccase`

The command line utility `ccase` was made to leverage the tools in the `convert_case` library.
```sh
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat_cali
2020-04-16 My Cat Cali

$ ccase -t camel "convert to camel"
convertToCamel
```

You can read more about the `ccase` executable in the [`ccase` directory](https://github.com/rutrum/convert-case/tree/master/ccase) within this repository.

## Node.js package `node-convert-case`

A Node.js library made with bindings to convert_case. It leverages [neon](https://neon-bindings.com/) to accomplish this.
```js
import { Case, Boundary, Pattern, CS } from "node-convert-case";

// Using toCase
let marioTitle: string = CS("super_mario_64").toCase(Case.Title).toString();
assert("Super Mario 64" === marioTitle);

// Using toCase with the optional argument 'fromCase'
marioTitle = CS("super_mario_64").toCase(Case.Title, Case.Lower).toString();
assert("Super_mario_64" === marioTitle);

// Using isCase
let pascalStr = "ExceptionHandler";
assert(CS(pascalStr).isCase(Case.Pascal));

// Using mutate
let characterCode: string = CS("567N9854G321K").mutate(
    {
        boundaries: [Boundary.UpperDigit],
        delim: "-",
        pattern: Pattern.Lowercase
    }
).toString();
assert("567n-9854g-321k" === characterCode);
```

## Links

| | `convert_case` | `ccase` | `node-convert-case` |
| --- | --- | --- | --- |
| Repository | [github](https://github.com/rutrum/convert-case) | [github](https://github.com/rutrum/convert-case/tree/master/ccase) | [github](https://github.com/Wild-W/convert-case) |
| Library | [crates.io](https://crates.io/crates/convert_case) | [crates.io](https://crates.io/crates/ccase) | [npmjs.com](https://www.npmjs.com/package/node-convert-case) |
| Documentation | [docs.rs](https://docs.rs/convert_case) | | |

## Cases

This is list of cases that convert\_case supports.  Some cases are simply aliases of others.  The "Random" and "PseudoRandom" cases are provided in the `convert_case` library with the "random" feature, and are automatically provided in the `ccase` binary.

| Case | Example |
| ---- | ------- |
| Upper | MY VARIABLE NAME |
| Lower | my variable name |
| Title | My Variable Name |
| Toggle | mY vARIABLE nAME |
| Alternating | mY vArIaBlE nAmE |
| Camel | myVariableName |
| Pascal | MyVariableName |
| UpperCamel | MyVariableName |
| Snake | my\_variable\_name |
| UpperSnake | MY\_VARIABLE\_NAME |
| ScreamingSnake | MY\_VARIABLE\_NAME |
| Kebab | my-variable-name |
| Cobol | MY-VARIABLE-NAME |
| Train | My-Variable-Name |
| Flat | myvariablename |
| UpperFlat | MYVARIABLENAME |
| Random | MY vaRiabLe nAME |
| PseudoRandom | mY VaRiAblE nAMe |

## License

Licensed under [MIT License](./LICENSE).
