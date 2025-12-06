fn main() {
    let input = read_input_from_file("input.txt");
    let time = input[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .unwrap();
    let distance = input[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u128>()
        .unwrap();

    let mut result = 0;
    for t in 0..time {
        if t * (time - t) > distance {
            result += 1;
        }
    }
    println!("Result {}", result);
}

fn read_input_from_file(file_path: &str) -> Vec<String> {
    std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
