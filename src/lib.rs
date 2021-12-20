mod deck;
pub mod find_all_sets;
mod set;

use std::{
    collections::HashMap, error::Error, fmt, fs, fs::File, io::prelude::*, path::Path, str::FromStr,
};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum HandType {
    Ascending,
    Descending,
}

impl fmt::Display for HandType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandType::Ascending => fmt.write_str("ascending")?,
            HandType::Descending => fmt.write_str("descending")?,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseTypeError;
impl std::error::Error for ParseTypeError {}
impl fmt::Display for ParseTypeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("not a valid hand type")?;
        Ok(())
    }
}

impl FromStr for HandType {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ascending" {
            Ok(HandType::Ascending)
        } else if s == "descending" {
            Ok(HandType::Descending)
        } else {
            Err(ParseTypeError)
        }
    }
}

/* records info about a hand */
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Info {
    set_count: usize, //number of sets found in hand
    hand_size: usize, //number of cards in the hand
    deals: usize,     //how many times cards have been removed from deck
    hand_type: HandType,
}

impl Info {
    fn serialize(&self) -> String {
        self.set_count.to_string()
            + ","
            + &self.hand_size.to_string()
            + ","
            + &self.deals.to_string()
            + ","
            + &self.hand_type.to_string()
    }
}

pub fn write_out(data: &HashMap<Info, u64>, file_name: &str) {
    let serialized = String::from("sets,hand_size,deals,hand_type,count\n")
        + &itertools::join(
            data.iter()
                .map(|(info, count)| info.serialize() + "," + &count.to_string()),
            "\n",
        );

    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => log::info!("wrote data to {}", display),
    }
}

pub fn consolidate() -> Result<(), Box<dyn Error + 'static>> {
    let mut data: HashMap<Info, u64> = HashMap::new();
    let data_dir = Path::new("data/");
    for file in fs::read_dir(data_dir)? {
        let path = file?.path();
        let contents = fs::read_to_string(path)?;

        let mut rdr = csv::Reader::from_reader(contents.as_bytes());

        for result in rdr.records() {
            let record = result.unwrap();
            let info = Info {
                set_count: record[0].parse()?,
                hand_size: record[1].parse()?,
                deals: record[2].parse()?,
                hand_type: record[3].parse()?,
            };
            let count: u64 = record[4].parse()?;

            let val = data.entry(info).or_insert(0);
            *val += count;
        }
    }

    write_out(&data, "data.csv");
    Ok(())
}
