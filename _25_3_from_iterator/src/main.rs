fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares: Vec<_> = primes.into_iter().map(|p| p * p).collect();
    println!("prime_squares: {prime_squares:?}");
}
