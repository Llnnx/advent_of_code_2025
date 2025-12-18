use core::str;
use {std::fs::read_to_string, i32};

fn main() {
    let mut dial: i32 = 50;
    let file_path: &str = "./p1_input.txt";
    let mut password: i32 = 0;
    let instructions: Vec<String> = read_lines(file_path);
    let mut prev_dial: i32 = 50;

    for instruction in instructions {
        let temp = dial + give_action(instruction.clone());
        password += give_password(instruction);
        if temp < 0 {
            dial = temp + 100;
            // For p2
            if prev_dial != 0 {
                password += 1; 
            }
        } else if temp > 99 {
            dial = temp - 100;
            // For p2
            if prev_dial != 0 {
                password += 1; 
            }
        } else if temp == 0 {
            dial = 0;
            password += 1
        } else {
            dial = temp;
        }
        prev_dial = dial.clone();
    }

    println!("Password: {}", password);
}

fn read_lines(file_path: &str) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    let file_contents = read_to_string(file_path).unwrap();
    for line in file_contents.lines() {
        vector.push(line.to_string());
    }
    vector
}

fn give_action(mut instruction: String) -> i32 {
    let direction = instruction.remove(0);
    let amount = instruction.trim().parse::<i32>().unwrap();

    if direction == 'L' {
        -amount % 100
    } else {
        amount % 100
    }
}

// For p2
fn give_password(mut instruction: String) -> i32 {
    instruction.remove(0);
    let amount = instruction.trim().parse::<i32>().unwrap();
    amount / 100
}
