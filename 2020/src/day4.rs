//! --- Day 4: Passport Processing ---
//! You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.
//!
//! It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.
//!
//! Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.
//!
//! The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:
//!
//! byr (Birth Year)
//! iyr (Issue Year)
//! eyr (Expiration Year)
//! hgt (Height)
//! hcl (Hair Color)
//! ecl (Eye Color)
//! pid (Passport ID)
//! cid (Country ID)
//! Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.
//!
//! Here is an example batch file containing four passports:
//!
//! ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
//! byr:1937 iyr:2017 cid:147 hgt:183cm
//!
//! iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
//! hcl:#cfa07d byr:1929
//!
//! hcl:#ae17e1 iyr:2013
//! eyr:2024
//! ecl:brn pid:760753108 byr:1931
//! hgt:179cm
//!
//! hcl:#cfa07d eyr:2025 pid:166559648
//! iyr:2011 ecl:brn hgt:59in
//! The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).
//!
//! The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.
//!
//! The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.
//!
//! According to the above rules, your improved system would report 2 valid passports.
//!
//! Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, PartialEq)]
struct Passport {
    // Birth Year
    byr: Option<String>,
    // Issue Year
    iyr: Option<String>,
    // Expiration Year
    eyr: Option<String>,
    // Height
    hgt: Option<String>,
    // Hair Color
    hcl: Option<String>,
    // Eye Color
    ecl: Option<String>,
    // Passport ID
    pid: Option<String>,
    // Country ID
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl From<&str> for Passport {
    fn from(input: &str) -> Passport {
        let mut p = Passport::default();
        input
            .replace('\n', " ")
            .split(' ')
            .filter(|p| !p.is_empty())
            .for_each(|part| {
                let (k, v) = part.split_at(part.find(":").unwrap());
                match k {
                    "byr" => p.byr = Some(v[1..].to_string()),
                    "iyr" => p.iyr = Some(v[1..].to_string()),
                    "eyr" => p.eyr = Some(v[1..].to_string()),
                    "hgt" => p.hgt = Some(v[1..].to_string()),
                    "hcl" => p.hcl = Some(v[1..].to_string()),
                    "ecl" => p.ecl = Some(v[1..].to_string()),
                    "pid" => p.pid = Some(v[1..].to_string()),
                    "cid" => p.cid = Some(v[1..].to_string()),
                    s => panic!(format!("unknown key: '{}'", s)),
                };
            });
        p
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::from).collect()
}

#[aoc(day4, part1)]
fn solution_part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r##"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"##;

    #[test]
    fn parse_passports() {
        assert_eq!(
            parse(INPUT),
            vec![
                Passport {
                    ecl: Some("gry".to_string()),
                    pid: Some("860033327".to_string()),
                    eyr: Some("2020".to_string()),
                    hcl: Some("#fffffd".to_string()),
                    byr: Some("1937".to_string()),
                    iyr: Some("2017".to_string()),
                    cid: Some("147".to_string()),
                    hgt: Some("183cm".to_string()),
                },
                Passport {
                    iyr: Some("2013".to_string()),
                    ecl: Some("amb".to_string()),
                    cid: Some("350".to_string()),
                    eyr: Some("2023".to_string()),
                    pid: Some("028048884".to_string()),
                    hcl: Some("#cfa07d".to_string()),
                    byr: Some("1929".to_string()),
                    ..Default::default()
                },
                Passport {
                    hcl: Some("#ae17e1".to_string()),
                    iyr: Some("2013".to_string()),
                    eyr: Some("2024".to_string()),
                    ecl: Some("brn".to_string()),
                    pid: Some("760753108".to_string()),
                    byr: Some("1931".to_string()),
                    hgt: Some("179cm".to_string()),
                    ..Default::default()
                },
                Passport {
                    hcl: Some("#cfa07d".to_string()),
                    eyr: Some("2025".to_string()),
                    pid: Some("166559648".to_string()),
                    iyr: Some("2011".to_string()),
                    ecl: Some("brn".to_string()),
                    hgt: Some("59in".to_string()),
                    ..Default::default()
                },
            ]
        );
    }
}
