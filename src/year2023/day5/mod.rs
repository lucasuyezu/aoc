use std::{ops::Range, str::FromStr};

use crate::utils::ParseInputError;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Pah {
    pub src_range: Range<usize>,
    pub dst_range_start: usize,
}

impl FromStr for Pah {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let dst_range_start: usize = split.next().unwrap().parse().unwrap();
        let src_range_start: usize = split.next().unwrap().parse().unwrap();
        let range_size: usize = split.next().unwrap().parse().unwrap();

        Ok(Pah {
            src_range: (src_range_start..(src_range_start + range_size + 1)),
            dst_range_start,
        })
    }
}

fn convert(pahs: &Vec<Pah>, idx: &usize) -> usize {
    match pahs.iter().find(|pah| pah.src_range.contains(idx)) {
        Some(pah) => pah.dst_range_start + idx - pah.src_range.start,
        None => *idx,
    }
}

#[derive(Debug)]
pub struct Almanac {
    pub seed_to_soil: Vec<Pah>,
    pub soil_to_fertilizer: Vec<Pah>,
    pub fertilizer_to_water: Vec<Pah>,
    pub water_to_light: Vec<Pah>,
    pub light_to_temp: Vec<Pah>,
    pub temp_to_humidity: Vec<Pah>,
    pub humidity_to_loc: Vec<Pah>,
}

impl Almanac {
    pub fn min_location_number(&self, seeds_ranges: &Vec<Range<usize>>) -> usize {
        println!("Computing seeds...");

        let seeds: Vec<usize> = seeds_ranges
            .par_iter()
            .map(|seed_range| seed_range.clone().collect::<Vec<usize>>())
            .flatten()
            .collect();
        println!("Computed seeds. Computing soils...");

        let soil: Vec<usize> = seeds.par_iter().map(|s| convert(&self.seed_to_soil, s)).collect();
        println!("Computed soils. Computing fertilizers...");

        let fert: Vec<usize> = soil.par_iter().map(|s| convert(&self.soil_to_fertilizer, &s)).collect();
        println!("Computed fertilizers. Computing water...");

        let water: Vec<usize> = fert
            .par_iter()
            .map(|s| convert(&self.fertilizer_to_water, &s))
            .collect();
        println!("Computed water. Computing light...");

        let light: Vec<usize> = water.par_iter().map(|s| convert(&self.water_to_light, &s)).collect();
        println!("Computed light. Computing temps...");

        let temp: Vec<usize> = light.par_iter().map(|s| convert(&self.light_to_temp, &s)).collect();
        println!("Computed temps. Computing humidities...");

        let humidity: Vec<usize> = temp.par_iter().map(|s| convert(&self.temp_to_humidity, &s)).collect();
        println!("Computed humidities. Computing locations...");

        let loc: Vec<usize> = humidity
            .par_iter()
            .map(|s| convert(&self.humidity_to_loc, &s))
            .collect();
        println!("Computed locations. Find minimum location...");

        loc.iter().min().unwrap().clone()
    }
}

impl FromStr for Almanac {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seed_soil_str, rest) = s.split_once("\n\n").unwrap();
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
    let (seed_line, rest) = input.split_once("\n").unwrap();
    let (_, seeds_str) = seed_line.split_once(": ").unwrap();

    let mut seeds_ranges: Vec<Range<usize>> = vec![];
    let mut split_iter = seeds_str.trim().split(" ").peekable();

    while let Some(_) = split_iter.peek() {
        let range_start: usize = split_iter.next().unwrap().parse().unwrap();
        let range = range_start..(range_start + 1);
        seeds_ranges.push(range);
    }

    rest.parse::<Almanac>().unwrap().min_location_number(&seeds_ranges)
}

pub fn solve_part_2(input: &str) -> usize {
    let (seed_line, rest) = input.split_once("\n").unwrap();
    let (_, seeds_str) = seed_line.split_once(": ").unwrap();

    let mut seeds_ranges: Vec<Range<usize>> = vec![];
    let mut split_iter = seeds_str.trim().split(" ").peekable();

    while let Some(_) = split_iter.peek() {
        let range_start: usize = split_iter.next().unwrap().parse().unwrap();
        let range_size: usize = split_iter.next().unwrap().parse().unwrap();
        let range = range_start..(range_start + range_size);
        seeds_ranges.push(range);
    }

    rest.parse::<Almanac>().unwrap().min_location_number(&seeds_ranges)
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
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 46);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 11_611_182);
    }
}
