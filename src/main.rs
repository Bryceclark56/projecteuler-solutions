/* Project Euler
 *  Problem 1 
 *  
 * Find the sum of all the multiples of 3 or 5 below N,
 *  where N is a natural number.
 */

 use std::io::Write;

fn main() -> Result<(), std::io::Error>{
    println!("Project Euler - Problem 1");
    println!("=========================");

    print!("Enter value for N: ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let n: u64 = input.trim().parse().unwrap();
    println!("\nRunning calculation...");


    let mut sum = 0;

    for i in 1..n {
        if i % 3 == 0 {
            sum += i;
        }
        else if i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
