# Splax language

Interpreter for Splax, A dynamic programming language I made while learning how compilers and interpreters work.

## Splax Language Documentation


### Hello, world!
A simple hello world program in splax:
```python
print "Hello, World!";
```
Semi-colons at the end of every line is mandatory in Splax.

### Datatypes
Splax has following datatypes

#### Numbers
These can number literals which can be both integers and floating point numbers.

examples: `1`, `2.5`, `9`

#### Strings
These are string literals defined inside `"`

examples: `"Splax"`, `"Strings are easy"`

### Booleans
These are boolean literals which can be either `true` or `false`.

examples: `true`, `false`

### Nulls
Splax has nulls, the trillion dollar mistale. It can be defined using the `null` keyword. All uninitialized variables are given the value of `null`.

examples: `null`



### Operators.
Splax has following operators:
#### Assignment
`=` - equals

#### Unary operators
`-` - Unary negation

### Logical operators
`and` - logical AND

`or`  - logical OR

`!`   - logical NOT


#### Arithmetic operators
`+` - sum

`-` - difference

`*` - product 

`/` - division 

`%` - mod


#### Comparison operators
`==` - is equals

`!=` - is not equals

`>`  - is less than

`>=` - is less than or equals

`>`  - is greater than

`>=` - is greater than or equals




### Comments
Splax has only one type of comment, single line comments, which can be defined using `//` at the beginning of a line.

```c
// This is a comment.
// The Lexer completely ignores any line starting with //
// The Whole line is ignored.
```

### Variables
Splax has variables which can be defined using the `let` keyword without defining any data type, splax can automatically detect datatype at runtime.

syntax:
```rust
let variable_name;
let variable_name = initial_value;
```

example:
```rust
let a; // default value is null if nothing is assigned.
let b = 2; // numbers: both integers
let c = 2.5; // and floats
let d = "Strings are easy"; // strings
let e = true; // booleans
```


### Scope
Splax variables have scope like any other modern programming language (the term `modern` here can be understood as the same as modern in `modern chess`)

```rust
let a = 1;
{
    let a = 2;
    print a; // 2
}
print a; // 1
```


### Conditionals
Splax has `if` `else` conditionals. It can be defined using the following syntax:
```c
if (condition) {
    // todo
} else {
    // else todo
}
```

example: 
```rust
let a = 1;
if (a == 1) {
    print "A is infact 1";
} else {
    print "A is not 1";
}
```

### While loop
while loops in splax can be defined using the following syntax:
```c
while (condition) {
    // todo
}
```
example:
```rust
let a = 10;
while (a > 1) {
    print a;
    a = a - 1;
}
```

### For loops
Splax has syntactic sugar for `for` loops.

syntax:
```c
for(initialiser; condition; incrementer) {
    // todo
}
```
example:

```c
for(let i = 0; i < 10; i = i + 1) {
    print i;
}
```


### Functions
Splax have user defined functions, and ability to call them.
#### Function declaration
A function in splax can be defined using the following syntax:
```rust
fn function_name(parameters) {
    // todo
}
```
example:
```rust
fn greet(name) {
    print "Hello " + name;
}
```

#### Calling functions
A function can be called using the following syntax:
```c
function_name(parameters);
```
example
```c
greet("Splax");
```

## Usage

Run the executable for the help message.
```
A learning programming language.

Usage: splax <COMMAND>

Commands:
  repl  Interactive repl
  run   Run from a file
  docs  See docs
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```



## Building from source.
1. Clone the repo and make sure you have installed rust compiler and cargo. Here's the [official installation guide](https://rustup.rs/).

2. Build using cargo.
```sh
cargo build --release
```

You can test the interpreter on example present in [/examples](https://github.com/prashantrahul141/splax/tree/main/examples) folder