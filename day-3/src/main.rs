use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let contents_str = contents.as_str();

    let mut mul_result = 0;
    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = regex.captures_iter(contents_str);
    for cap in result {
        let num1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

        println!("The numbers are: {}, {}", num1, num2);

        mul_result += num1 * num2;
    }

    println!("The result of the multiplication is: {}", mul_result);
}
