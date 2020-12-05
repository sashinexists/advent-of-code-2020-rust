use aocf::Aoc;


pub fn run() {
    let expense_report:Vec<i32> = process_input(read_input());
    println!("The fixed report comes to {}", fix_expense_report(&expense_report));
}

fn fix_expense_report(expense_report:&Vec<i32>) -> i32
{
    for i in 0..expense_report.len() {
        for j in 0..expense_report.len() {
            if i!=j && expense_report[i]+expense_report[j]==2020 {
                return expense_report[i]*expense_report[j];
            }
        }
    }
    return 0;
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(1)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output;
}

fn process_input(input:String) -> Vec<i32> {
    let numbers:Vec<&str> = input.split("\n").collect();
    return numbers.iter().map(|number|number.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}