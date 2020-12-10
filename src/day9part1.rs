use aocf::Aoc;

const PREAMBLE:usize = 25;
const TEST_DATA:&str = 
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

pub fn run() {
    let set = process_input(&read_input());
    println!("The first invalid number is {}", find_invalid_number(set));
}

fn find_invalid_number(set:Vec<usize>) -> usize {
    let mut invalid = 0;
    let mut i = PREAMBLE+0;
    for number in &set {
        if !verify_number(generate_range(&set, &i), &set[i]) {return set[i]}
        i+=1;
    }
    println!("Couldn't find valid number");
    invalid
}

fn verify_number(range:[usize;PREAMBLE],number:&usize) -> bool {
    for i in range.iter() {
        for j in range.iter().rev() {
            if i!=j && i+j==*number {
                return true;
            }
        }
    }
    return false;
}

fn generate_range(set:&Vec<usize>, number:&usize) -> [usize;PREAMBLE] {
    let mut range:[usize;PREAMBLE] = [0;PREAMBLE];
    let mut j = 0;
    for i in number-PREAMBLE..number-0 {
        range[j] = set[i];
        j+=1;
    }
    range
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(9)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output.trim().to_string();
}

fn process_input(input:&str) -> Vec<usize> {
    let input = input.split("\n");
    input.map(|number|number.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}

