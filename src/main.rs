mod factorial;

fn main() {
    println!("### Rust Labs ###");

    println!("\n* Factorial");
    println!("   - Classic: n = 5 => {}", factorial::classic(5));
    println!("   - Recursive: n = 5 => {}", factorial::recursive(5));
    println!(
        "   - Using iterator: n = 5 => {}",
        factorial::using_iterator(5)
    );
}
