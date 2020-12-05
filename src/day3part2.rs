use aocf::Aoc;

pub fn run() {
    let valley = process_input(&read_input());
    println!("There are {} trees in the way.", count_trees(&valley));
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn starting_position() -> Position {
        Position{x:0,y:0}
    }
}

struct Slope {
    x: usize,
    y: usize,
}

impl Slope {
    fn count_trees(&self, valley: &Vec<Vec<bool>>) -> i32 {
        let mut count: i32 = 0;
        let mut position = Position::starting_position();
        while position.y < valley.len() {
            if is_there_a_tree(valley, &position) {
                count += 1
            };
            position = slide_tobbogan(valley, &position, self);
        }
        return count;
    }
}

fn count_trees(valley: &Vec<Vec<bool>>) -> i32 {
    let slopes:[Slope; 5] = [Slope{x:1,y:1}, Slope{x:3,y:1},Slope{x:5,y:1},Slope{x:7,y:1},Slope{x:1,y:2}];
    let mut count:i32 = 1;
    for slope in slopes.iter() {
        count *= slope.count_trees(&valley);
    }
    return count;
}

fn slide_tobbogan(valley: &Vec<Vec<bool>>, position: &Position, slope: &Slope) -> Position {
    let row_length = valley[0].len();
    let y = position.y + slope.y;
    let mut x = position.x + slope.x;
    if x >= row_length {
        x -= row_length
    };
    return Position { x: x, y: y };
}

fn is_there_a_tree(valley: &Vec<Vec<bool>>, position: &Position) -> bool {
    return valley[position.y][position.x];
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(3)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output;
}

fn process_input(input: &str) -> Vec<Vec<bool>> {
    let input: Vec<&str> = input.split("\n").collect();
    let mut valley: Vec<Vec<bool>> = Vec::new();
    for row in input {
        valley.push(process_row(row));
    }
    return valley;
}

fn process_row(row_str: &str) -> Vec<bool> {
    let mut row: Vec<bool> = Vec::new();
    for c in row_str.chars() {
        if c == '#' {
            row.push(true)
        } else if c == '.' {
            row.push(false)
        };
    }
    return row;
}

fn display_valley(valley: &Vec<Vec<bool>>) {
    for row in valley {
        let mut display: String = String::new();
        for space in row {
            if *space {
                display += "🌲"
            } else {
                display += "🤍"
            }
        }
        println!("{}", display);
    }
}
