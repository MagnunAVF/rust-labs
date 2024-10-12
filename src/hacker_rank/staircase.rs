// URL: https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true

fn staircase(n: i32) {
    for i in 0..n {
        let empty_blocks_size = n - 1 - i;
        let stairs_blocks_size = i + 1;
        let mut line = vec![' '; empty_blocks_size as usize];
        let line_stairs = vec!['#'; stairs_blocks_size as usize];
        line.extend(line_stairs);

        let line_to_print: String = line.into_iter().collect();

        println!("{}", line_to_print);
    }
}

pub fn main() {
    println!("\n07 - Staircase");

    let n = 6;
    println!(" n => {}", n);
    staircase(n);
}
