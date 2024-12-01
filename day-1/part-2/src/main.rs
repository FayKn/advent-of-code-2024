use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let content_arr = contents.split('\n');

    let collection: Vec<&str> = content_arr.collect();

    // split the lists in left and right
    let mut left_list: Vec<&str> = Vec::new();
    let mut right_list: Vec<&str> = Vec::new();
    for item in collection {
        let lists: Vec<&str> = item.split("   ").collect();
        if lists.len() == 2 {
            left_list.push(lists[0]);
            right_list.push(lists[1]);
        }
    }

    let left_list: Vec<i32> = left_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let right_list: Vec<i32> = right_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut right_freq = HashMap::new();

    // count the frequency of a number in the right list and add it to the hashmap
    for item in right_list.iter() {
        let count = right_freq.entry(item).or_insert(0);
        *count += 1;
    }

    println!("{:?}", right_freq);

    let mut diff_sum = 0;
    for i in 0..left_list.len() {
        let ll = left_list[i];
        let rl_count = right_freq.get(&ll).unwrap_or(&0);

        let mut diff = ll * rl_count;

        if diff < 0 {
            diff = diff * -1;
        }

        diff_sum += diff;
    }

    println!("The sum of the differences is: {}", diff_sum);
}