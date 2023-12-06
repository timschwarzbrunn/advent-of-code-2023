use itertools::Itertools;
use std::io::BufRead;
use std::ops::Range;

#[derive(Debug)]
struct RangeMapper {
    destination_range: Range<usize>,
    source_range: Range<usize>,
}

#[derive(Debug)]
struct Mapper {
    range_mappers: Vec<RangeMapper>,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<Range<usize>>,
    mappers: Vec<Mapper>,
}

impl RangeMapper {
    fn from_line(line: String) -> Self {
        // Example line: 50 98 2
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Self {
            destination_range: (parts[0]..parts[0] + parts[2]),
            source_range: (parts[1]..parts[1] + parts[2]),
        }
    }

    fn convert_number(&self, input: usize) -> usize {
        if self.source_range.contains(&input) {
            self.destination_range.start + (input - self.source_range.start)
        } else {
            input
        }
    }

    fn convert_range(&self, input_range: Range<usize>) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
        let mut ranges_changed: Vec<Range<usize>> = Vec::new();
        let mut ranges_unchanged: Vec<Range<usize>> = Vec::new();
        if self.source_range.contains(&input_range.start)
            && self.source_range.contains(&(&input_range.end - 1))
        {
            // Everything is transformed.
            ranges_changed.push(
                self.convert_number(input_range.start)
                    ..self.convert_number(input_range.end - 1) + 1,
            );
        } else if self.source_range.contains(&input_range.start)
            && !self.source_range.contains(&(&input_range.end - 1))
        {
            // The first part needs to be transformed, the last part will stay.
            ranges_changed.push(self.convert_number(input_range.start)..self.destination_range.end);
            ranges_unchanged.push(self.source_range.end..input_range.end);
        } else if !self.source_range.contains(&input_range.start)
            && self.source_range.contains(&(&input_range.end - 1))
        {
            // The first part will stay, the last part needs to be transformed.
            ranges_unchanged.push(input_range.start..self.source_range.start);
            ranges_changed
                .push(self.destination_range.start..self.convert_number(input_range.end - 1) + 1);
        } else if input_range.start < self.source_range.start
            && input_range.end > self.source_range.end
        {
            // It is overlapping. The first part will stay, the middle part needs to be transformed, the last part will stay.
            ranges_unchanged.push(input_range.start..self.source_range.start);
            ranges_changed.push(self.destination_range.clone());
            ranges_unchanged.push(self.source_range.end..input_range.end);
        } else {
            // It is not overlapping.
            ranges_unchanged.push(input_range);
        }
        (ranges_changed, ranges_unchanged)
    }
}

impl Mapper {
    fn new() -> Self {
        Self {
            range_mappers: Vec::new(),
        }
    }

    fn add_range_mapper(&mut self, line: String) {
        self.range_mappers.push(RangeMapper::from_line(line));
    }

    fn convert_ranges(&self, input_ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut result_ranges: Vec<Range<usize>> = Vec::new();
        let mut ranges_still_to_consider: Vec<Range<usize>> = input_ranges.clone();
        for range_mapper in &self.range_mappers {
            let mut ranges_to_consider_for_next_mapper: Vec<Range<usize>> = Vec::new();
            for range in &ranges_still_to_consider {
                let (mut ranges_changed, mut ranges_unchanged) =
                    range_mapper.convert_range(range.clone());
                result_ranges.append(&mut ranges_changed);
                ranges_to_consider_for_next_mapper.append(&mut ranges_unchanged);
            }
            ranges_still_to_consider = ranges_to_consider_for_next_mapper;
        }
        result_ranges.append(&mut ranges_still_to_consider);
        result_ranges
    }
}

pub enum SeedMode {
    Single,
    Range,
}

enum AlmanacParser {
    ReadSeed,
    EmptyLine,
    MapperName,
    RangeMapper,
}

impl AlmanacParser {
    fn next_state(&self) -> Self {
        match self {
            AlmanacParser::ReadSeed => AlmanacParser::EmptyLine,
            AlmanacParser::EmptyLine => AlmanacParser::MapperName,
            AlmanacParser::MapperName => AlmanacParser::RangeMapper,
            // RangeMapper stays RangeMapper until it is set from the outside.
            AlmanacParser::RangeMapper => AlmanacParser::RangeMapper,
        }
    }
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            mappers: Vec::new(),
        }
    }

    pub fn from_reader<B: BufRead>(reader: B, seed_mode: SeedMode) -> Self {
        let mut almanac = Almanac::new();
        let mut parser_state: AlmanacParser = AlmanacParser::ReadSeed;
        for line in reader.lines() {
            let line = line.unwrap();
            match parser_state {
                AlmanacParser::ReadSeed => {
                    // Example line: seeds: 79 14 55 13
                    match seed_mode {
                        SeedMode::Single => {
                            almanac.seeds = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .split_whitespace()
                                .map(|s| s.parse::<usize>().unwrap())
                                .map(|range_start| range_start..range_start + 1)
                                .collect();
                        }
                        SeedMode::Range => {
                            almanac.seeds = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .split_whitespace()
                                .map(|s| s.parse::<usize>().unwrap())
                                .chunks(2)
                                .into_iter()
                                .map(|mut chunk| {
                                    let range_start = chunk.next().unwrap();
                                    let range_length = chunk.next().unwrap();
                                    range_start..range_start + range_length
                                })
                                .collect()
                        }
                    }
                }
                AlmanacParser::EmptyLine => {
                    // Do nothing.
                }
                AlmanacParser::MapperName => {
                    // Create a new mapper.
                    almanac.mappers.push(Mapper::new());
                }
                AlmanacParser::RangeMapper => {
                    if line.is_empty() {
                        // Nothing to do here, we will get a new mapper.
                        parser_state = AlmanacParser::EmptyLine;
                    } else {
                        // Add a new RangeMapper to the last appended Mapper.
                        if let Some(last_mapper) = almanac.mappers.last_mut() {
                            last_mapper.add_range_mapper(line);
                        } else {
                            println!("Error: Cannot add range mapper.");
                        }
                    }
                }
            }
            parser_state = parser_state.next_state();
        }
        almanac
    }

    pub fn get_min_location(&self) -> usize {
        let mut ranges_for_next_run = self.seeds.clone();
        for mapper in &self.mappers {
            ranges_for_next_run = mapper.convert_ranges(ranges_for_next_run);
        }
        ranges_for_next_run
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_converter_overlapping() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![0..30]),
            vec![(20..30), (0..10), (20..30)]
        );
    }

    #[test]
    fn test_range_converter_inside() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![11..19]),
            vec![21..29]
        );
    }

    #[test]
    fn test_range_converter_left() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![10..30]),
            vec![(20..30), (20..30)]
        );
    }

    #[test]
    fn test_range_converter_right() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![0..20]),
            vec![(20..30), (0..10)]
        );
    }

    #[test]
    fn test_range_converter_single_inside() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![10..11]),
            vec![(20..21)]
        );
    }

    #[test]
    fn test_range_converter_single_outside() {
        assert_eq!(
            Mapper {
                range_mappers: vec![RangeMapper {
                    destination_range: 20..30,
                    source_range: 10..20,
                }],
            }
            .convert_ranges(vec![0..1]),
            vec![(0..1)]
        );
    }

    #[test]
    fn test_simple_almanac() {
        let almanac = Almanac {
            seeds: vec![20..40, 50..60],
            mappers: vec![
                Mapper {
                    range_mappers: vec![
                        RangeMapper {
                            destination_range: 10..20,
                            source_range: 50..60,
                        },
                        RangeMapper {
                            destination_range: 80..90,
                            source_range: 90..100,
                        },
                        RangeMapper {
                            destination_range: 65..67,
                            source_range: 63..65,
                        },
                    ],
                },
                Mapper {
                    range_mappers: vec![RangeMapper {
                        destination_range: 50..60,
                        source_range: 40..50,
                    }],
                },
            ],
        };
        assert_eq!(almanac.get_min_location(), 10);
    }
}
