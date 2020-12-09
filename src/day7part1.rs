use aocf::Aoc;
use std::collections::HashMap;


pub fn run() {
    let test = 
    "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    let mut bags = process_input(&read_input());
    bags.remove("shiny gold");
    println!("{} bags can eventually hold a shiny gold bag", how_many_bags_hold_colour(&mut bags, "shiny gold"));
    /*for (colour, bag) in bags {
        for contained in bag.contains {
            println!("{} contains {}", colour, contained);
        }
    }*/

}



struct Bag {
    colour: String,
    contains: Vec<String>,
}

impl Bag {
    fn create(data: &str) -> Bag {
        let mut data: Vec<&str> = data.split(" bags contain ").collect();
        let colour: &str = data[0];
        let replacement = &data[1].replace(" bag, ", " bags, ");
        data[1] = replacement;
        let contains_data: Vec<&str> = data[1].split(" bags, ").collect();
        let mut bag = Bag {
            colour: colour.to_string(),
            contains: Vec::new(),
        };
        for data in contains_data {
            bag.add_contains(data);
        }
        bag
    }

    fn add_contains(&mut self, contains_data: &str) {
        let number_candidate: char = contains_data.chars().nth(0).unwrap();
        if number_candidate != 'n' {
            let number = number_candidate.to_digit(10).unwrap();
            for _i in 0..number {
                self.contains.push(contains_data[1..].trim().to_string())
            }
        }
    }

    fn holds_bag_of_colour(&self, colour:&str, bags:&mut HashMap<String, Bag>) -> bool {
        for contained in &self.contains {
            //println!("{} is in {}", contained, self.colour);
            if contained==colour ||  bags[contained].contains.iter().any(|col|col==colour||bags[col].holds_bag_of_colour(colour, bags)) {
                bags.remove(contained);
                return true;
            };
        }
        false
    }

    fn display(&self) {
        let mut contains = String::new();
        for bag in self.contains.iter() {
            contains += " ";
            contains += &bag;
            contains += ",";
        }
        println!("{}: {}", self.colour, contains)
    }
}

fn how_many_bags_hold_colour(bags:&mut HashMap<String, Bag>, colour:&str) -> u32 {
    let mut count = 0;
    for (bag_col, bag) in bags.iter() {
        if bag.holds_bag_of_colour(colour,  bags) {
            println!("{} bags can carry {} bags.", bag_col, colour);
            count+=1;
        };
    }
    count
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(7)).init().unwrap();
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output.trim().to_string();
}

fn process_input(input: &str) -> HashMap<String, Bag> {
    let input: Vec<&str> = input.split("\n").collect();
    let mut bags   = HashMap::new();
    for line in input {
        let bag:Bag = Bag::create(&line[..line.len() - 5].trim());
        let colour =bag.colour;
        bags.insert(colour,Bag::create(&line[..line.len() - 5].trim()));
    }
    bags
}
