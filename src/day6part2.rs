use aocf::Aoc;

pub fn run() {
    let input:String = read_input();
    let groups:Vec<String> = process_input(&input);
    println!("The sum of all unanimous answers is {}.", sum_all_unanimous_answers(&groups));
}

fn sum_all_unanimous_answers(groups:&Vec<String>) -> u32 {
    let mut count:u32 = 0;
    for group in groups {
        count += count_all_unanimous_answers(group);
    }
    return count;
}

fn count_all_unanimous_answers(group:&str) ->u32 {
    let group:Vec<&str> = group.split("||||||||").collect();
    let group:Vec<String> = group.iter().map(|person|delete_duplicate_char_in_string(person)).collect::<Vec<String>>();
    let number_of_members:usize = group.len();
    let group:String = collapse_string_vector(group);
    let mut count:u32 = 0;
    for c in delete_duplicate_char_in_string(&group).chars()
    {
        if count_char_in_str(c, &group)==number_of_members {
            count+=1;
        }
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
            .map(|&group|group.replace("\n", "||||||||"))
            .collect::<Vec<String>>();
    return input;
}

fn delete_duplicate_char_in_string(string:&str) ->String {
    let mut string:Vec<char> = string.chars().collect::<Vec<char>>();
    string.sort();
    string.dedup();
    return string.iter().collect::<String>();
}

fn count_char_in_str(ch:char, string:&str) -> usize {
    let mut count:usize = 0;
    for c in string.chars() {
        if c==ch {count+=1}
    }
    return count;
}

fn collapse_string_vector(string_vector:Vec<String>) -> String {
    let mut collapsed:String = String::new();
    for string in string_vector{
        collapsed += &string;
    }
    return collapsed;
}

