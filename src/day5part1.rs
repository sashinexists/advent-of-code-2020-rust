use aocf::Aoc;

pub fn run() {
    let boarding_passes: Vec<BoardingPass> = process_input(&read_input());
    println!(
        "The highest boarding number is {}.",
        find_highest_seat_id(&boarding_passes)
    );
}

struct BoardingPass {
    row_instructions: [bool; 7],
    column_instructions: [bool; 3],
}

impl BoardingPass {
    fn create(seat_code: &str) -> BoardingPass {
        let row: String = seat_code[0..=6].to_string();
        let column: String = seat_code[7..].to_string();
        let mut row_instructions: [bool; 7] = [false, false, false, false, false, false, false];
        let mut column_instructions: [bool; 3] = [false, false, false];
        for (i, seat) in row.chars().enumerate() {
            if seat == 'F' {
                row_instructions[i] = true
            };
        }
        for (i, seat) in column.chars().enumerate() {
            if seat == 'L' {
                column_instructions[i] = true
            };
        }
        return BoardingPass {
            row_instructions: row_instructions,
            column_instructions: column_instructions,
        };
    }

    fn get_row(&self) -> u32 {
        let mut start: u32 = 0;
        let mut range: u32 = 128;
        let mut i: usize = 0;
        while range > 1 {
            if !self.row_instructions[i] {
                start += range / 2
            }
            range /= 2;
            i += 1;
        }
        return start;
    }

    fn get_column(&self) -> u32 {
        let mut start: u32 = 0;
        let mut range: u32 = 8;
        let mut i: usize = 0;
        while range > 1 {
            if !self.column_instructions[i] {
                start += range / 2
            }
            range /= 2;
            i += 1;
        }
        return start;
    }

    fn get_seat_code(&self) -> u32 {
        return self.get_row() * 8 + self.get_column();
    }

    fn print(&self) {
        let mut display: String = String::new();
        for seat in self.row_instructions.iter() {
            if *seat {
                display.push('🌱')
            } else {
                display.push('🌷')
            };
        }
        for seat in self.column_instructions.iter() {
            if *seat {
                display.push('🌱')
            } else {
                display.push('🌷')
            };
        }
        println!("{}", display);
    }
}

fn find_my_seat(seats:&Vec<BoardingPass>) ->u32 {
    let my_row:u32 = find_my_row(&seats);
    let seats:&Vec<&BoardingPass> = &seats.iter().filter(|&seat|seat.get_row()==my_row).collect::<Vec<&BoardingPass>>();
    let seats:Vec<u32>= seats.iter().map(|seat|seat.get_column()).collect::<Vec<u32>>();
    let mut my_seat:u32 = 0;
    seats.iter().for_each(|item|println!("Column: {} exists", item));
    for i in 1..8 {
        if !seats.iter().any(|item|item==&i) {my_seat = i}
        else {println!("{}",i)};
    }
    return my_row * 8 + my_seat;
}

fn find_my_row(seats:&Vec<BoardingPass>) -> u32 {
    let mut all_rows: Vec<u32> = seats.iter().map(|pass| pass.get_row()).collect();
    all_rows.sort();
    let mut my_row:u32 = 0;
    for i in 0..108 {
        if count_instance_in_vec(i, &all_rows) == 7 {my_row = i};
    }
    return my_row;
}

fn find_highest_seat_id(boarding_passes: &Vec<BoardingPass>) -> u32 {
    let boarding_passes: Vec<u32> = boarding_passes
        .iter()
        .map(|pass| pass.get_seat_code())
        .collect();
    return find_highest_number(boarding_passes);
}

fn find_highest_number(numbers: Vec<u32>) -> u32 {
    let mut highest_number: u32 = numbers[0];
    for number in numbers {
        if number > highest_number {
            highest_number = number
        };
    }
    return highest_number;
}

fn count_instance_in_vec(instance: u32, vector: &Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    let vector:Vec<u32> = vector.clone();
    for el in vector {
        if el==instance {count+=1;};
    }
    return count;
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(5)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output.trim().to_string();
}

fn process_input(input: &str) -> Vec<BoardingPass> {
    let mut boarding_passes: Vec<BoardingPass> = Vec::new();
    let input: Vec<&str> = input.split("\n").collect();
    for seat_code in input {
        boarding_passes.push(BoardingPass::create(seat_code));
    }
    return boarding_passes;
}
