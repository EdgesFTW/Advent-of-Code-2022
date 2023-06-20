use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn day1(filename: String) -> std::io::Result<(i64, i64, i64)> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);

    let (mut first_total_cal, mut second_total_cal, mut third_total_cal) = (0, 0, 0);
    let mut running_total = 0;

    for line in buf_reader.lines() {
        // println!("{}", line.unwrap());

        assert!(line.is_ok());

        let cal: i64 = line.unwrap_or(String::from("0")).parse().unwrap_or(-1);

        if cal == -1 {
            // check where elf should go
            match running_total {
                x if x > first_total_cal => {
                    third_total_cal = second_total_cal;
                    second_total_cal = first_total_cal;
                    first_total_cal = x;
                }
                x if x > second_total_cal => {
                    third_total_cal = second_total_cal;
                    second_total_cal = x;
                }
                x if x > third_total_cal => {
                    third_total_cal = x;
                }
                _ => {}
            }

            running_total = 0;
            continue;
        }

        running_total += cal;
    }

    Ok((first_total_cal, second_total_cal, third_total_cal))
}

fn main() -> std::io::Result<()> {
    let res = day1(String::from("input.txt"));

    match res {
        Ok(x) => {
            let (first_total_cal, second_total_cal, third_total_cal) = x;
            println!("First: {}", first_total_cal);
            println!("Second: {}", second_total_cal);
            println!("Third: {}", third_total_cal);
            println!(
                "Sum: {}",
                first_total_cal + second_total_cal + third_total_cal
            );
        }
        Err(_) => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn first_test() {
        let res = day1(String::from("test.txt"));

        match res {
            Ok(x) => {
                assert!(x.0 == 24000);
                assert!(x.1 == 11000);
                assert!(x.2 == 10000);
            }
            Err(_) => {
                panic!()
            }
        }
    }
}
