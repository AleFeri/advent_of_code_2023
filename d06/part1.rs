fn main() {
    let input = read_input_from_file("input.txt");
    let times = input[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = input[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut result = 1;
    for i in 0..times.len() {
        let mut win = 0;
        for time in 0..times[i] {
            if time * (times[i] - time) > distances[i] {
                win += 1;
            }
        }
        result *= win;
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
