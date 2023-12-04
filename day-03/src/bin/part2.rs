
#[derive(Clone, Debug)]
struct Coordinates{
    x: usize,
    y: usize
}

#[derive(Clone, Debug)]
struct Number {
    str_contents: String,
    length: usize,
    coordinates: Coordinates,
    symbols: Vec<Symbol>
}

#[derive(Clone, Debug)]
struct Symbol{
    char: char,
    coordinates: Coordinates,
}

#[derive(Clone, Debug)]
struct Gear{
    coordinates: Coordinates,
    numbers: Vec<usize>
}

fn main() {
    let input = include_str!("./input3.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> usize {
    let mut total_sum: usize = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    println!("{}",input);
    println!("");
    
    let mut nums :Vec<Number> = Vec::new();
    let mut newnum = Number{str_contents: "".to_owned(), length: 0, coordinates: Coordinates { x: 0, y: 0 }, symbols: Vec::new()};
    let mut appending_num = false;

    let line_len = lines[0].len();

    let mut possible_gears :Vec<Gear> = Vec::new();

    for (y, line) in lines.iter().enumerate(){
        for (x, c) in line.chars().enumerate(){            
            println!("{}, {} => {}", x, y, c);
            if  !c.is_digit(10) && c != '*' && !appending_num {
                continue;
            }
            if c == '*' {
                let mut gear_found = false;
                for gear in &possible_gears {
                    if gear.coordinates.x == x && gear.coordinates.y == y {
                        println!("Duplicate gear found @ {:?}", gear.coordinates);
                        gear_found = true;
                        break;
                    }
                }
                if !gear_found {
                    println!("Gear found for the first time");
                    possible_gears.push(Gear{coordinates: Coordinates { x: x, y: y }, numbers: Vec::new()});
                }
            }
            if c.is_digit(10) {
                if !appending_num {
                    // println!("Found start of number {}", c);
                    newnum = Number { str_contents: c.to_string(), length: 1, coordinates: Coordinates { x: x, y: y }, symbols: Vec::new()};
                    appending_num = true;
                }else{
                    newnum.str_contents += &c.to_string();
                    newnum.length += 1;
                }
            }

            if appending_num && (!c.is_digit(10) || x == line_len - 1){
                let min_x = if newnum.coordinates.x == 0 { 0 } else {newnum.coordinates.x - 1};
                let max_x = if newnum.coordinates.x + newnum.length < line_len { newnum.coordinates.x + newnum.length } else { line_len - 1 };

                let min_y = if newnum.coordinates.y > 0 { y - 1 } else { y };
                let max_y = if newnum.coordinates.y < line_len - 1 { y + 1} else { y };
            
                for check_y in min_y..=max_y {
                    for (check_x, check_char) in lines[check_y][min_x..=max_x].chars().enumerate() {
                        if !check_char.is_digit(10) && check_char == '*' {
                            println!("\nFound {} while searching around {}", check_char, newnum.str_contents);
                            let mut sym: Symbol = Symbol { char: check_char, coordinates: Coordinates { x: 0, y: check_y } };
                            if newnum.coordinates.x != 0 {
                                sym.coordinates.x = newnum.coordinates.x + check_x -1;
                            }else{
                                sym.coordinates.x = newnum.coordinates.x + check_x;
                            }
                            println!("{:?}", sym);
                            newnum.symbols.push(sym.clone());
                            if check_char == '*' {
                                let mut gear_found = false;
                                for gear in &mut possible_gears {
                                    if gear.coordinates.x == sym.coordinates.x && gear.coordinates.y == sym.coordinates.y {
                                        println!("Number touching existing gear @ {:?}", gear.coordinates);
                                        println!("{:?}", gear);
                                        gear.numbers.push(newnum.str_contents.parse::<usize>().expect("this should be a valid number"));
                                        println!("{:?}", gear);
                                        gear_found = true;
                                        break;
                                    }
                                }
                                if !gear_found {
                                    println!("Adding possible gear");
                                    possible_gears.push(Gear{coordinates: Coordinates { x: sym.coordinates.x, y: sym.coordinates.y }, numbers: vec![newnum.str_contents.parse::<usize>().expect("this should be a valid number")]});
                                }
                            }
                        }
                        // print!("{}", check_char);
                    }
                    // println!("");
                }
                // if newnum.symbols.len() == 0 {
                //     for check_y in min_y..=max_y {
                //         print!("{}: ", check_y);
                //         for check_char in lines.clone().nth(check_y).expect("Should be within range")[min_x..=max_x].chars(){
                //             print!("{}",check_char);
                //         }
                //         println!("");
                //     }
                //     println!("Number completed: {}, Length: {}, Coordinates: {:?}, Symbols: {}", newnum.str_contents, newnum.length, newnum.coordinates, newnum.symbols.len());
                // }


                appending_num = false;
                nums.push(newnum.clone());
            }
        }
    }

    println!("\n\nPossible Gears:");
    for gear in &possible_gears {
        println!("{:?}", gear);
    }
    
    println!("\n\nValid Gears:");
    for gear in &possible_gears {
        if gear.numbers.len() == 2{
            println!("{:?}", gear);
            let gear_ratio = gear.numbers[0] * gear.numbers[1];
            total_sum += gear_ratio;
        }
    }



    total_sum
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let input = "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part1_full_input(){
        let input = include_str!("./input3.txt");
        let result = part1(input);
        assert_eq!(result, 81166799);
    }
}