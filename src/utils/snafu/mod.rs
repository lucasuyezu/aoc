use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
    string::ParseError,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Snafu {
    pub value: isize,
}

impl Add for Snafu {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Snafu {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            value: self.value + rhs.value,
        }
    }
}

impl FromStr for Snafu {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut multiplier = 1;
        let mut value = 0;

        for c in s.chars().rev() {
            let digit = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                d @ _ => panic!("Invalid digit '{}'", d),
            };

            value += digit * multiplier;

            multiplier *= 5;
        }

        Ok(Snafu { value })
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut value = self.value;
        let mut rem;

        while value != 0 {
            rem = (value + 2).rem_euclid(5);
            value = (value + 2) / 5;

            result.insert(
                0,
                match rem {
                    0 => '=',
                    1 => '-',
                    2 => '0',
                    3 => '1',
                    4 => '2',
                    d @ _ => panic!("Unsupported rem {}", d),
                },
            );
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_snafu() {
        assert_eq!("1".parse::<Snafu>().unwrap(), Snafu { value: 1 });
        assert_eq!("2".parse::<Snafu>().unwrap(), Snafu { value: 2 });
        assert_eq!("1=".parse::<Snafu>().unwrap(), Snafu { value: 3 });
        assert_eq!("1-".parse::<Snafu>().unwrap(), Snafu { value: 4 });
        assert_eq!("10".parse::<Snafu>().unwrap(), Snafu { value: 5 });
        assert_eq!("11".parse::<Snafu>().unwrap(), Snafu { value: 6 });
        assert_eq!("12".parse::<Snafu>().unwrap(), Snafu { value: 7 });
        assert_eq!("2=".parse::<Snafu>().unwrap(), Snafu { value: 8 });
        assert_eq!("2-".parse::<Snafu>().unwrap(), Snafu { value: 9 });
        assert_eq!("20".parse::<Snafu>().unwrap(), Snafu { value: 10 });
        assert_eq!("1=0".parse::<Snafu>().unwrap(), Snafu { value: 15 });
        assert_eq!("1-0".parse::<Snafu>().unwrap(), Snafu { value: 20 });
        assert_eq!("1=11-2".parse::<Snafu>().unwrap(), Snafu { value: 2022 });
        assert_eq!("1-0---0".parse::<Snafu>().unwrap(), Snafu { value: 12345 });
        assert_eq!("1121-1110-1=0".parse::<Snafu>().unwrap(), Snafu { value: 314159265 });
    }

    #[test]
    fn test_snafu_to_string() {
        assert_eq!(Snafu { value: 1 }.to_string(), "1");
        assert_eq!(Snafu { value: 2 }.to_string(), "2");
        assert_eq!(Snafu { value: 3 }.to_string(), "1=");
        assert_eq!(Snafu { value: 4 }.to_string(), "1-");
        assert_eq!(Snafu { value: 5 }.to_string(), "10");
        assert_eq!(Snafu { value: 6 }.to_string(), "11");
        assert_eq!(Snafu { value: 7 }.to_string(), "12");
        assert_eq!(Snafu { value: 8 }.to_string(), "2=");
        assert_eq!(Snafu { value: 9 }.to_string(), "2-");
        assert_eq!(Snafu { value: 10 }.to_string(), "20");
        assert_eq!(Snafu { value: 15 }.to_string(), "1=0");
        assert_eq!(Snafu { value: 20 }.to_string(), "1-0");
        assert_eq!(Snafu { value: 2022 }.to_string(), "1=11-2");
        assert_eq!(Snafu { value: 12345 }.to_string(), "1-0---0");
        assert_eq!(Snafu { value: 314159265 }.to_string(), "1121-1110-1=0");
    }
}
