use std::collections::HashMap;

fn median(list: &Vec<i32>) -> i32 {
    let middle_index = list.len() / 2;

    list.get(middle_index).cloned().unwrap()
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut hash_map = HashMap::new();

    for item in list {
        let count = hash_map.entry(item).or_insert(0);
        *count += 1;
    }

    let mut biggest = 0;
    let mut biggest_key = list[0];

    for (key, value) in hash_map {
        if biggest < value {
            biggest = value;
            biggest_key = *key;
        }
    }

    return biggest_key;
}

fn main() {
    let list = vec![10, 23, 6, 97, 45, 2, 7, 9, 10, 1, 2];

    let median = median(&list);
    let mode = mode(&list);

    println!("list {:?}", list);
    println!("median {:?}", median);
    println!("mode {:?}", mode);
}
