use aocf::Aoc;
use std::collections::HashMap;
use regex::Regex;

const TEST_INPUT:&str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

pub fn run() {
    let bags = process_input(TEST_INPUT);
    for (colour, parents) in bags {
        let parent_string:String = "".to_string();
        for parent in parents {
            
        }
    }
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(7)).init().unwrap();
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    let bags_regex = Regex::new(r"bag?s").unwrap();
    let comma_spaces_regex = Regex::new(r" , ").unwrap();
    let full_stop_regex = Regex::new(r".\n").unwrap();
    output = bags_regex.replace_all(&output, "").to_string();
    output = comma_spaces_regex.replace_all(&output, ",").to_string();
    output = full_stop_regex.replace_all(&output, "\n").to_string();
    return output[..output.len()-1].to_string();
}

fn process_input(input: &str) -> HashMap<String, Vec<String>> {
    let bag_strings:Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let bags:HashMap<String, Vec<String>> = HashMap::new();
    for line in bag_strings.iter(){
        let colour:&str = line.split("  contain ").collect::<Vec<&str>>()[0];
        let parents = find_parents(colour, bag_strings.clone());
    }
    bags
}

fn find_parents(colour:&str, input: Vec<&str>) -> Vec<String> {
    let mut parents:Vec<String> = Vec::new();
    for bag in input {
        let data = bag.split("  contain ").collect::<Vec<String>>();
        let bag_colour = data[0];
        let children = data[1];
        if children.contains(colour) {parents.push(bag_colour)}
    }
    return parents;
}