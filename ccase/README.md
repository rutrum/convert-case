# `ccase`

A command line utility to convert to and from various cases.  `ccase` is short for "convert case."

## Usage

Once installed, you can start converting strings to any case.
```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat_cali
2020-04-16 My Cat Cali

$ ccase -t camel "convert to camel"
convertToCamel
```

By default `ccase` will determine word boundaries based on all hyphens, spaces, underscores, and changes in capitalization.  You can also supply a parsing method by supplying a case with the `--from -f` option for more accuracy.
```
$ ccase -t upper "grimaldi-2003_discrete_pdf"
GRIMALDI 2003 DISCRETE PDF

$ ccase -f kebab -t upper "grimaldi-2003_discrete_pdf"
GRIMALDI 2003_DISCRETE_PDF
```

## Use Case

This binary was written with a very specific use case in mind (so the author could do a cool one-liner.)  Given a list of file names in snake case
```
$ ls ~/roms
donkey_kong_64.z64
kirby_64_the_crystal_shards.z64
super_mario_64.z64
```
write out a list of nicely formatted title-case game titles.  After removing the file extension using cut, with `ccase` one can do the following.
```
$ ls ~/roms | cut -d '.' -f 1 | ccase -f snake -t title
Donkey Kong 64
Kirby 64 The Crystal Shards
Super Mario 64
```

## Edge Cases

`ccase` can handle acroynms.
```
$ ccase -t snake IOStream
io_stream
```
It also ignores leading, tailing, and duplicated delimeters.
```
$ ccase -t kebab __my  bad-_variable- 
my-bad-variable
```
Any special characters are also ignored.
```
$ ccase -t screamingsnake "10,000 Days"
10,000_DAYS
```
Unicode support!
```
$ ccase -t pascal "granat-äpfel"
GranatÄpfel
```

## Install

You need `cargo` to install this utility.  You can get cargo from
```
curl https://sh.rustup.rs -sSf | sh
```
Once cargo is installed,
```
cargo install ccase
```

## Rust Library `convert_case`

`ccase` was written in Rust and is ready to be used inline with your rust code as a library.  You can read the `convert_case` documentation on [docs.rs](https://docs.rs/convert_case/).

## Cases

You can also view the list of cases using the `--list -l` option.  Some cases are simply aliases of others.

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

## Links

| | `convert_case` | `ccase` |
| --- | --- | --- |
| Repository | [github](https://github.com/rutrum/convert-case) | [github](https://github.com/rutrum/convert-case/tree/master/ccase) |
| Crate | [crates.io](https://crates.io/crates/convert_case) | [crates.io](https://crates.io/crates/ccase) |
| Documentation | [docs.rs](https://docs.rs/convert_case) | |
