# Convert Case

Converts to and from various cases.

## Usage

Installing `convert_case` gives you the `ccase` executable.
```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat_cali
2020-04-16 My Cat Cali

$ ccase -t camel "convert to camel"
convertToCamel
```

By default `convert_case` will determine word boundaries based on all hyphens, spaces, underscores, and changes in capitalization.  You can also supply a parsing method by supplying a case with the `--from -f` option for more accuracy.
```
$ ccase -t upper "grimaldi-2003_discrete_pdf"
GRIMALDI 2003 DISCRETE PDF

$ ccase -f kebab -t upper "grimaldi-2003_discrete_pdf"
GRIMALDI 2003_DISCRETE_PDF
```

## Edge Cases

Convert case can handle acroynms.
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
cargo install convert_case
```

## Rust Library

Convert case was written in Rust and is ready to be used inline with your rust code as a library.  You can read the API documentation on [docs.rs](https://docs.rs/convert_case/).

## Cases

You can also view the list of cases using the `--list -l` option.  Some cases are simply aliases of others.

| Case | Example |
| ---- | ------- |
| Upper | MY VARIABLE NAME |
| Lower | my variable name |
| Title | My Variable Name |
| Toggle | mY vARIABLE nAME |
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
| Alternating | mY vArIaBlE nAmE |
