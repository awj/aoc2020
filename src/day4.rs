#[derive(Default, Debug)]
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

    fn flag(&mut self, key: &str) {
        match key {
            "byr" => self.byr = true,
            "iyr" => self.iyr = true,
            "eyr" => self.eyr = true,
            "hgt" => self.hgt = true,
            "hcl" => self.hcl = true,
            "ecl" => self.ecl = true,
            "pid" => self.pid = true,
            _ => ()
        };
    }
}

pub fn valid_passports(input: Vec<& str>) -> i32 {
    let mut current_validation = PassportValidation::default();

    let mut num_valid = 0;

    for line in input {
        if line.is_empty() {
            if current_validation.is_valid() {
                num_valid += 1;
            }

            current_validation.reset()
        } else {
            let fragments = line.split(" ");
            for fragment in fragments {
                let mut bits = fragment.split(":");
                let key = bits.nth(0).unwrap();

                current_validation.flag(key)
            }
        }
    }

    // Catch the final passport
    if current_validation.is_valid() {
        num_valid += 1;
    }

    num_valid
}
