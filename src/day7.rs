extern crate nom;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Bag<'a> {
    description: &'a str,
    can_contain: Option<HashMap<&'a str, u32>>
}

pub fn parse_line(input: &str) -> Bag {
    let parts:Vec<&str> = input.split(" bags contain ").collect();
    if parts[1] == "no other bags." {
        Bag {
            description: parts[0],
            can_contain: None
        }
    } else {
        let mut hash = HashMap::new();

        for description in parts[1].split(", ") {
            // Split on the first space between count and bag description
            let bits:Vec<&str> = description.splitn(2, " ").collect();
            let count = bits[0].parse::<u32>().unwrap();
            let name:Vec<&str> = bits[1].split(" bag").collect();
            hash.insert(name[0], count);
        }

        Bag {
            description: parts[0],
            can_contain: Some(hash)
        }
    }
}

type Containment<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn containments<'a>(bags: &'a Vec<Bag>) -> Containment<'a> {
    let mut hash: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();

    for bag in bags {
        if let Some(containees) = &bag.can_contain {
            for containee in containees.keys() {
                if let Some(set) = hash.get_mut(containee) {
                    set.insert(bag.description);
                } else {
                    let mut set = HashSet::new();
                    set.insert(bag.description);
                    hash.insert(containee, set);
                }
            }
        }
    }

    hash
}

pub fn bags_hash<'a>(bags: &'a Vec<Bag>) -> HashMap<&'a str, &'a Bag<'a>> {
    bags.iter().map(|b| (b.description, b)).collect()
}

pub fn count_containees<'a>(target: &'a str, bag_hash: &'a HashMap<&'a str, &'a Bag<'a>>) -> u32 {
    let mut count = 0;

    deep_containees(target, &bag_hash, &mut count, 1);

    count
}

pub fn deep_containees<'a, 'b>(target: &'a str, bag_hash: &'a HashMap<&'a str, &'a Bag<'a>>, count: &'b mut u32, mult: u32) {
    if let Some(bag) = bag_hash.get(target) {
        if let Some(contents) = &bag.can_contain {
            for (k, v) in contents.iter() {
                let num_bags = v * mult;
                *count += num_bags;
                deep_containees(k, bag_hash, count, num_bags);
            }
        }
    }
}

pub fn all_containments<'a>(target: &'a str, containments: &'a Containment<'a>) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    deep_containments(target, &containments, &mut result);

    result
}

pub fn deep_containments<'a, 'b>(target: &'a str, containments: &'a Containment<'a>, acc: &'b mut HashSet<&'a str>) {
    if let Some(containers) = containments.get(target) {
        for container in containers {
            acc.insert(container);
            deep_containments(container, containments, acc);
        }
    }
}
