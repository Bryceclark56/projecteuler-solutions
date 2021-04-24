/* Project Euler
 *  Problem 3
 *  
 * Find the largest prime factor of N,
 *  where N is a natural number.
 */

fn main() {
    println!("Project Euler - Problem 3");
    println!("=========================");

    const N: u64 = 600_851_475_143;
    const MAX_SIEVE: u64 = 100_000_000;

    let primes = primes_up_to_n(MAX_SIEVE as usize);

    let mut n = N.clone();
    let mut last_prime = 0;
    let mut r;
    for i in primes.into_iter() {
        r = 0;
        while r == 0 {
            r = n % i;
            if r == 0 {
                n = n / i;
                last_prime = i;
            }
        }
    }

    println!("Should be: {}", last_prime);
}

// Generates prime numbers less than or equal to n
fn primes_up_to_n(n: usize) -> Vec<u64> {
    println!("Generating prime numbers up to N...");

    let mut sieve: Vec<u64>= (0..=n as u64).collect();

    sieve[1] = 0;

    for i in 2..=n {
        if sieve[i] == 0 {continue} // Skip any values already marked non-prime

        for j in ((i+i)..=n).step_by(i) {
            sieve[j] = 0; // Set to 0 if multiple of i (AKA non-prime)
        }
    }

    // Remove the zero values
    let primes = sieve.into_iter()
        .filter(|i| *i != 0)
        .collect::<Vec<u64>>();

    //println!("{:?}", primes);
    println!("Generated primes with sieve");

    primes
}