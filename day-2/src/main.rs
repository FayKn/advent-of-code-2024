use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let content_arr = contents.split('\n');

    let collection: Vec<&str> = content_arr.collect();

    let mut numbers:Vec<Vec<i32>> = Vec::new();
    for item in collection {
        // skip empty strings
        if item.is_empty() {
            continue;
        }

        let numbers_split: Vec<&str> = item.split(" ").collect();
        numbers.insert(0, numbers_split.iter().map(|x| x.parse::<i32>().unwrap()).collect());
    }

    let mut safe_counter = 0;

    for number in numbers.iter() {
        let mut last_num :i32 = -99;
        let mut last_inc_by = 0;
        let mut last_decr_by = 0;

        let mut in_loop_safe_counter = 0;

        let mut loop_progress = 0;
        for num in number.iter() {
            if loop_progress == 0 {
                last_num = *num;

                loop_progress += 1;
                in_loop_safe_counter += 1;
                continue;
            }

            let inc_by = num - last_num;
            let decr_by = last_num - num;

            if (inc_by <= 3 && inc_by >= 1 && last_decr_by <= 0) || (decr_by >= 1 && decr_by <= 3 && last_inc_by <= 0) {
                in_loop_safe_counter += 1;
            } else {
                // println!("The number is not safe: {}", num);
                // println!("The number is not safe: {:?}", number);
                // println!("inc_by: {}, decr_by: {}, last_decr_by: {}, last_inc_by: {}", inc_by, decr_by, last_decr_by, last_inc_by);
                break;
            }
            last_num = *num;
            last_decr_by = decr_by;
            last_inc_by = inc_by;

            loop_progress += 1;
        }

        if in_loop_safe_counter == number.len() {
            // println!("The safe numbers are: {:?}", number);

            safe_counter += 1;
        }
    }

    println!("The number of safe numbers is: {}", safe_counter);
}