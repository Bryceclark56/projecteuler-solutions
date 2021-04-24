/* Project Euler
 *  Problem 2
 *  
 * Find the sum of the even valued Fibonacci terms up to N,
 *  where N is a natural number.
 */

fn main() {
    const N: u64 = 4_000_000;

    println!("Project Euler - Problem 2");
    println!("=========================");

    let output = fib(N);

    println!("N: {}", N);
    println!("Sum: {}", output);
}

fn fib(n: u64) -> u64 {
    let mut first = 0;
    let mut second = 1;

    let mut sum = 0;

    let mut k;
    loop {
        k = first + second;
        if k > n {break}
        
        if k % 2 == 0 {
            sum += k;
        }
        

        first = second;
        second = k;
    }

    sum
}