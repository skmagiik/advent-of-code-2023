use nom::{
    bytes::complete::{tag, take_until, is_not},
    character::complete::{
        self, digit1, line_ending, space0, space1, newline, alpha1, anychar
    },
    multi::{fold_many1, many1, separated_list1},
    sequence::{
        delimited, separated_pair, terminated, tuple, preceded
    },
    IResult, Parser, combinator::{opt, complete}, Map
};


fn main() {
    let input = include_str!("./input5.txt");
    let output = part1(input);
    println!("{}", output);
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>>{
    preceded(tag("seeds: "), separated_list1(space1, complete::u32))(input)
}

#[derive(Clone, Copy, Debug)]
enum MapTypes {
    Unknown,
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location
}

#[derive(Clone, Copy, Debug)]
struct MapInstruction {
    source: usize,
    destination: usize,
    length: usize,
    source_type: MapTypes,
    destination_type: MapTypes
}


fn parse_map_instructions(input: &str, source_type: MapTypes, destination_type: MapTypes) -> IResult<&str, MapInstruction>{
    // println!("{:?}", input);
    let values = input.split(" ")
        // .inspect(|x| println!("{}", x))
        .map(|x| x.parse::<usize>().expect("should be a value"))
        .collect::<Vec<usize>>();
    // println!("{:?}", values);/
    IResult::Ok(("", MapInstruction{source: values[1], destination: values[0], length: values[2], source_type: source_type, destination_type: destination_type}))
}

fn parse_section_to_maps(input: &str, section_index: usize) -> IResult<&str, Vec<MapInstruction>>{
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut maps: Vec<MapInstruction> = Vec::new();

    let source_type = match section_index {
        1 => MapTypes::Seed,
        2 => MapTypes::Soil,
        3 => MapTypes::Fertilizer,
        4 => MapTypes::Water,
        5 => MapTypes::Light,
        6 => MapTypes::Temp,
        7 => MapTypes::Humidity,
        _ => MapTypes::Unknown
    };

    let dest_type = match section_index {
        1 => MapTypes::Soil,
        2 => MapTypes::Fertilizer,
        3 => MapTypes::Water,
        4 => MapTypes::Light,
        5 => MapTypes::Temp,
        6 => MapTypes::Humidity,
        7 => MapTypes::Location,
        _ => MapTypes::Unknown
    };

    for line in &lines[1..]{
        let map = parse_map_instructions(line, source_type.clone(), dest_type.clone()).unwrap().1;
        // println!("{:?}", map);
        maps.push(map);
    }
    Ok((lines[0], maps))
}


impl MapInstruction{
    fn remap_single(self, seed: u32) -> u32 {
        if (seed as usize) >= self.source && (seed as usize) < (self.source+self.length) {
            // remap seed
            let offset = seed - self.source as u32;
            return self.destination as u32 + offset;
        }
        seed
    }

    fn can_remap(self, seed: u32) -> bool {
        (seed as usize) >= self.source && (seed as usize) < (self.source+self.length)
    }
}

fn part1(input: &str) -> u32 {
    let mut total_sum = 0;
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    // for (section_index, section_contents) in sections.iter().enumerate(){
        // println!("Section {}:\n{}\n", section_index, section_contents);
    // }

    let seeds = match parse_seeds(sections[0]) {
        Ok((_, s)) => s,
        _ => Vec::new()
    };
    // println!("seeds: {:?}", seeds);

    let mut section_maps: Vec<Vec<MapInstruction>> = Vec::new();

    for (section_index, section) in sections[1..].iter().enumerate(){
        let (title, seed_to_soil_maps) = parse_section_to_maps(section, section_index+1).unwrap();
        // println!("{}", title);
        // for map in &seed_to_soil_maps{
        //     println!("{:?}", map);
        // }
        // println!("");
        section_maps.push(seed_to_soil_maps);
    }

    let mut locations: Vec<u32> = Vec::new();

    for seed in seeds {
        let mut cur_seed = seed;
        // println!("mapping seed: {}", cur_seed);
        for section in &section_maps{
            for map in section {
                if map.can_remap(cur_seed){
                    // println!("Matching map: {}", cur_seed);
                    // println!("{:?}", map);
                    cur_seed = map.remap_single(cur_seed);
                    break;
                }
            }
        }
        // println!("end location: {}\n", cur_seed);
        locations.push(cur_seed);
    }

    // println!("Location: {}", locations.iter().min().unwrap());

    *locations.iter().min().unwrap()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input5.txt");
        let result = part1(input);
        assert_eq!(result, 174137457);
    }
}