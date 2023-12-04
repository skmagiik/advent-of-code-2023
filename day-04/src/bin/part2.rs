fn main() {
    let input = include_str!("./input4.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    // println!("{} cards/lines",lines.len());

    let mut card_inventory: Vec<usize> = Vec::new();
    for _i in 0..lines.len() {
        card_inventory.push(1);
    }
    // println!("{:?}", card_inventory);

    for (card_index, line) in lines.iter().enumerate(){
        // println!("{}", line);
        let col_loc = line.find(':').expect("each line should have a : char");
        // println!("col_loc: {}", col_loc);
        // let mut card_num_start_index = 0;
        // for (i, c) in line.chars().enumerate(){
        //     if c.is_digit(10) {
        //         card_num_start_index = i;
        //         break;
        //     }
        // }
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
        for num in &card_values {
            if winning_values.contains(&num) {
                // println!("Winning value: {}", num);
                card_points += 1;
            }
        }
        // println!("Card score: {}", card_points);
        if card_points > 0{
            for new_card_index in card_index+1..card_index+card_points+1 {
                // println!("Won new card {}", new_card_index+1);
                card_inventory[new_card_index] += card_inventory[card_index];
            }

        }
    }

    // println!("{:?}", card_inventory);
    card_inventory.iter().sum()
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
        assert_eq!(result, 30);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input4.txt");
        let result = part1(input);
        assert_eq!(result, 8477787);
    }
}