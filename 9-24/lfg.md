# Today's work

- go through data types

## variables and mutability

variables are initially immutable
using mut will later can be helpful when reading the code again. (e.g. is this variable constant?)

constant vs variables

constants are born immutable no mut on `const` and const can be declared on any place in code.

## Shadowing

can declare variables declared in first place with same name.
shadowing is different from `mut` variable.
without using `let` and inserting new value to variable will cause compile error.

## data types

every value in rust got data types.
There are two **big** groups of types

- scalar
- compound

**rust is statically typed language**
when compiled every variables must have it's type. but rust compiler is smart it can infer the type :D

### scalar types

- Integer
- floatin-point numbers
- Booleans
- characters

### Integer

numbers without fractional compound(no decimal point)
`u32` : usinged integer 32 bit variable
you know what it means
`isize` and `usize` : depends on computing env (32-bit or 64-bit)
i32 is generally used.

### float-point

f32 vs f64
f64 is standard most cpu is capable of it, and there are no speed difference.
Plus f64 is more precise.

### Booleans

you know what it is

### char

`char` tpye represents unicode scalar.
just use `' '` when using it.

### compound types

complex types in short. can make various types into one tpye.

### Tuples

tuples can have different type as element.
using pattern match user can destruct the tuple
and can even access to element using `.` and index.

### Array

unlike tuple every elements must have same data type.
unlike other languages, arrays in rust have **fixed length**
useful when you want to save data to `stack` rather than `heap`
(just use vector when you are not sure which one to use :D)

 - fun fact about array
 when you do run this code
 trying to access over the array length
 ```
 fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
 ```

 this causes `runtime error` not `compile error`.
 when trying to access array using index
 1. rust checks if index user specified is less than length.
 2. if bigger rust panic
 3. and exits program

 this is first example of memory safety. low-level languages don't usually don't do this kinda check. rust protects you rust loves you