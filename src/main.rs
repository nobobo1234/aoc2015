mod day01;

fn main() {
    println!("Running day 1:");

    let part1 = match day01::part1() {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };

    println!("Solution to part 1: {}", part1);
}
