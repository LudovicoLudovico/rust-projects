// Prime decomposition
use std::collections::HashMap;
use std::io;
fn main() {
    print!("Insert the number to be factorized:\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let mut input = match input.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse input to number"),
    };

    print!("\nFactorization:\n");

    let mut i = 2;
    let upper_bound = input / 2;

    let mut factorization: HashMap<u32, u32> = HashMap::new();
    let mut last_prime = 2;

    while i < upper_bound {
        if is_prime(&last_prime, &i) && input % i == 0 {
            input /= i;
            last_prime = i;
            let count = factorization.entry(i).or_insert(0);
            *count += 1;
            i -= 1;
        }

        i += 1;
    }

    for (k, v) in factorization {
        println!("{k}^{v}");
    }
}

fn is_prime(&last_prime: &u32, &number: &u32) -> bool {
    for n in last_prime..(number / 2) {
        if number % n == 0 {
            return false;
        }
    }

    return true;
}
