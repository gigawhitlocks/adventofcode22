use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct FoodItem {
    calories: u32,
}

impl fmt::Display for FoodItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.calories)
    }
}

struct Elf {
    food_items: Vec<FoodItem>,
    total: u32,
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {}", self.food_items, self.total)
    }
}

fn new_elf(calories: Vec<u32>) -> Elf {
    let mut elf = Elf {
        food_items: Vec::<FoodItem>::new(),
        total: 0,
    };

    let mut total: u32 = 0;
    for c in calories.iter() {
        total += *c;
        let f = FoodItem { calories: *c };
        elf.food_items.push(f);
    }
    elf.total = total;
    return elf;
}

fn read_elves() -> Result<Vec<Elf>, Box<dyn Error>> {
    let string_input: String = fs::read_to_string("src/input.txt")?;
    let mut elves: Vec<Elf> = Vec::new();

    let string_input_split_groups: Vec<&str> = string_input.split("\n\n").collect();
    for group in string_input_split_groups.iter() {
        let elf_strings: Vec<&str> = group.split("\n").collect();
        let mut elf_nums: Vec<u32> = vec![];
        for elf_string in elf_strings.iter() {
            if *elf_string != "" {
                let elf_num: u32 = elf_string.parse::<u32>().unwrap();
                elf_nums.push(elf_num);
            }
        }
        elves.push(new_elf(elf_nums));
    }
    return Ok(elves);
}

fn main() {
    let elves = match read_elves() {
        Ok(e) => e,
        _ => panic!(),
    };

    let mut max: Elf = new_elf(vec![0]);

    for e in elves {
        if e.total > max.total {
            max = e
        }
    }
    println!("{}", max.to_string());
}
