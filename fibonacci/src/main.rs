use std::time::Instant;

use num::{BigUint, One, Zero};

fn fibonacci(n: u128) -> BigUint {
    if BigUint::from(n) == BigUint::zero() || BigUint::from(n) == BigUint::one() {
        return BigUint::from(n);
    }

    let mut next_fib = BigUint::one();
    let mut fib_sequence: Vec<BigUint> = vec![BigUint::zero(), BigUint::one()];

    for _ in 2..=n {
        next_fib = BigUint::from(
            &fib_sequence[fib_sequence.len() - 2] + &fib_sequence[fib_sequence.len() - 1],
        );
        fib_sequence.push(next_fib.clone());
    }

    next_fib
}

fn fibonacci_recursive(n: u128) -> BigUint {
    if BigUint::from(n) == BigUint::zero() || BigUint::from(n) == BigUint::one() {
        return BigUint::from(n);
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn main() {
    let start = Instant::now();
    let n = 25;
    println!(
        "{} Fibonacci is {} and time taken is {:?}",
        n,
        fibonacci(n),
        start.elapsed()
    );

    let start = Instant::now();
    println!(
        "{} Fibonacci recursive is {} and time taken is {:?}",
        n,
        fibonacci_recursive(n),
        start.elapsed()
    );
}
