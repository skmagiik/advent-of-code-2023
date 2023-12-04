
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let options = [ "1", "2", "3", "4", "5", "6", "7", "8", "9"];
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
                        leftmost = opt_index as i32 + 1;
                        rightmost = opt_index as i32 + 1;
                    }
                    else{
                        rightmost = opt_index as i32 + 1;
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
    use std::result;

    use super::*;

    #[test]
    fn test_part1(){
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, 56465);
    }
}