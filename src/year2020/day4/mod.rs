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
        return self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt_value != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None;
    }

    fn is_height_valid_part_2(&self) -> bool {
        if self.hgt_value == None || self.hgt_unit == None {
            return false;
        }

        let u = self.hgt_unit.as_ref().unwrap();
        let v: u16 = *self.hgt_value.as_ref().unwrap();

        return (u == "cm" && (150..194).contains(&v)) || (u == "in" && (59..77).contains(&v));
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
        return v == "amb"
            || v == "blu"
            || v == "brn"
            || v == "gry"
            || v == "grn"
            || v == "hzl"
            || v == "oth";
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
        return self.byr != None
            && (1920..2003).contains(&self.byr.unwrap())
            && self.iyr != None
            && (2010..2021).contains(&self.iyr.unwrap())
            && self.eyr != None
            && (2020..2031).contains(&self.eyr.unwrap())
            && self.is_height_valid_part_2()
            && self.is_hcl_valid_part_2()
            && self.is_ecl_valid_part_2()
            && self.is_pid_valid_part_2();
    }
}

pub fn solve_part_1(lines: &Vec<String>) -> usize {
    return solve(lines, "part1");
}

pub fn solve_part_2(lines: &Vec<String>) -> usize {
    return solve(lines, "part2");
}

fn solve(passport_lines: &Vec<String>, part: &str) -> usize {
    let mut valid_passport_count = 0;
    let mut passport: Passport;

    let mut byr: Option<u16> = None;
    let mut iyr: Option<u16> = None;
    let mut eyr: Option<u16> = None;
    let mut hgt_unit: Option<String> = None;
    let mut hgt_value: Option<u16> = None;
    let mut hcl: Option<String> = None;
    let mut ecl: Option<String> = None;
    let mut pid: Option<String> = None;
    let mut cid: Option<u16> = None;

    // Build passports
    for passport_line in passport_lines {
        // println!("passport_line={}", passport_line);

        if passport_line.as_str().is_empty() {
            passport = Passport {
                byr: byr,
                iyr: iyr,
                eyr: eyr,
                hgt_unit: hgt_unit,
                hgt_value: hgt_value,
                hcl: hcl,
                ecl: ecl,
                pid: pid,
                cid: cid,
            };

            if part == "part1" && passport.is_valid_part_1() {
                valid_passport_count += 1;
            }

            if part == "part2" && passport.is_valid_part_2() {
                valid_passport_count += 1;
            }

            byr = None;
            iyr = None;
            eyr = None;
            hgt_unit = None;
            hgt_value = None;
            hcl = None;
            ecl = None;
            pid = None;
            cid = None;
        } else {
            // Split line at spaces
            for field in passport_line.split_whitespace() {
                // Fill in fields from line
                let (_, value) = field.split_at(4);

                if field.starts_with("byr") {
                    byr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("iyr") {
                    iyr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("eyr") {
                    eyr = Some(value.parse::<u16>().unwrap());
                }
                if field.starts_with("hgt") {
                    if field.ends_with("cm") || field.ends_with("in") {
                        let (v, u) = value.split_at(value.len() - 2);
                        hgt_unit = Some(u.to_string());
                        hgt_value = Some(v.parse::<u16>().unwrap());
                    } else {
                        hgt_value = Some(value.parse::<u16>().unwrap());
                    }
                }
                if field.starts_with("hcl") {
                    hcl = Some(value.to_string());
                }
                if field.starts_with("ecl") {
                    ecl = Some(value.to_string());
                }
                if field.starts_with("pid") {
                    pid = Some(value.to_string());
                }
                if field.starts_with("cid") {
                    cid = Some(value.parse::<u16>().unwrap());
                }
            }
        }
    }

    passport = Passport {
        byr: byr,
        iyr: iyr,
        eyr: eyr,
        hgt_unit: hgt_unit,
        hgt_value: hgt_value,
        hcl: hcl,
        ecl: ecl,
        pid: pid,
        cid: cid,
    };

    if part == "part1" && passport.is_valid_part_1() {
        valid_passport_count += 1;
    }

    if part == "part2" && passport.is_valid_part_2() {
        valid_passport_count += 1;
    }

    return valid_passport_count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day4/test_input_part_1"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 2);
    }

    #[test]
    fn part1_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day4/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 190);
    }

    #[test]
    fn part2_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day4/test_input_part_2"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 4);
    }

    #[test]
    fn part2_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day4/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 121);
    }
}
