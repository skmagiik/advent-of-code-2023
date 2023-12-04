
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let options = [ "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];
    let mut total_sum = 0;
    let lines = input.lines();
    for line in lines{
        // println!("{}", line);
        let mut leftmost: i32 = -1;
        let mut rightmost: i32 = -1;
        for i in 0..line.len(){
            'opt_for: for (opt_index,opt) in options.iter().enumerate(){
                if opt.len() > line.len() - i{
                    continue 'opt_for;
                }
                if *opt == &line[i..i+opt.len()]{
                    // println!("found {} at {}", opt, i);
                    if leftmost == -1{
                        leftmost = opt_index as i32 / 2 + 1;
                        rightmost = opt_index as i32 / 2 + 1;
                    }
                    else{
                        rightmost = opt_index as i32 / 2 + 1;
                    }
                }
            }
        }
        // println!("{}", leftmost*10 + rightmost);
        total_sum += leftmost*10 + rightmost;
    }
    total_sum
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part1(input);
        assert_eq!(result, 281);
    }
    
    #[test]
    fn test_part2_full_input(){
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, 55902);
    }
}