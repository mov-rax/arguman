# arguman
A simple command-line argument parser written in Rust.

All that is needed to use this parser to is to create a new MagicArguman!

An example of utilizing Arguman:

```rust
let args:Vec<String> = env::args().collect();
let mut manager = arguman::MagicArguman::new(args);
manager.input()
    .flag::<u32>("n")
    .flag::<f64>("f")
    .flag_solo("s")
    .flag_req::<String>("name");

let errors:&Vec<MagicErr> = manager.get_errors();
let input:Option<&String> = manager.get_input();
let name:Option<&String> = manager.get::<String>("name");
let number:Option<&u32> = manager.get::<u32>("n");
```
*Now, how does any of that work??*

## Here you go!
In the example above, `arguman::MagicArguman::new(args)` takes in the arguments as a type `Vec<String>`. Make sure
that the name you assign it is *mutable*. Otherwise, it won't be able to do much.

The method `input()` lets the parser know that you want user input without any flags before it. For example, if a 
program with the name *app* is executed like `./app antidisestablishmentarianism`, then `input()` will read and store a
clone of the value `antidisestablishmentarianism` as a *String*, which can later be accessed (read only) by utilizing 
the dedicated method `get_input()`.

The method `flag<T>(&str)` lets the parser know that you want to add an *OPTIONAL* flag. The type _**MUST**_ be
statically assigned at compile-time. The flag can its possible value can be accessed (read-only) by utilizing the
method `get<T>(&str)`. If the two types are not matching, an error will be recorded and a value of `None` will be
returned.

The method `flag_solo(&str)` lets the parser know that you want to add an *OPTIONAL* boolean flag. Unlike the previous
method, no type is required when creating the flag. However, like the previous method, the possible value can be
accessed (read-only) by utilizing the method `get<bool>(&str)`.

The method `flag_req<T>(&str)` is a specialized version of the `flag<T>(&str)` method. The specialness about this
method is that the flag is *NOT OPTIONAL*, and is *REQUIRED*. The value stored in it can be accessed similarly to
the non-specialized variant. In the event that the user does not use the flag, an error will be stored. To check if
any errors have occurred when parsing any required flag, simply use the `error()` method, which will return `true` if
an error has occurred, and `false` if not. If more information regarding errors are desired, they can be accessed
(read-only) through the method `get_errors()`, which will return a `&Vec<MagicErr>`.

For those who wish to know what type of errors transpired, the errors are returned as an enum `MagicErr`, which contains
the following four errors and their pertinent information:
```rust
enum MagicErr{
FlagErr(String),
ParseErr(String),
ValueErr(String),
InputErr
}
```

- `FlagErr` means that a non-optional flag was omitted.
  - It contains a `String` with the omitted flag.
- `ParseErr` means that a type could not be parsed
  - Could mean that you set a different type when setting a flag and getting its value
  - Most probably means that the user could have written `BUSHDID911` instead of a 64-bit float.
- `ValueErr` means that the parser has detected that a value is missing!
  - The user may have written the flag but not a value afterwards
- `InputErr` means that you decided that `input()` was used, but no input was detected by the parser
  - Could not be called but have invalid input (any input is valid input, even a flag).
  
#### Questions/Suggestions?

Don't be shy to ask!

**mov-rax 2020**