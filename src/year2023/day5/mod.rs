use std::{collections::HashSet, str::FromStr};

use crate::utils::ParseInputError;

#[derive(Debug)]
pub struct Pah {
    pub src_range_start: usize,
    pub dst_range_start: usize,
    pub range_size: usize,
}

impl FromStr for Pah {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let dst_range_start: usize = split.next().unwrap().parse().unwrap();
        let src_range_start: usize = split.next().unwrap().parse().unwrap();
        let range_size: usize = split.next().unwrap().parse().unwrap();

        Ok(Pah {
            src_range_start,
            dst_range_start,
            range_size,
        })
    }
}

fn convert(pahs: &Vec<Pah>, idx: &usize) -> usize {
    let result = if let Some(pah) = pahs
        .iter()
        .find(|pah| (pah.src_range_start..(pah.src_range_start + pah.range_size + 1)).contains(idx))
    {
        let offset = idx - pah.src_range_start;
        pah.dst_range_start + offset
    } else {
        *idx
    };

    println!("idx {} -> result {}", idx, result);
    result
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: HashSet<usize>,
    pub seed_to_soil: Vec<Pah>,
    pub soil_to_fertilizer: Vec<Pah>,
    pub fertilizer_to_water: Vec<Pah>,
    pub water_to_light: Vec<Pah>,
    pub light_to_temp: Vec<Pah>,
    pub temp_to_humidity: Vec<Pah>,
    pub humidity_to_loc: Vec<Pah>,
}

impl Almanac {
    pub fn min_location_number(&self) -> usize {
        let x: Vec<usize> = self.seeds.iter().map(|s| convert(&self.seed_to_soil, s)).collect();

        x.iter()
            .map(|s| convert(&self.soil_to_fertilizer, &s))
            .map(|s| convert(&self.fertilizer_to_water, &s))
            .map(|s| convert(&self.water_to_light, &s))
            .map(|s| convert(&self.light_to_temp, &s))
            .map(|s| convert(&self.temp_to_humidity, &s))
            .map(|s| convert(&self.humidity_to_loc, &s))
            .min()
            .unwrap()
            .clone()
    }
}

impl FromStr for Almanac {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seed_line, rest) = s.split_once("\n").unwrap();
        let (_, seeds_str) = seed_line.split_once(": ").unwrap();

        let mut seeds = HashSet::new();
        for s in seeds_str.trim().split(" ") {
            seeds.insert(s.parse().unwrap());
        }

        let (seed_soil_str, rest) = rest.split_once("\n\n").unwrap();
        let seed_to_soil: Vec<Pah> = build_vec_pah(seed_soil_str);

        let (soil_to_fertilizer_str, rest) = rest.split_once("\n\n").unwrap();
        let soil_to_fertilizer: Vec<Pah> = build_vec_pah(soil_to_fertilizer_str);

        let (fertilizer_to_water_str, rest) = rest.split_once("\n\n").unwrap();
        let fertilizer_to_water: Vec<Pah> = build_vec_pah(fertilizer_to_water_str);

        let (water_to_light_str, rest) = rest.split_once("\n\n").unwrap();
        let water_to_light: Vec<Pah> = build_vec_pah(water_to_light_str);

        let (light_to_temp_str, rest) = rest.split_once("\n\n").unwrap();
        let light_to_temp: Vec<Pah> = build_vec_pah(light_to_temp_str);

        let (temp_to_humidity_str, rest) = rest.split_once("\n\n").unwrap();
        let temp_to_humidity: Vec<Pah> = build_vec_pah(temp_to_humidity_str);

        let humidity_to_loc: Vec<Pah> = build_vec_pah(rest);

        Ok(Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_loc,
        })
    }
}

fn build_vec_pah(s: &str) -> Vec<Pah> {
    s.trim()
        .split_once("\n")
        .unwrap()
        .1
        .lines()
        .map(|s| s.parse::<Pah>().unwrap())
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    input.parse::<Almanac>().unwrap().min_location_number()
}

pub fn solve_part_2(_input: &str) -> usize {
    // input.lines().map(|line| line.parse::<Pah>().unwrap())

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 35);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 173_706_076);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 30);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 8_172_507);
    }
}
