use std::convert::TryFrom;

#[derive(Debug)]
pub struct PWSpec {
    min: i32,
    max: i32,
    char: char
}

impl PWSpec {
    pub fn parse(s: &str) -> Option<Self> {
        let spec_parts: Vec<&str> = s.split(" ").collect();
        let range: Vec<&str> = spec_parts[0].split("-").collect();

        let min_fromstr = range[0].parse::<i32>().ok()?;
        let max_fromstr = range[1].parse::<i32>().ok()?;

        Some(PWSpec {
            min: min_fromstr,
            max: max_fromstr,
            char: spec_parts[1].chars().nth(0).unwrap()
        })
    }

    pub fn valid(&self, password: &str) -> bool {
        let instances = password.matches(|c| c == self.char).count();
        let instance_count = i32::try_from(instances).unwrap();

        instance_count >= self.min && instance_count <= self.max
    }
}

#[derive(Debug)]
pub struct PWLine<'a> {
    spec: PWSpec,
    password: &'a str
}

impl<'a> PWLine<'a> {
    pub fn parse(s: &'a str) -> Option<Self> {
        let parts: Vec<&str> = s.split(": ").collect();

        let spec = PWSpec::parse(parts[0])?;

        Some(
            PWLine {
                spec: spec,
                password: parts[1].clone()
            }
        )
    }

    pub fn valid(&self) -> bool {
        self.spec.valid(self.password)
    }
}
