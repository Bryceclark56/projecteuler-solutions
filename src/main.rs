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

    let n: u64 = retrieve_n()?;
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

type ResultN = Result<u64, std::io::Error>;

/* Check if argument for N exists. If so, return that
 * Otherwise, get N interactively. */
fn retrieve_n() -> ResultN {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let n = args[1].parse().unwrap();

        println!("N: {}", n);

        return Ok(n);
    }
    else {
        return interactive_n();
    }
}

fn interactive_n() -> ResultN {
    print!("Enter value for N: ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim().parse().unwrap())
}