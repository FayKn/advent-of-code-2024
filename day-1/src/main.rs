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

    let mut left_list: Vec<i32> = left_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut right_list: Vec<i32> = right_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    left_list.sort();
    right_list.sort();

    let mut diff_sum = 0;
    for i in 0..left_list.len() {
        let rl = right_list[i];
        let ll = left_list[i];

        let mut diff = rl - ll;

        if diff < 0 {
            diff = diff * -1;
        }

        diff_sum += diff;
    }

    println!("The sum of the differences is: {}", diff_sum);
}