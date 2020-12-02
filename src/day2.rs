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
        // min/max should "always" be a valid index into the password, so we can
        // just panic! if we either cannot convert it to `usize` or we go out of
        // bounds on the character list.
        let first_index = usize::try_from(self.min - 1).unwrap();
        let first_char = password.chars().nth(first_index).unwrap();

        let second_index = usize::try_from(self.max - 1).unwrap();
        let second_char = password.chars().nth(second_index).unwrap();

        ( first_char == self.char && second_char != self.char ) ||
            ( first_char != self.char && second_char == self.char )
    }

    pub fn old_valid(&self, password: &str) -> bool {
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

// Note: we're saying that the string slice going into `PWLine` as the password
// cannot outlive the input string we're parsing (since that's what it
// references).
//
// Seems like maybe we could/should use a dependent lifetime (impl<'a: 'b, 'b>
// ...) here for flexibility so that string slice can have an
// independent-but-smaller lifetime than the input? Can't figure out how to get
// that to compile, nor do I understand errors, so I guess this is good
// enough...
impl<'a> PWLine<'a> {
    pub fn parse(s: &'a str) -> Option<Self> {
        let parts: Vec<&str> = s.split(": ").collect();

        let spec = PWSpec::parse(parts[0])?;

        Some(
            PWLine {
                spec: spec,
                password: parts[1]
            }
        )
    }

    pub fn valid(&self) -> bool {
        self.spec.valid(self.password)
    }
}
