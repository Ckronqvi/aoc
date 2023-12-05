//Thanks AmmarAbouZor for the solution

use std::ops::RangeInclusive;

#[derive(Debug)]
struct RangeDiff {
    src_rng: RangeInclusive<isize>,
    diff: isize,
}

impl RangeDiff {
    fn new(src_rng: RangeInclusive<isize>, diff: isize) -> Self {
        Self { src_rng, diff }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<isize>,
    maps: Vec<Vec<RangeDiff>>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let mut seeds = parts.next().unwrap();
        seeds = seeds.strip_prefix("seeds: ").unwrap();
        let seeds: Vec<isize> = seeds
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let maps = parts
            .map(|range_txt| {
                // first line is irrelevant
                let (_, range_txt) = range_txt.split_once('\n').unwrap();

                range_txt
                    .lines()
                    .map(|line| {
                        let parts: Vec<isize> = line
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect();
                        let source_rng = RangeInclusive::new(parts[1], parts[1] + parts[2]);
                        let diff = parts[0] - parts[1];

                        RangeDiff::new(source_rng, diff)
                    })
                    .collect()
            })
            .collect();

        Almanac { seeds, maps }
    }
}

impl Almanac {
    
    fn generate_ranges(&self) -> Vec<RangeInclusive<isize>> {
        self.seeds.chunks(2).fold(Vec::new(), |mut ranges, rng| {
            ranges.push(rng[0]..=rng[0] + rng[1] - 1);
            ranges
        })
    }

    fn find_min_location_ranges(&self) -> isize {
        let start_ranges = self.generate_ranges();

        let final_ranges = self.maps.iter().fold(start_ranges, |ranges, map| {
            ranges
                .into_iter()
                .flat_map(|rng| self.solve_ranges(map, rng))
                .collect()
        });

        final_ranges.iter().map(|rng| *rng.start()).min().unwrap()
    }

    fn solve_ranges(
        &self,
        rng_diffs: &[RangeDiff],
        rng: RangeInclusive<isize>,
    ) -> Vec<RangeInclusive<isize>> {
        let mut ranges = Vec::new();

        // This will be used to find out which chunks of the range aren't mapped
        let mut temp_ranges = Vec::new();

        for rng_diff in rng_diffs {
            let src_rng = &rng_diff.src_rng;
            match (src_rng.contains(rng.start()), src_rng.contains(rng.end())) {
                // Inside
                (true, true) => {
                    ranges.push(*rng.start() + rng_diff.diff..=*rng.end() + rng_diff.diff);
                    temp_ranges.push(*rng.start()..=*rng.end());
                }
                // Start inside. End outside
                (true, false) => {
                    ranges.push(*rng.start() + rng_diff.diff..=*src_rng.end() + rng_diff.diff);
                    temp_ranges.push(*rng.start()..=*src_rng.end());
                }
                // Start outside. End inside
                (false, true) => {
                    ranges.push(*src_rng.start() + rng_diff.diff..=*rng.end() + rng_diff.diff);
                    temp_ranges.push(*src_rng.start()..=*rng.end());
                }
                (false, false) => {
                    let overlapp = rng.start() < src_rng.start() && rng.end() > src_rng.end();
                    if overlapp {
                        ranges.push(
                            *src_rng.start() + rng_diff.diff..=*src_rng.end() + rng_diff.diff,
                        );
                        temp_ranges.push(*src_rng.start()..=*src_rng.end());
                    }
                }
            }
        }

        // Fill out the unmapped chunks
        temp_ranges.sort_unstable_by_key(|rng| *rng.start());

        if ranges.is_empty() {
            ranges.push(rng.clone());
        } else {
            let mut bound_to_check = *rng.start();
            for tmp_rng in temp_ranges {
                if bound_to_check < *tmp_rng.start() {
                    ranges.push(bound_to_check..=*tmp_rng.start() - 1);
                }
                bound_to_check = *tmp_rng.end();
            }
        }

        ranges
    }
}


fn find_lowes_ranges(input: &str) -> isize {
    let almanac = Almanac::from(input);
    almanac.find_min_location_ranges()
}


fn part_2(input: &str) {
    let answer_2 = find_lowes_ranges(input);

    println!("Part 2 answer is {answer_2}");
}


fn main() {
    let input = std::fs::read_to_string("src/inputreal.txt").unwrap();
    part_2(&input);
}
