use std::fs;

fn highest_calories(input_arg: Option<String>) -> i32 {
    let input_str = match input_arg {
        Some(input) => input,
        None => fs::read_to_string("src/input").expect("Failed to read input")
    };
    let input = input_str.split("\n\n");

    let mut elves: Vec<i32> = Vec::new();
    let elves_count = input.clone().count();
    println!("There are {} elves", elves_count);

    for part in input {
        let calories = part.lines().map(|line| line.parse::<i32>().unwrap()).sum::<i32>();

        elves.push(calories);
    }

    elves.iter().max().unwrap().clone()
}

fn main() {
    let output = highest_calories(None);

    println!("The highest calories found is: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let output = highest_calories(Some(input.to_string()));
        println!("Output: {}", output);

        assert_eq!(output, 24000);
    }
}