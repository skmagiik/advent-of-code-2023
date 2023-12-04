fn main() {
    let input = include_str!("./input4.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let mut total_sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    // println!("{}",input);

    for line in lines{
        // println!("{}", line);
        let col_loc = line.find(':').expect("each line should have a : char");
        // println!("col_loc: {}", col_loc);
        let mut card_num_start_index = 0;
        for (i, c) in line.chars().enumerate(){
            if c.is_digit(10) {
                card_num_start_index = i;
                break;
            }
        }
        // let card = &line[card_num_start_index..col_loc].parse::<usize>().expect("This should be a valid card number");
        // println!("Card {}", card);

        let pipe_loc = line.find('|').expect("each line should have a | char");
        let winning_str = &line[col_loc+1..pipe_loc].trim();
        let card_str = &line[pipe_loc+1..].trim();
        // println!("Card {} Winning: {}, Numbers: {}", card, winning_str, card_str);

        let winning_values = winning_str.split(' ')
            .filter(|x| (*x != ""))
            // .inspect(|x| println!("{x}"))
            .map(|x| x.parse::<usize>().expect("each winning value should be a number"))
            .collect::<Vec<usize>>();
        // println!("{:?}", winning_values);

        let card_values = card_str.split(' ')
            .filter(|x| (*x != ""))
            // .inspect(|x| println!("{x}"))
            .map(|x| x.parse::<usize>().expect("each winning value should be a number"))
            .collect::<Vec<usize>>();
        // println!("{:?}", card_values);
        
        let mut card_points = 0;
        for num in card_values {
            if winning_values.contains(&num) {
                // println!("Winning value: {}", num);
                if card_points == 0 {
                    card_points += 1;
                }else{
                    card_points *= 2;
                }
            }
        }
        // println!("Card score: {}", card_points);
        total_sum += card_points;
    }

    total_sum
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input4.txt");
        let result = part1(input);
        assert_eq!(result, 17782);
    }
}