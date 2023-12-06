use core::time;
use std::vec;

use nom::{
    bytes::complete::{tag, take_until, is_not},
    character::complete::{
        self, digit1, line_ending,  multispace0, multispace1, space0, space1, newline, alpha1, anychar
    },
    multi::{fold_many1, many1, separated_list1},
    sequence::{
        delimited, separated_pair, terminated, tuple, preceded
    },
    IResult, Parser, combinator::{opt, complete}, Map
};


fn main() {
    let input = include_str!("./input6.txt");
    let output = part1(input);
    println!("{}", output);
}


fn parse_params(input: &str) -> Vec<usize>{
    let mut params: Vec<usize> = Vec::new();
    let param_data = input.split(":").collect::<Vec<_>>()[1].split_whitespace().map(|x| x.parse::<usize>().expect("this should be a valid number")).collect::<Vec<_>>();
    for param in param_data{
        params.push(param);
    }
    params
}

fn calculate_ways_to_win(time: usize, dist: usize) -> usize {
    let mut ways_to_win = 0;
    for ms in 1..time/2+1{
        let travel_dist = ms*(time-ms);
        if travel_dist > dist {
            // println!("hold for {ms}ms");
            // println!("traveled {travel_dist} mm");
            // println!("won the race!");
            ways_to_win += 2;
        }
    }
    if time % 2 == 0 {
        ways_to_win -= 1;
    }
    ways_to_win
}

fn part1(input: &str) -> usize {
    let mut total_sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    let time_params = parse_params(lines[0]);
    let dist_params = parse_params(lines[1]);

    let mut output = 1;
    for race_index in 0..time_params.len(){
        // println!("Race {}: {}ms {}mm", race_index+1, time_params[race_index], dist_params[race_index]);
        let ways_to_win = calculate_ways_to_win(time_params[race_index], dist_params[race_index]);
        // println!("Ways to win: {ways_to_win}\n");
        output *= ways_to_win;
    } 
    
    output
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input6.txt");
        let result = part1(input);
        assert_eq!(result, 5133600);
    }
}