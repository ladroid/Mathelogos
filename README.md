# Mathelogos

## Introduction

The Mathelogos library is designed to introduce functional programming constructs to Rust, a language traditionally known for its systems programming capabilities and performance. By leveraging macros, Mathelogos mimics the expressiveness and conciseness of a functional language like Haskell within the Rust ecosystem. This document explains the functional constructs provided by Mathelogos, compares them with their Haskell counterparts, and elaborates on the motivation and goals of this library.

## Goals of Mathelogos

The primary goal of Mathelogos is to facilitate functional programming paradigms in Rust, enhancing the language's capabilities for developers familiar with functional programming. This includes introducing concepts like higher-order functions, lazy evaluation, and monads in a way that integrates seamlessly with Rust's own features like ownership and type safety. By doing so, Mathelogos aims to combine Rust's performance and safety with the expressive power of functional programming.

## Functional Constructs in Mathelogos

Mathelogos introduces several macros to emulate functional programming constructs:

- **lambda!**: Creates anonymous functions, akin to lambda expressions in Haskell.
- **compose!**: Combines two functions into a single operation, similar to function composition in Haskell.
- **map!**: Transforms the elements of a list, corresponding to Haskell's `map`.
- **curry!**: Transforms a function taking multiple arguments into a sequence of functions each taking a single argument.
- **foldl!**: Implements a left fold, similar to Haskell's `foldl`.
- **list_comprehension!**: Provides a syntax for filtering and mapping collections, analogous to Haskell's list comprehensions.
- **filter!**: Filters elements of a collection based on a predicate, akin to Haskell's `filter`.
- **reduce!**: Combines elements of a collection using a binary function, similar to Haskell's `foldl` but often referred to as `reduce` in other languages.
- **match_with!**: A pattern matching construct, simplifying complex conditional logic.

## Comparison with Haskell

To illustrate the usage of Mathelogos and its comparison with Haskell, consider the following examples:

### Lambda and Compose

**Rust with Mathelogos:**

```rust
let lambda_example = lambda!(x, y => x + y);
let add_two_and_mul_by_three = compose!(|x| mul(x, 3), |x| add(x, 2));
```

**Haskell:**

```haskell
lambdaExample = \x y -> x + y
addTwoAndMulByThree = (\x -> mul x 3) . (\x -> add x 2)
```

### Map and Fold

**Rust with Mathelogos:**

```rust
let squared_numbers = map!(|&x| x * x, &numbers);
let sum = foldl!(|acc, &x| acc + x, 0, &numbers);
```

**Haskell:**

```haskell
squaredNumbers = map (\x -> x * x) numbers
sum = foldl (+) 0 numbers
```

### List Comprehension and Lazy Evaluation

**Rust with Mathelogos:**

```rust
let evens_squared = list_comprehension!(x * x, x <- &numbers, x % 2 == 0);
let mut lazy_value = Thunk::new(|| { add(5, 10) });
```

**Haskell:**

```haskell
evensSquared = [x * x | x <- numbers, even x]
lazyValue = let value = add 5 10 in value
```

### Monadic Operations

**Rust with Mathelogos:**

```rust
let result = Maybe::Just(5).map(|x| x * 2);
```

**Haskell:**

```haskell
result = Just 5 >>= \x -> return (x * 2)
```

## License

Mathelogos is licensed under Apache license version 2.0. See [LICENSE](LICENSE) file.
