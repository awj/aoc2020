#[derive(Default, Debug)]
/// A record of validation of a passport. Since passports are split across input
/// lines you likely will need this to be mutable so you can toggle flags as you
/// come across them.
struct PassportValidation {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl PassportValidation {
    fn is_valid(&self) -> bool {
        self.byr &&
            self.iyr &&
            self.eyr &&
            self.hgt &&
            self.hcl &&
            self.ecl &&
            self.pid
    }

    fn reset(&mut self) {
        self.byr = false;
        self.iyr = false;
        self.eyr = false;
        self.hgt = false;
        self.hcl = false;
        self.ecl = false;
        self.pid = false;
    }

    fn flag(&mut self, key: &str, val: &str) {
        match key {
            "byr" => self.flag_byr(val),
            "iyr" => self.flag_iyr(val),
            "eyr" => self.flag_eyr(val),
            "hgt" => self.flag_hgt(val),
            "hcl" => self.flag_hcl(val),
            "ecl" => self.flag_ecl(val),
            "pid" => self.flag_pid(val),
            _ => ()
        };
    }

    fn flag_byr(&mut self, val: &str) {
        self.byr = match val.parse::<i32>() {
            Ok(num) => num >= 1920 && num <= 2002,
            Err(_) => false
        }
    }

    fn flag_iyr(&mut self, val: &str) {
        self.iyr = match val.parse::<i32>() {
            Ok(num) => num >= 2010 && num <= 2020,
            Err(_) => false
        }
    }

    fn flag_eyr(&mut self, val: &str) {
        self.eyr = match val.parse::<i32>() {
            Ok(num) => num >= 2020 && num <= 2030,
            Err(_) => false
        }
    }

    fn flag_ecl(&mut self, val: &str) {
        // |-ing together match cases makes this pretty
        self.ecl = match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    }

    fn flag_hcl(&mut self, val: &str) {
        self.hcl = if val.starts_with("#") {
            val.len() == 7 && val[1..].chars().all(|c| {
                c.is_ascii_digit() || (c.is_ascii_alphanumeric() && c.is_ascii_lowercase())
            })
        } else {
            false
        }
    }

    fn flag_hgt(&mut self, val: &str) {
        self.hgt = if val.ends_with("cm") {
            // LOL look at all the unwrap()s!
            let num = val.split("cm").nth(0).unwrap().parse::<i32>().unwrap();
            num >= 150 && num <= 193
        } else if val.ends_with("in") {
            let num = val.split("in").nth(0).unwrap().parse::<i32>().unwrap();
            num >= 59 && num <= 76
        // if we don't have a cm|in suffix, we're not a valid height
        } else { false }
    }

    fn flag_pid(&mut self, val: &str) {
        self.pid = val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())
    }

}

pub fn valid_passports(input: Vec<& str>) -> i32 {
    // Create a mutable validation for flagging across lines / resetting on newlines
    let mut current_validation = PassportValidation::default();

    // ongoing count of passing validations
    let mut num_valid = 0;

    // for each line of input
    for line in input {
        // If the line is empty, that's the end of the passport so flag it and
        // reset the validation to start the next passport
        if line.is_empty() {
            if current_validation.is_valid() {
                num_valid += 1;
            }

            current_validation.reset()
        } else {
            // otherwise break the line into key/value parts and flag each part.
            let fragments = line.split(" ");
            for fragment in fragments {
                let bits: Vec<& str> = fragment.split(":").collect();
                let key = bits[0];
                let val = bits[1];

                current_validation.flag(key, val)
            }
        }
    }

    // Catch the final passport in case we didn't see an empty line after it
    if current_validation.is_valid() {
        num_valid += 1;
    }

    num_valid
}
