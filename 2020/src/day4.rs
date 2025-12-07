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
//!
//! --- Part Two ---
//! The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!
//!
//! You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:
//!
//! byr (Birth Year) - four digits; at least 1920 and at most 2002.
//! iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//! eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//! hgt (Height) - a number followed by either cm or in:
//! If cm, the number must be at least 150 and at most 193.
//! If in, the number must be at least 59 and at most 76.
//! hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//! ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//! pid (Passport ID) - a nine-digit number, including leading zeroes.
//! cid (Country ID) - ignored, missing or not.
//! Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:
//!
//! byr valid:   2002
//! byr invalid: 2003
//!
//! hgt valid:   60in
//! hgt valid:   190cm
//! hgt invalid: 190in
//! hgt invalid: 190
//!
//! hcl valid:   #123abc
//! hcl invalid: #123abz
//! hcl invalid: 123abc
//!
//! ecl valid:   brn
//! ecl invalid: wat
//!
//! pid valid:   000000001
//! pid invalid: 0123456789
//! Here are some invalid passports:
//!
//! eyr:1972 cid:100
//! hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//!
//! iyr:2019
//! hcl:#602927 eyr:1967 hgt:170cm
//! ecl:grn pid:012533040 byr:1946
//!
//! hcl:dab227 iyr:2012
//! ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//!
//! hgt:59cm ecl:zzz
//! eyr:2038 hcl:74454a iyr:2023
//! pid:3556412378 byr:2007
//! Here are some valid passports:
//!
//! pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
//! hcl:#623a2f
//!
//! eyr:2029 ecl:blu cid:129 byr:1989
//! iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//!
//! hcl:#888785
//! hgt:164cm byr:2001 iyr:2015 cid:88
//! pid:545766238 ecl:hzl
//! eyr:2022
//!
//! iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
//! Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?

use std::str::FromStr;

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

fn valid_num(s: &Option<String>, min: u32, max: u32) -> bool {
    match s {
        Some(yr) => match yr.parse() {
            Ok(n) => min <= n && n <= max,
            Err(_) => false,
        },
        None => false,
    }
}

fn valid_height(s: &Option<String>) -> bool {
    if let Some(h) = s {
        if h.ends_with("cm") {
            return valid_num(&Some(h[..h.len() - 2].to_string()), 150, 193);
        };
        if h.ends_with("in") {
            return valid_num(&Some(h[..h.len() - 2].to_string()), 59, 76);
        };
    }
    false
}

fn valid_hair_color(s: &Option<String>) -> bool {
    if let Some(h) = s {
        let chars: Vec<_> = h.chars().collect();
        if chars.len() != 7 {
            return false;
        }
        for c in &chars[1..] {
            if &'0' <= c && c <= &'f' {
                continue;
            }
        }
        return true;
    }

    false
}

fn valid_eye_color(s: &Option<String>) -> bool {
    if let Some(c) = s {
        return match c.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        };
    }
    false
}

fn valid_passport_id(s: &Option<String>) -> bool {
    if let Some(pid) = s {
        if pid.len() != 9 {
            return false;
        }
        for c in pid.chars() {
            if '0' <= c && c <= '9' {
                continue;
            }
        }
        return true;
    }
    false
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part2(&self) -> bool {
        valid_num(&self.byr, 1920, 2002)
            && valid_num(&self.iyr, 2010, 2020)
            && valid_num(&self.eyr, 2020, 2030)
            && valid_height(&self.hgt)
            && valid_hair_color(&self.hcl)
            && valid_eye_color(&self.ecl)
            && valid_passport_id(&self.pid)
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(input: &str) -> Result<Passport, ()> {
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
                    s => panic!("unknown key: '{}'", s),
                };
            });
        Ok(p)
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Passport> {
    input.split("\n\n").filter_map(|s| s.parse().ok()).collect()
}

#[aoc(day4, part1)]
fn solution_part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_part1()).count()
}

#[aoc(day4, part2)]
fn solution_part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_part2()).count()
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

    #[test]
    fn invalid_part2() {
        let input = r##"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"##;
        assert_eq!(solution_part2(&parse(input)), 0);
    }

    #[test]
    fn valid_part2() {
        let input = r##"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"##;
        assert_eq!(solution_part2(&parse(input)), 4);
    }
}
