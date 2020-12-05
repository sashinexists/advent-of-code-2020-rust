use aocf::Aoc;

pub fn run() {
    let input = read_input();
    let passports = process_input(&input);
    println!("There are {} valid passports.", count_valid_passports(passports));
}

fn count_valid_passports(passports:Vec<&str>) -> i32
{
    let mut count:i32 = 0;
    for passport in passports{
        if validate_entry(&passport) {count+=1};
    }
    return count;
}

fn validate_entry(entry: &str) -> bool {
    return entry.contains("iyr") 
        && entry.contains("eyr")
        && entry.contains("hgt")
        && entry.contains("hcl")
        && entry.contains("ecl")
        && entry.contains("pid")
        && entry.contains("byr")
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(4)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i;
    }
    return output;
}

fn process_input(input: &str) -> Vec<&str> {
    let split = input.split("\n\n");
    return split.collect();
}
