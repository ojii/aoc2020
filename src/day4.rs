use crate::day4::ValidatedData::Valid;
use crate::maybe_from::MaybeFrom;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter::FromIterator;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> = {
        let mut s = HashSet::with_capacity(7);
        s.insert("ecl");
        s.insert("pid");
        s.insert("eyr");
        s.insert("hcl");
        s.insert("byr");
        s.insert("iyr");
        s.insert("hgt");
        s
    };
}

#[derive(Debug)]
enum Height {
    Centimeters(i32),
    Inches(i32),
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl MaybeFrom<&str> for EyeColor {
    fn maybe_from(value: &str) -> Option<Self> {
        match value {
            "amb" => Some(EyeColor::Amber),
            "blu" => Some(EyeColor::Blue),
            "brn" => Some(EyeColor::Brown),
            "gry" => Some(EyeColor::Gray),
            "grn" => Some(EyeColor::Green),
            "hzl" => Some(EyeColor::Hazel),
            "oth" => Some(EyeColor::Other),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ValidatedData<T: Debug> {
    Valid(T),
    Invalid(String),
}

impl<T: Debug> ValidatedData<T> {
    fn is_valid(&self) -> bool {
        match self {
            ValidatedData::Valid(_) => true,
            ValidatedData::Invalid(_) => false,
        }
    }
}

impl ValidatedData<i32> {
    fn byr(value: &str) -> ValidatedData<i32> {
        value
            .parse::<i32>()
            .ok()
            .and_then(|year| {
                if year >= 1920 && year <= 2002 {
                    Some(year)
                } else {
                    None
                }
            })
            .map(|year| ValidatedData::Valid(year))
            .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
    }

    fn iyr(value: &str) -> ValidatedData<i32> {
        value
            .parse::<i32>()
            .ok()
            .and_then(|year| {
                if year >= 2010 && year <= 2020 {
                    Some(year)
                } else {
                    None
                }
            })
            .map(|year| ValidatedData::Valid(year))
            .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
    }

    fn eyr(value: &str) -> ValidatedData<i32> {
        value
            .parse::<i32>()
            .ok()
            .and_then(|year| {
                if year >= 2020 && year <= 2030 {
                    Some(year)
                } else {
                    None
                }
            })
            .map(|year| ValidatedData::Valid(year))
            .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
    }
}

impl ValidatedData<Height> {
    fn hgt(value: &str) -> ValidatedData<Height> {
        if value.ends_with("cm") {
            value
                .chars()
                .take(value.chars().count() - 2)
                .collect::<String>()
                .parse::<i32>()
                .ok()
                .and_then(|num| {
                    if num >= 150 && num <= 193 {
                        Some(num)
                    } else {
                        None
                    }
                })
                .map(|num| ValidatedData::Valid(Height::Centimeters(num)))
                .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
        } else if value.ends_with("in") {
            value
                .chars()
                .take(value.chars().count() - 2)
                .collect::<String>()
                .parse::<i32>()
                .ok()
                .and_then(|num| {
                    if num >= 59 && num <= 76 {
                        Some(num)
                    } else {
                        None
                    }
                })
                .map(|num| ValidatedData::Valid(Height::Inches(num)))
                .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
        } else {
            ValidatedData::Invalid(value.to_string())
        }
    }
}

impl ValidatedData<String> {
    fn hcl(value: &str) -> ValidatedData<String> {
        let mut chars = value.chars();
        if chars.next() == Some('#')
            && chars.all(|c| c.is_ascii_hexdigit())
            && value.chars().count() == 7
        {
            ValidatedData::Valid(value.to_string())
        } else {
            ValidatedData::Invalid(value.to_string())
        }
    }

    fn pid(value: &str) -> ValidatedData<String> {
        if value.chars().count() == 9 && value.chars().all(|c| c.is_ascii_digit()) {
            ValidatedData::Valid(value.to_string())
        } else {
            ValidatedData::Invalid(value.to_string())
        }
    }

    fn cid(value: &str) -> ValidatedData<String> {
        ValidatedData::Valid(value.to_string())
    }
}

impl ValidatedData<EyeColor> {
    fn ecl(value: &str) -> ValidatedData<EyeColor> {
        EyeColor::maybe_from(value)
            .map(|ec| ValidatedData::Valid(ec))
            .unwrap_or_else(|| ValidatedData::Invalid(value.to_string()))
    }
}

#[derive(Debug)]
struct Passport {
    ecl: ValidatedData<EyeColor>,
    pid: ValidatedData<String>,
    eyr: ValidatedData<i32>,
    hcl: ValidatedData<String>,
    byr: ValidatedData<i32>,
    iyr: ValidatedData<i32>,
    cid: Option<ValidatedData<String>>,
    hgt: ValidatedData<Height>,
}

impl Passport {
    fn errors(&self) -> usize {
        [
            self.ecl.is_valid(),
            self.pid.is_valid(),
            self.eyr.is_valid(),
            self.hcl.is_valid(),
            self.byr.is_valid(),
            self.iyr.is_valid(),
            self.hgt.is_valid(),
        ]
        .iter()
        .filter(|v| !*v)
        .count()
    }
    fn is_valid(&self) -> bool {
        self.errors() == 0
    }
}

impl MaybeFrom<&str> for Passport {
    fn maybe_from(value: &str) -> Option<Self> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for part in value.split_whitespace() {
            let (key, value) = part.split_once(":")?;
            fields.insert(key, value);
        }
        if REQUIRED_FIELDS.is_subset(&HashSet::from_iter(fields.keys().map(|k| k.clone()))) {
            Some(Passport {
                ecl: ValidatedData::ecl(fields.get("ecl").unwrap()),
                pid: ValidatedData::pid(fields.get("pid").unwrap()),
                eyr: ValidatedData::eyr(fields.get("eyr").unwrap()),
                hcl: ValidatedData::hcl(fields.get("hcl").unwrap()),
                byr: ValidatedData::byr(fields.get("byr").unwrap()),
                iyr: ValidatedData::iyr(fields.get("iyr").unwrap()),
                cid: fields.get("cid").map(|s| ValidatedData::cid(s)),
                hgt: ValidatedData::hgt(fields.get("hgt").unwrap()),
            })
        } else {
            None
        }
    }
}

pub fn run() {
    let input = include_str!("data/4/1");
    let passports: Vec<Passport> = input
        .split("\n\n")
        .flat_map(|part| Passport::maybe_from(part))
        .collect();
    println!("{}", passports.len());
    println!("{}", passports.iter().filter(|p| p.is_valid()).count());
    for passport in passports {
        if !passport.is_valid() {
            println!("{}: {:?}", passport.errors(), passport)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::Passport;
    use crate::maybe_from::MaybeFrom;

    #[test]
    fn valid_passports() {
        let passports: Vec<Passport> = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
        .split("\n\n")
        .flat_map(|part| Passport::maybe_from(part))
        .collect();
        assert_eq!(passports.len(), 4);
        assert!(passports.iter().all(|passport| passport.is_valid()));
    }

    #[test]
    fn invalid_passports() {
        let passports: Vec<Passport> = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"
        .split("\n\n")
        .flat_map(|part| Passport::maybe_from(part))
        .collect();
        assert_eq!(passports.len(), 4);
        assert!(passports.iter().all(|passport| !passport.is_valid()));
    }
}
