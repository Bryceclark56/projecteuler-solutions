/* Project Euler
 *  Problem 1 
 *  
 * Find the sum of all the multiples of 3 or 5 below N,
 *  where N is a natural number.
 */

fn main() {
    const N: isize = 1000; // Constant N for now, default + cli argument later.

    println!("Project Euler - Problem 1");
    println!("=========================");

    println!("N = {}", N);
    println!("\nRunning calculation...\n");


    let mut sum = 0;

    for i in 1..N {
        if i % 3 == 0 {
            sum += i;
        }
        else if i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum: {}", sum);
}
