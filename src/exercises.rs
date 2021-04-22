use std::collections::HashMap;

pub fn main() {
    println!("Exercises");

    let numbers = vec![1, 2, 2, 4, 5, 2, 3, 4, 5, 2, 3, 1];
    println!("List of integers: {:?}", numbers);
    println!("mean: {}", mean(&numbers));
    println!("median: {}", median(&numbers));
    println!("mode: {:?}", mode(&numbers));
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    return sum as f32 / numbers.len() as f32;
}

fn median(numbers: &Vec<i32>) -> f32 {
    let mut cloned = numbers.clone();
    cloned.sort_unstable();
    let l = cloned.len();

    let lower = cloned[l / 2 - 1];
    let upper = cloned[l / 2 - (l % 2 != 0) as usize];
    return (lower + upper) as f32 / 2.0;
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut max = 0;
    let mut grouped: HashMap<i32, i32> = HashMap::new();

    for n in numbers {
        let counter = grouped.entry(*n).or_insert(0);
        *counter += 1;
        if *counter > max {
            max = *counter
        }
    }

    let mut mode: Vec<i32> = vec![];
    for (k, n) in &grouped {
        if *n == max {
            mode.push(*k)
        }
    }

    return mode;
}
