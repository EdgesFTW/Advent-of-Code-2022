use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;

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


#[derive(Clone)]
struct ThreeElfs {
    first: String,
    second: String,
    third: String,
}


fn part2(filename: String) -> i32{
    let mut result: i32 = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut groups: Vec<ThreeElfs> = Vec::new();

    // load datastructure
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>().unwrap();
    let mut cur_group: ThreeElfs = ThreeElfs {
        first: String::new(),
        second: String::new(),
        third: String::new()
    };
    for (i, val) in lines.iter().enumerate(){
        let str: String = val.clone();
        match i%3 {
            0 =>
                cur_group.first = str,
            1 =>
                cur_group.second = str,
            2 => {
                cur_group.third = str;
                groups.push(cur_group.clone());
            },
            _ => panic!("i%3 > 2")

        }
    }

    let possible_items = ["a","b","c","d","e","f","g","h","i","j","k","l","m",
    "n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E",
    "F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W",
    "X","Y","Z"];
    assert!(possible_items.len() == 52);
    for group in groups{
        for (i, item) in possible_items.iter().enumerate(){
            let first_has = group.first.contains(item);
            let second_has = group.second.contains(item);
            let third_has = group.third.contains(item);
            let priority: i32 = (i + 1) as i32 ;

            if first_has && second_has && third_has {
                result += priority;
                break;
            }
        }
    }



    return result;
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

    if args.len() < 2 {
        panic!("use arguments 'part1' or 'part2'.")
    }

    let result: i32 = match &args[1] {
        x if x == "part1" => part1(String::from("input.txt")),
        x if x == "part2" => part2(String::from("input.txt")),
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
