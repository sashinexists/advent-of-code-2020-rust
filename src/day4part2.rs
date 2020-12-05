use aocf::Aoc;

pub fn run() {
    let input:String = read_input();
    let passports:Vec<&str> = process_input(&input);
    let passports:Vec<Passport> = build_passports(passports);
    let count:i32 = count_valid_passports(passports);
    println!("There are a total of {} valid passports in this lot", count);
}

struct Passport {
    birth_year: i32,
    issue_year: i32,
    height: i32,
    height_units: String,
    hair_colour: String,
    passport_id: String,
    eye_colour: String,
    country_id: i32,
    expiration_year: i32
}

const MINIMUM_BIRTH_YEAR:i32 = 1920;
const MAXIMUM_BIRTH_YEAR:i32 = 2002;
const MINIMUM_ISSUE_YEAR:i32 = 2010;
const MAXIMUM_ISSUE_YEAR:i32 = 2020;
const MINIMUM_EXPIRATION_YEAR:i32 = 2020;
const MAXIMUM_EXPIRATION_YEAR:i32 = 2030;
const MINIMUM_HEIGHT_IN_CM:i32 = 150;
const MAXIMUM_HEIGHT_IN_CM:i32 = 193;
const MINIMUM_HEIGHT_IN_INCHES:i32 = 59;
const MAXIMUM_HEIGHT_IN_INCHES:i32 = 76;

impl Passport {
    fn build(passport_string:&str) -> Passport {
        let passport_string = passport_string.replace("\n", " ");
        let passport_variables:Vec<&str> = passport_string.split(" ").collect();
        let (
            mut birth_year,
            mut issue_year,
            mut height,
            mut height_units,
            mut hair_colour,
            mut passport_id,
            mut eye_colour,
            mut country_id,
            mut expiration_year
        ): (i32, i32, i32, String, String, String, String, i32,i32) =
            (0, 0, 0, String::new(), String::new(), String::new(), String::new(), 0, 0);
        for variable in passport_variables {
            if variable.contains("byr") 
            {
                birth_year = variable[4..].parse::<i32>().unwrap();
            } else if variable.contains("eyr") {
                expiration_year = variable[4..].parse::<i32>().unwrap();
            } else if variable.contains("iyr") {
                issue_year = variable[4..].parse::<i32>().unwrap();
            } else if variable.contains("hgt") {
                if variable[4..variable.len()-2].parse::<i32>().is_ok() {
                height = variable[4..variable.len()-2].parse::<i32>().unwrap();
            }
                height_units = variable[variable.len()-2..].to_string();
            } else if variable.contains("hcl") {
                hair_colour = variable[4..].to_string();
            } else if variable.contains("ecl") {
                eye_colour = variable[4..].to_string();
            } else if variable.contains("pid") {
                passport_id = variable[4..].to_string();
            } else if variable.contains("cid") {
                country_id = variable[4..].parse::<i32>().unwrap();
            }
        }
        return Passport {birth_year:birth_year, expiration_year:expiration_year, height:height, height_units:height_units, issue_year:issue_year, hair_colour:hair_colour, eye_colour:eye_colour, passport_id:passport_id, country_id:country_id};    
    }

    fn validate(&self) ->bool {
        return self.validate_birth_year()
                && self.validate_expiration_year()
                && self.validate_issue_year()
                && self.validate_height()
                && self.validate_eye_colour()
                && self.validate_hair_colour()
                && self.validate_passport_id();
    }

    fn validate_birth_year(&self) ->bool {
        return self.birth_year >= MINIMUM_BIRTH_YEAR && self.birth_year <= MAXIMUM_BIRTH_YEAR;
    }

    fn validate_expiration_year(&self) -> bool {
        return self.expiration_year >= MINIMUM_EXPIRATION_YEAR && self.expiration_year <= MAXIMUM_EXPIRATION_YEAR;
    }

    fn validate_issue_year(&self) -> bool {
        return self.issue_year >= MINIMUM_ISSUE_YEAR && self.issue_year <= MAXIMUM_ISSUE_YEAR;
    }

    fn validate_height(&self) -> bool {
        if self.height_units=="cm" {
            return self.height >= MINIMUM_HEIGHT_IN_CM && self.height <=MAXIMUM_HEIGHT_IN_CM
        }
        return self.height >= MINIMUM_HEIGHT_IN_INCHES && self.height <=MAXIMUM_HEIGHT_IN_INCHES
    }

    fn validate_hair_colour(&self) ->bool {
        return is_valid_colour(&self.hair_colour);
    }

    fn validate_eye_colour(&self) ->bool {
        return self.eye_colour == "hzl"
            || self.eye_colour == "amb"
            || self.eye_colour == "blu"
            || self.eye_colour == "brn"
            || self.eye_colour == "gry"
            || self.eye_colour == "grn"
            || self.eye_colour == "oth";
    }

    fn validate_passport_id(&self) -> bool {
        for c in self.passport_id.chars()
        {
            if !c.is_alphanumeric() {return false};
        }
        return self.passport_id.len()==9;
    }

    fn print(&self) {
        println!(
            "Birth year: {}\nIssue year: {}\nExpiration year: {}\nHeight: {}{}\nHair Colour: {}\nEye Colour: {}\nPassport ID: {}\nCountry ID: {}\nValid: {}"
            , self.birth_year, 
            self.issue_year, 
            self.expiration_year, 
            self.height, self.height_units,
            self.hair_colour,
            self.eye_colour,
            self.passport_id,
            self.country_id,
            self.validate()
        )
    }
}

fn count_valid_passports(passports: Vec<Passport>) -> i32 {
    let mut count: i32 = 0;
    for passport in passports {
        if passport.validate() {
            count += 1
        };
    }
    return count;
}

fn build_passports(passport_strings:Vec<&str>) -> Vec<Passport> {
    let mut passports:Vec<Passport> = Vec::new();
    for passport in passport_strings {
        passports.push(Passport::build(passport));
    }
    return passports;
}

fn is_valid_colour(colour:&str) -> bool {
    for (i,c) in colour.chars().enumerate()
    {
        if i==0 && c!='#' {return false};
        if i!=0 && !c.is_alphanumeric() {return false};
    }
    return colour.len()==7;
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
