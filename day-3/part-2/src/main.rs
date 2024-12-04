use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let contents_str = contents.as_str();

    let mut mul_enabled = true;
    let mut mul_result = 0;

    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let result = regex.captures_iter(contents_str);

    for cap in result {
        if let Some(instr) = cap.get(1) {
            match instr.as_str() {
                "do()" => mul_enabled = true,
                "don't()" => mul_enabled = false,
                _ => {
                    if mul_enabled {
                        if let (Some(num1), Some(num2)) = (cap.get(2), cap.get(3)) {
                            let num1 = num1.as_str().parse::<i32>().unwrap();
                            let num2 = num2.as_str().parse::<i32>().unwrap();
                            mul_result += num1 * num2;
                        }
                    }
                }
            }
        }
    }

    println!("The result of the multiplication is: {}", mul_result);
}