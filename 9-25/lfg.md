# Today's Work
- finish chapter 3

## Function

starts with `fn`

order of function doesn't matter just need to be declared at some place

```
fn some_function() {
    println!("some function);
}

fn main() {
    println!("main");
    another_function();
    some_function();
}

fn another_function() {
    println!("another function")
}
```

every parameter must be annotated with function!

## Statements and expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

Expression could be part of statement

```
let x = 6;
```
6 is expression returnig the value 6.
Calling function is expression. calling macro is expression.
creating a new block is expression.

```
{
    let x = 3;
    x + 1
}
```
expressions don't have semicolon

## functions with return values

return value don't need name.
use `->` to declare type of return value
we can use `return` to return the value but last expression in the block is returned as value

when you put semicolon at the end of expressions the error will occur expression turns into statement and since statement don't return value `()` empty tuple will be returned.

## Control flow

### if
there is `if` you know what it is **but** remeber that condition must be `boolean`. unlike js or ruby rust don't do the magic (change the type to `boolean`)

you can use `if` inside `let` statement.
`if` and `if else` must have same type returned;

```
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six" //  This is WRoNG!
    };

    println!("The value of number is: {}", number);
}
```
Rust can't determine the type when being executed.

### Repetition

there is `loop` you can gtfo of the loop using `break`
but making repetition with loop could be complicated you know why...
(need to mix `if`,`if else`,`break`)

you can use label when using multiple loop.

and there is `while`, you know repeat when condition is `true`

`for`... use it with array or vec anything with iterator. 
used very generally.