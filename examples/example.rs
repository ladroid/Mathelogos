extern crate mathelogos;
use mathelogos::*;

// Example functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

// Using the framework
fn main() {
    let add_five = curry!(add)(5);
    println!("5 + 3 = {}", add_five(3));

    let add_two_and_mul_by_three = compose!(|x| mul(x, 3), |x| add(x, 2));
    println!("(2 + 2) * 3 = {}", add_two_and_mul_by_three(2));

    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers = map!(|&x| x * x, &numbers);
    println!("Squares: {:?}", squared_numbers);

    let sum = foldl!(|acc, &x| acc + x, 0, &numbers);
    println!("Sum: {}", sum);

    // List comprehension example
    let evens_squared = list_comprehension!(x * x, x <- &numbers, x % 2 == 0);
    println!("Even squares: {:?}", evens_squared);

    // Lazy evaluation example
    let mut lazy_value = Thunk::new(|| {
        println!("Evaluating...");
        add(5, 10)
    });
    println!("Lazy value not evaluated yet.");
    println!("Lazy value: {}", lazy_value.evaluate());
    println!("====================================================================");
    // Using Maybe monad
    let value = Maybe::Just(5);
    let result = value.map(|x| x * 2);
    if let Maybe::Just(v) = result {
        println!("Result: {}", v); // Output: Result: 10
    }

    // Using filter and reduce
    let numberss = vec![1, 2, 3, 4, 5];
    let even_numbers = filter!(|x| x % 2 == 0, &numberss);
    println!("Even Numbers: {:?}", even_numbers); // Output: Even Numbers: [2, 4]
    
    let product = reduce!(|acc, x| acc * x, 1, &even_numbers);
    println!("Product of even numbers: {}", product); // Output: Product of even numbers: 8

    // Example using match_with macro
    let number = 3;
    let description = match_with!(number, {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Something else",
    });
    println!("Description: {}", description); // Output: Description: Three
    println!("====================================================================");
    // Immutable list example
    let list = ImmutableList::new().prepend(3).prepend(2).prepend(1);
    if let Maybe::Just(&first) = list.head() {
        println!("First element: {}", first); // Output: First element: 1
    }

    // Using zipWith
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6];
    let zipped_sum = zip_with!(add, &numbers1, &numbers2);
    println!("Zipped Sum: {:?}", zipped_sum); // Output: Zipped Sum: [5, 7, 9]
    println!("====================================================================");
    // Using lambda
    let lambda_example = lambda!(x, y => x + y);
    println!("Lambda result: {}", lambda_example(2, 3));
    // Using foldl
    let sum = foldl!(|acc, &x| acc + x, 0, &numbers);
    println!("Foldl Sum: {}", sum); // Output: Sum: 15
    println!("====================================================================");
    let numbers = vec![1, 2, 3, 4, 5];
    let primes = vec![2, 3, 5];
    let product_primes = list_comprehension!(x * y, x <- &numbers, x % 2 == 0, y <- &primes, y > 2);
    println!("Products of even numbers and primes > 2: {:?}", product_primes);
}
