/* Project Euler
 *  Problem 3
 *  
 * Find the largest palindrome made from
 *  the product of two 3-digit numbers.
 */

fn main() {
    println!("Project Euler - Problem 4");
    println!("=========================\n");

    println!("Finding solution...");

    let range = 100..=999;

    let largest_palindrome = range.clone().map(|n| {
        range.clone().map(|k| n * k)
        .filter(|k| k.to_string().eq(&k.to_string().chars().rev().collect::<String>()))
        .max().unwrap_or(0)
    })
    .filter(|n| n.to_string().eq(&n.to_string().chars().rev().collect::<String>()))
    .max().unwrap();

    println!("Found {}", largest_palindrome);
}

