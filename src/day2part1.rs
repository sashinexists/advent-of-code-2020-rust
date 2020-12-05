use aocf::Aoc;

pub fn run() {
    let records:Vec<Record> = process_input(&read_input());
    println!("There are {} valid passwords in the batch.", count_valid_passwords(records));
}

struct Record {
    password: String,
    policy: Policy,
}

impl Record {
    fn create(record: &str) -> Record {
        let record: Vec<&str> = record.split(": ").collect();
        let password: String = record[1].to_string();
        let policy: Vec<&str> = record[0].split(" ").collect();
        let letter: char = policy[1].parse::<char>().unwrap();
        let policy: Vec<&str> = policy[0].split("-").collect();
        let minimum: i32 = policy[0].parse::<i32>().unwrap();
        let maximum: i32 = policy[1].parse::<i32>().unwrap();
        let policy = Policy {
            minimum: minimum,
            maximum: maximum,
            letter: letter,
        };
        return Record {
            password: password,
            policy: policy,
        };
    }

    fn print(&self) {
        println!("The password for this record is {}\nThe policy for this password is to include the letter {} at least {} times but no more that {} times.\n", self.password, self.policy.letter,self.policy.minimum, self.policy.maximum)
    }

    fn validate(&self) -> bool {
        let occurences:i32 = count_specific_char_in_a_str(self.policy.letter, &self.password);
        return occurences >= self.policy.minimum && occurences <= self.policy.maximum;
    }
}

struct Policy {
    minimum: i32,
    maximum: i32,
    letter: char,
}

fn count_valid_passwords(records:Vec<Record>) -> i32 {
    let mut count:i32 = 0;
    for record in records {
        if record.validate() {count+=1};
    }
    return count;
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(2)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output;
}

fn process_input(input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    let input: Vec<&str> = input.split("\n").collect();
    for record in input {
        records.push(Record::create(record));
    }
    return records;
}

fn count_specific_char_in_a_str(ch:char, string:&str) -> i32 {
    let mut count:i32 = 0;
    for c in string.chars()
    {
        if c==ch {count+=1};
    }
    return count;
}
