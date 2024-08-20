mod factorial;
mod prime_numbers;

fn run_factorial() {
    println!("\n* Factorial");

    println!(" - Classic: n = 0 => {}", factorial::classic(0));
    println!(" - Classic: n = 5 => {}", factorial::classic(5));

    println!(" - Recursive: n = 0 => {}", factorial::recursive(0));
    println!(" - Recursive: n = 5 => {}", factorial::recursive(5));

    println!(
        " - Using iterator: n = 0 => {}",
        factorial::using_iterator(0)
    );
    println!(
        " - Using iterator: n = 5 => {}",
        factorial::using_iterator(5)
    );
}

fn run_prime_numbers() {
    println!("\n* Prime numbers");

    println!(
        " - Using while: 1 is prime? {}",
        prime_numbers::using_while(1)
    );
    println!(
        " - Using while: 2 is prime? {}",
        prime_numbers::using_while(2)
    );
    println!(
        " - Using while: 44 is prime? {}",
        prime_numbers::using_while(44)
    );
    println!(
        " - Using while: 97 is prime? {}",
        prime_numbers::using_while(97)
    );

    println!(" - Using for: 1 is prime? {}", prime_numbers::using_for(1));
    println!(" - Using for: 2 is prime? {}", prime_numbers::using_for(2));
    println!(
        " - Using for: 44 is prime? {}",
        prime_numbers::using_for(44)
    );
    println!(
        " - Using for: 97 is prime? {}",
        prime_numbers::using_for(97)
    );

    println!(
        " - Using for_v2: 1 is prime? {}",
        prime_numbers::using_for_v2(1)
    );
    println!(
        " - Using for_v2: 2 is prime? {}",
        prime_numbers::using_for_v2(2)
    );
    println!(
        " - Using for_v2: 44 is prime? {}",
        prime_numbers::using_for_v2(44)
    );
    println!(
        " - Using for_v2: 97 is prime? {}",
        prime_numbers::using_for_v2(97)
    );
}

fn main() {
    println!("### Rust Labs ###");

    run_factorial();

    run_prime_numbers();
}
