use aocf::Aoc;
use std::collections::HashMap;
use regex::Regex;

const TEST_INPUT:&str = 
"light red bags contain 1 bright white bag, 2 muted yellow bags.
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
    let mut bags_that_hold_shiny_gold = all_bags_that_can_contain_colour(bags, "shiny gold");
    //bags_that_hold_shiny_gold.dedup_by(|a,b|a[..]==b[..]);
    /*for bag in bags_that_hold_shiny_gold {
        println!("{}", bag);
    }*/
    println!("{} bags can hold shiny gold.", bags_that_hold_shiny_gold.len());

}

fn all_bags_that_can_contain_colour(bags: HashMap<String,Vec<String>>, colour:&str) -> Vec<String> {
    let mut bag_parents = bags[colour].clone();
    let mut all_bags = bag_parents.clone();
    for parent in bag_parents {
        let possible_parents = all_bags_that_can_contain_colour(bags.clone(), &parent);
        for possible_parent in possible_parents {
            if !all_bags.iter().any(|bag|bag==&possible_parent) {all_bags.push(possible_parent)};
        }
    }
    all_bags
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
    let mut bags:HashMap<String, Vec<String>> = HashMap::new();
    for line in bag_strings.iter(){
        let colour:&str = line.split("  contain ").collect::<Vec<&str>>()[0];
        let parents = find_parents(colour, bag_strings.clone());
        bags.insert(colour.to_string(), parents);
    }
    bags
}

fn find_parents(colour:&str, input: Vec<&str>) -> Vec<String> {
    let mut parents:Vec<String> = Vec::new();
    for bag in input {
        let data = bag.split("  contain ").collect::<Vec<&str>>();
        let bag_colour = data[0];
        let children = data[1];
        //println!("The colour is {}.\n The children are: {}\n Do the children contain the colour?\n {}\n\n", colour, children, children.contains(colour));
        if children.contains(colour) {
            parents.push(bag_colour.trim().to_string());
        }
    }
    return parents;
}