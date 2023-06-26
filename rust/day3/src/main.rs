use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1(filename: String) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let letters: Vec<char> = line.chars().collect();
        let middle = letters.len() / 2;

        // compartment 1
        let mut set1: [bool; 52] = [false; 52];
        for i in 0..middle {
            let index = match letters[i] {
                x if x.is_ascii_uppercase() => letters[i] as usize - 'A' as usize + 26,
                x if x.is_ascii_lowercase() => letters[i] as usize - 'a' as usize + 0,
                _ => panic!("non-ascii alpha character"),
            };
            set1[index] = true;
        }
        // compartment 2
        let mut set2: [bool; 52] = [false; 52];
        for i in middle..letters.len() {
            let index = match letters[i] {
                x if x.is_ascii_uppercase() => letters[i] as usize - 'A' as usize + 26,
                x if x.is_ascii_lowercase() => letters[i] as usize - 'a' as usize + 0,
                _ => panic!("non-ascii alpha character"),
            };
            set2[index] = true;
        }

        for i in 0..52 {
            if set1[i] && set2[i] {
                let priority = i as i32 + 1;
                sum += priority;
            }
        }
    }

    return sum;
}

/* the number of items in each compartment is equal -> even number of items
 *
 * when deciding on a data structure for the duplication detection,
 * I chose to have an array of booleans that could hold whether or not the
 * corresponding character has been seen before. The mapping from char to
 * index is one-to-one by casting the character to an integer.
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let result: i32 = match &args[1] {
        x if x == "part1" => part1(String::from("input.txt")),
        x if x == "part2" => unimplemented!(),
        _ => panic!("use arguments 'part1' or 'part2'."),
    };

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let sum = part1(String::from("test.txt"));
        println!("{}", sum);
        assert!(sum == 157);
    }
    #[test]
    fn test_part2() {
        unimplemented!();
    }
}
