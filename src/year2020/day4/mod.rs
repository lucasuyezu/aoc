use regex::Regex;

#[derive(Debug)]
pub struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt_unit: Option<String>,
    hgt_value: Option<u16>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Passport {
    fn is_valid_part_1(&self) -> bool {
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt_value != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }

    fn is_height_valid_part_2(&self) -> bool {
        if self.hgt_value == None || self.hgt_unit == None {
            return false;
        }

        let u = self.hgt_unit.as_ref().unwrap();
        let v: u16 = *self.hgt_value.as_ref().unwrap();

        (u == "cm" && (150..194).contains(&v)) || (u == "in" && (59..77).contains(&v))
    }

    fn is_hcl_valid_part_2(&self) -> bool {
        if self.hcl == None {
            return false;
        }

        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"\A#[0-9a-fA-F]{6}\z").unwrap();
        }

        return HCL_RE.is_match(&self.hcl.as_ref().unwrap());
    }

    fn is_ecl_valid_part_2(&self) -> bool {
        if self.ecl == None {
            return false;
        }

        let v = self.ecl.as_ref().unwrap();

        v == "amb"
            || v == "blu"
            || v == "brn"
            || v == "gry"
            || v == "grn"
            || v == "hzl"
            || v == "oth"
    }

    fn is_pid_valid_part_2(&self) -> bool {
        if self.pid == None {
            return false;
        }

        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"\A[0-9]{9}\z").unwrap();
        }

        return PID_RE.is_match(&self.pid.as_ref().unwrap());
    }

    fn is_valid_part_2(&self) -> bool {
        self.byr != None
            && (1920..2003).contains(&self.byr.unwrap())
            && self.iyr != None
            && (2010..2021).contains(&self.iyr.unwrap())
            && self.eyr != None
            && (2020..2031).contains(&self.eyr.unwrap())
            && self.is_height_valid_part_2()
            && self.is_hcl_valid_part_2()
            && self.is_ecl_valid_part_2()
            && self.is_pid_valid_part_2()
    }
}

pub fn solve_part_1(lines: &[String]) -> usize {
    solve(lines, "part1")
}

pub fn solve_part_2(lines: &[String]) -> usize {
    solve(lines, "part2")
}

fn solve(passport_lines: &[String], part: &str) -> usize {
    let mut valid_passport_count = 0;

    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt_unit: None,
        hgt_value: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };

    // Build passports
    for passport_line in passport_lines {
        // println!("passport_line={}", passport_line);

        if passport_line.as_str().is_empty() {
            if part == "part1" && passport.is_valid_part_1() {
                valid_passport_count += 1;
            }

            if part == "part2" && passport.is_valid_part_2() {
                valid_passport_count += 1;
            }

            passport.byr = None;
            passport.iyr = None;
            passport.eyr = None;
            passport.hgt_unit = None;
            passport.hgt_value = None;
            passport.hcl = None;
            passport.ecl = None;
            passport.pid = None;
            passport.cid = None;
        } else {
            // Split line at spaces
            for field in passport_line.split_whitespace() {
                // Fill in fields from line
                let (_, value) = field.split_at(4);

                if field.starts_with("byr") {
                    passport.byr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("iyr") {
                    passport.iyr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("eyr") {
                    passport.eyr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("hgt") {
                    if field.ends_with("cm") || field.ends_with("in") {
                        let (v, u) = value.split_at(value.len() - 2);
                        passport.hgt_unit = Some(u.to_string());
                        passport.hgt_value = Some(v.parse::<u16>().unwrap());
                    } else {
                        passport.hgt_value = Some(value.parse::<u16>().unwrap());
                    }
                }
                if field.starts_with("hcl") {
                    passport.hcl = Some(value.to_string());
                }
                if field.starts_with("ecl") {
                    passport.ecl = Some(value.to_string());
                }
                if field.starts_with("pid") {
                    passport.pid = Some(value.to_string());
                }
                if field.starts_with("cid") {
                    passport.cid = Some(value.parse::<u16>().unwrap());
                }
            }
        }
    }

    if part == "part1" && passport.is_valid_part_1() {
        valid_passport_count += 1;
    }

    if part == "part2" && passport.is_valid_part_2() {
        valid_passport_count += 1;
    }

    valid_passport_count
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day4/test_input_part_1"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 2);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day4/input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 190);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day4/test_input_part_2"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 4);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day4/input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 121);
    }
}
