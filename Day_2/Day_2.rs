use std::{fs::read_to_string};

fn main() {
    let file_path: &str = "./day2_input.txt";
    let ranges: String = read_to_string(file_path).unwrap();
    let range_vector: Vec<&str> = ranges.trim().split(',').collect();

    part_1(range_vector.clone());

    part_2(range_vector);
}

fn part_1(range_vector: Vec<&str>) {
    let mut total: u128 = 0;

    for range in range_vector {
        total += give_sum_of_invalid_ids_in_range(range);
    }

    println!("Part 1: {}", total);
}

fn part_2(range_vector: Vec<&str>) {
    let mut total: u128 = 0;

    for range in range_vector {
        total += give_sum_of_invalid_ids_in_range_p2(range)
    }

     println!("Part 2: {}", total);
}

fn give_sum_of_invalid_ids_in_range(range: &str) -> u128 {
    let range_str: Vec<&str> = range.split('-').collect();
    let range_start: u128 = range_str[0].parse::<u128>().unwrap();
    let range_end: u128 = range_str[1].parse::<u128>().unwrap();
    let mut ids  = Vec::new();
    let mut sum: u128 = 0;

    for i in range_start..range_end + 1 {
        ids.push(i);
    }

    for id in ids {
        match check_id(id) {
            Ok(i) => {
                sum += i
            }
            Err(_e) => {
                sum += 0
            }
        }
    }
    sum
}

fn check_id(id: u128) -> Result<u128, std::num::ParseIntError> {
    if (id.to_string().len() % 2) == 0 {
        let id = id.to_string();
        let (first_half, second_half) = id.split_at(id.len()/2);
        if first_half == second_half {
            id.parse::<u128>()
        } else {Ok(0)}
    } else {Ok(0)}
}

fn give_sum_of_invalid_ids_in_range_p2(range: &str) -> u128 {
    let range_str: Vec<&str> = range.split('-').collect();
    let range_start: u128 = range_str[0].parse::<u128>().unwrap();
    let range_end: u128 = range_str[1].parse::<u128>().unwrap();
    let mut ids  = Vec::new();
    let mut sum: u128 = 0;

    for i in range_start..range_end + 1 {
        ids.push(i);
    }

    for id in ids {
        sum += check_id_p2(id);
    }

    sum
}

fn check_id_p2(id: u128) -> u128 {
    let id = id.to_string();
    let mut counter: usize = 1;
    let iterations: usize = (id.len() / 2) + 1;
    let mut sum_of_invalid_ids: u128 = 0;
    let mut invalid_ids: Vec<u128> = Vec::new();

    while (counter) < iterations {
        let binding = id.clone();
        let (pattern, id_pattern_removed) = binding.split_at(counter);

        if id_pattern_removed.to_string().remove(0) == '0' {
            counter += 1;
            continue;
        }

        let tmp = id_pattern_removed.strip_suffix(pattern);

        if tmp == Some("") {
            let invalid_id = id.clone();
            match invalid_id.parse::<u128>() {
            Ok(i) => {
                if invalid_ids.contains(&i) {
                } else {
                    sum_of_invalid_ids += i;
                    invalid_ids.push(i);
                }
            }
            Err(_e) => {}
            }
        } 
        // this shouldn't work but it did
        // i have no idea why
        else if tmp != None {
            match id.clone().starts_with(id_pattern_removed) {
                true => {
                    let invalid_id = id.clone();
                    match invalid_id.parse::<u128>() {
                    Ok(i) => {
                        if invalid_ids.contains(&i) {
                        } else {
                            sum_of_invalid_ids += i;
                            invalid_ids.push(i);
                        }
                    }
                    Err(_e) => {}
                    }
                }
                false => {}
            }
        } 
        

        counter += 1;
    }

    sum_of_invalid_ids
}
