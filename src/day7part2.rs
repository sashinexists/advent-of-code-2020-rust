use aocf::Aoc;
use regex::Regex;
use std::collections::HashMap;

const TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

pub fn run() {
    let bags = process_input(&read_input());
    let bags_that_shiny_gold_can_hold =
        every_bag_contained_by_colour(&"shiny gold".to_string(), &bags);
    /*bags_that_shiny_gold_can_hold
        .iter()
        .for_each(|bag| println!("{}", bag));*/
    println!("Shiny Gold can hold {} bags.", bags_that_shiny_gold_can_hold.len());
}

fn every_bag_contained_by_colour(
    colour: &String,
    bags: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let bag_children: Vec<String> = bags[colour].clone();
    let mut all_contained: Vec<String> = bag_children.clone();
    for child in bag_children {
        let possible_children = every_bag_contained_by_colour(&child, &bags);
        for possible_child in possible_children {
            //if !all_contained.iter().any(|bag| bag==&possible_child) {all_contained.push(possible_child)};
            all_contained.push(possible_child);
        }
    }
    all_contained
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(7)).init().unwrap();
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    //let mut output = String::new();
    //output = TEST_INPUT.trim().to_string();
    let bags_regex = Regex::new(r"bag?s?.").unwrap();
    let comma_spaces_regex = Regex::new(r"  ").unwrap();
    let full_stop_regex = Regex::new(r".\n").unwrap();
    output = bags_regex.replace_all(&output, "").to_string();
    output = comma_spaces_regex.replace_all(&output, ",").to_string();
    output = full_stop_regex.replace_all(&output, "\n").to_string();
    output[..output.len() - 1].to_string()
}

fn process_input(input: &str) -> HashMap<String, Vec<String>> {
    let bag_strings: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in bag_strings.iter() {
        let line = line.split(" contain ").collect::<Vec<&str>>();
        let colour: &str = line[0];
        let children = find_children(colour, line[1]);
        //println!("Colour is: {}.\nChildren are: {}\n", colour, line[1]);
        bags.insert(colour.to_string(), children);
    }
    bags
}

fn find_children(colour: &str, children: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    if children.chars().nth(0).unwrap() != 'n' {
        let children_vec: Vec<&str> = children.split(",").collect();
        for child in children_vec.iter() {
            let number = child.chars().nth(0).unwrap().to_digit(10).unwrap();
            let colour: String = child[2..].to_string();
            for _i in 0..number {
                output.push(colour.clone());
            }
        }
    }
    output
}
