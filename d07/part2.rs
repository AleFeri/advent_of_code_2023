use std::collections::HashMap;

fn main() {
    let mut input = read_input("input.txt");
    input.sort_by(|a, b| compare(&a[0], &b[0]));
    let mut result = 0;
    for i in 0..input.len() {
        result += (i as i128 + 1) * input[i][1].parse::<i128>().unwrap();
    }
    println!("Result {}", result);
}

fn categorize(a: &str) -> i32 {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    let char_vec: Vec<char> = a.chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    if letter_counts.len() > 1 && letter_counts.get(&'J').is_some() {
        let highest_letter = letter_counts
            .iter()
            .filter(|&(c, _)| *c != 'J')
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0
            .clone();
        *letter_counts.entry(highest_letter).or_insert(0) += *letter_counts.get(&'J').unwrap();
        letter_counts.remove(&'J');
    }
    if letter_counts.len() == 1 {
        return 7;
    } else if letter_counts.len() == 2 {
        if letter_counts.values().filter(|&v| *v == 4).count() == 1 {
            return 6;
        }
        return 5;
    } else if letter_counts.len() == 3 {
        if letter_counts.values().filter(|&v| *v == 3).count() == 1 {
            return 4;
        }
        return 3;
    } else if letter_counts.len() == 4 {
        return 2;
    }
    1
}

fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let cat_a = categorize(a);
    let cat_b = categorize(b);
    if cat_a > cat_b {
        return std::cmp::Ordering::Greater;
    } else if cat_a < cat_b {
        return std::cmp::Ordering::Less;
    }

    let a1 = a.chars().map(remap_letter).collect::<String>();
    let b1 = b.chars().map(remap_letter).collect::<String>();
    a1.cmp(&b1)
}

fn remap_letter(a: char) -> char {
    match a {
        'A' => 'Z',
        'K' => 'Y',
        'Q' => 'X',
        'J' => '1',
        'T' => 'V',
        _ => a,
    }
}

fn read_input(filename: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().map(String::from).collect())
        .collect()
}
