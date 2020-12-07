use aocf::Aoc;

pub fn run() {
    let input:String = read_input();
    let groups:Vec<String> = process_input(&input);
    for group in &groups {
        println!("{} has {} characters.", group, group.len());
    }
    println!("The sum of all unique questions answered is {}.", sum_all_unique_question_counts(&groups));
}

fn sum_all_unique_question_counts(groups:&Vec<String>) -> usize {
    let mut count:usize = 0;
    for group in groups {
        count += group.len();
    }
    return count;
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(6)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output.trim().to_string();
}

fn process_input(input:&str) -> Vec<String> {
    let input:Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let input:Vec<String> = input.iter()
            .map(|&group|group.replace("\n", ""))
            .collect::<Vec<String>>();
    let input = input.iter().map(|group|delete_duplicate_char_in_string(group)).collect::<Vec<String>>();
    return input;
}

fn delete_duplicate_char_in_string(string:&str) ->String {
    let mut string:Vec<char> = string.chars().collect::<Vec<char>>();
    string.sort();
    string.dedup();
    return string.iter().collect::<String>();
}