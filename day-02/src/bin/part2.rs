
fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let mut total_sum = 0;
    let lines = input.lines();
    for line in lines{
        // println!("\n{}", line);
        let game_info = line.split(": ").collect::<Vec<_>>();

        let game_id = game_info[0].replace("Game ", "").parse::<i32>().expect("This should be a valid game ID");
        println!("{}", game_id);

        let game_data = game_info[1].split("; ").collect::<Vec<_>>();

        
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for draw in game_data{
            // println!("{}", draw);
            let cubes = draw.split(", ").collect::<Vec<_>>();
            for cube in cubes{
                let cube_info = cube.split(" ").collect::<Vec<_>>();
                // println!("{} {}", cube_info[0], cube_info[1]);

                let qty = cube_info[0].parse::<i32>().expect("valid cube amount expected");
                match cube_info[1]{
                    "red" => if qty > min_red { min_red = qty; },
                    "green" => if qty > min_green { min_green = qty; },
                    "blue" => if qty > min_blue { min_blue = qty; },
                    _ => ()
                };
            }
            // println!("Min RGB: {} {} {}", min_red, min_green, min_blue);
        }
        total_sum += min_red * min_green * min_blue;
    }
    total_sum
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_part2_full_input(){
        let input = include_str!("./input2.txt");
        let result = part1(input);
        assert_eq!(result, 72596);
    }
}