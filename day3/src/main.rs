use std::fs;

fn main() {
    //Read in the file and generate a navigable map
    //let data = fs::read_to_string("test").unwrap();
    let data = fs::read_to_string("input").unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();

    let data_rows: Vec<&str> = data.split("\n").collect();

    for i in 0..data_rows.len() {
        let mut row = String::from(data_rows[i]);

        if row.len() == 0 {
            break;
        }
        let mut map_row: Vec<char> = Vec::new();
        while row.len() > 0 {
            let val = row.pop().unwrap();
            map_row.push(val);
        }

        map_row.reverse();

        map.push(map_row);
    }

    //part1(map.clone());
    part2(map.clone());
}

fn extract_number(row_index: usize, row: Vec<char>) -> i64 {
    let mut i = row_index;
    let mut current_number: i64 = 0;
    //rewind back to the first number
    while row[i].is_numeric() && i > 0 {
        i -= 1;
    }
    if !row[i].is_numeric() {
        //we overshoot, so dial it back a spot
        i += 1;
    }

    while i <= row.len() - 1 && row[i].is_numeric() {
        //we have a digit so first we add it to the current number
        current_number *= 10;
        current_number += row[i].to_string().parse::<i64>().unwrap();
        i += 1;
    }

    println!("Found Number: {current_number}");

    current_number
}

fn part2(map: Vec<Vec<char>>) -> i64 {
    let mut current_number: i64 = 0;
    for i in 0..map.len() {
        let row = map[i].clone();

        println!("{:?}", row);

        for j in 0..row.len() {
            if row[j].clone() == '*' {
                let mut touchers: i64 = 0;
                let mut ratio: i64 = 1;
                if i > 0 {
                    //check the row above
                    let above = map[i - 1].clone();
                    if j > 0 {
                        if above[j - 1].is_numeric() {
                            touchers += 1;
                            let extracted = extract_number(j - 1, above.clone());
                            ratio *= extracted;
                            if above[j] == '.' && above[j + 1].is_numeric() {
                                touchers += 1;
                                let extracted = extract_number(j + 1, above.clone());
                                ratio *= extracted;
                            }
                        }
                        if !above[j - 1].is_numeric() && above[j].is_numeric() {
                            touchers += 1;
                            let extracted = extract_number(j, above.clone());
                            ratio *= extracted;
                        }
                        if !above[j - 1].is_numeric()
                            && !above[j].is_numeric()
                            && above[j + 1].is_numeric()
                        {
                            touchers += 1;
                            let extracted = extract_number(j + 1, above.clone());
                            ratio *= extracted;
                        }
                    }
                }
                if i < map.len() - 1 {
                    //check the row below
                    let above = map[i + 1].clone();
                    if j > 0 {
                        if above[j - 1].is_numeric() {
                            touchers += 1;
                            let extracted = extract_number(j - 1, above.clone());
                            ratio *= extracted;
                            if above[j] == '.' && above[j + 1].is_numeric() {
                                touchers += 1;
                                let extracted = extract_number(j + 1, above.clone());
                                ratio *= extracted;
                            }
                        }
                        if !above[j - 1].is_numeric() && above[j].is_numeric() {
                            touchers += 1;
                            let extracted = extract_number(j, above.clone());
                            ratio *= extracted;
                        }
                        if !above[j - 1].is_numeric()
                            && !above[j].is_numeric()
                            && above[j + 1].is_numeric()
                        {
                            touchers += 1;
                            let extracted = extract_number(j + 1, above.clone());
                            ratio *= extracted;
                        }
                    }
                }

                if j > 0 {
                    if row[j - 1].is_numeric() {
                        touchers += 1;
                        let extracted = extract_number(j - 1, row.clone());
                        ratio *= extracted;
                    }
                }
                if j < row.len() - 1 {
                    if row[j + 1].is_numeric() {
                        touchers += 1;
                        let extracted = extract_number(j + 1, row.clone());
                        ratio *= extracted;
                    }
                }

                if touchers == 2 {
                    println!("Gear Ratio: {ratio}");
                    current_number += ratio;
                }
            }
        }
    }

    println!("Gear Ratio: {current_number}");
    current_number
}

fn part1(map: Vec<Vec<char>>) -> () {
    let mut total: i64 = 0;

    //Step through the map and find the numbers - because I suspect part 2 will need this anyway
    for i in 0..map.len() {
        let row = map[i].clone();

        println!("{:?}", row);
        let mut current_number: i64 = 0;
        let mut is_part: bool = false;
        for j in 0..row.len() {
            if row[j].is_numeric() {
                //we have a digit so first we add it to the current number
                current_number *= 10;
                current_number += row[j].to_string().parse::<i64>().unwrap();
                //If we haven't already established it as a part number
                if !is_part {
                    //Then we check for any nearby part...
                    if i > 0 {
                        //check the row above
                        let above = map[i - 1].clone();
                        if j > 0 {
                            if !above[j - 1].is_numeric() && above[j - 1] != '.' {
                                is_part = true;
                            }
                        }
                        if !above[j].is_numeric() && above[j] != '.' {
                            is_part = true;
                        }
                        if j < row.len() - 1 {
                            if !above[j + 1].is_numeric() && above[j + 1] != '.' {
                                is_part = true;
                            }
                        }
                    }

                    if j > 0 {
                        //check the value before
                        if !row[j - 1].is_numeric() && row[j - 1] != '.' {
                            is_part = true;
                        }
                    }

                    if i < map.len() - 1 {
                        //check the next row
                        //
                        let above = map[i + 1].clone();
                        if j > 0 {
                            if !above[j - 1].is_numeric() && above[j - 1] != '.' {
                                is_part = true;
                            }
                        }
                        if !above[j].is_numeric() && above[j] != '.' {
                            is_part = true;
                        }
                        if j < row.len() - 1 {
                            if !above[j + 1].is_numeric() && above[j + 1] != '.' {
                                is_part = true;
                            }
                        }
                    }

                    if j < row.len() - 1 {
                        //check the next position
                        if !row[j + 1].is_numeric() && row[j + 1] != '.' {
                            is_part = true;
                        }
                    }
                }
            } else {
                if current_number > 0 {
                    println!("{current_number}, {is_part}");
                    if is_part {
                        total += current_number;
                        is_part = false;
                    }
                    current_number = 0;
                }
            }
        }
        if current_number > 0 {
            println!("{current_number}, {is_part}");
            if is_part {
                total += current_number;
            }
        }
    }

    println!("Total is: {total}");
}
