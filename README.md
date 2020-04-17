# Convert Case

Converts to and from various cases.

```
$ ccase -t title super_mario_64
Super Mario 64

$ ccase -f snake -t title 2020-04-15_my_cat_cali
2020-04-16 My Cat Cali

$ ccase -t camel "convert to case"
convertToCase
```

Convert case will determine word boundaries automatically for any input.  You can also supply a parsing method by supplying a case with the `--from -f` option for more accuracy.

## Rust Library

Convert case was written in Rust and is ready to be used inline with your rust code as a library.

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

## Even more cases?

If you want more cases, they are easy to implement.  Contact me with a suggestion!
