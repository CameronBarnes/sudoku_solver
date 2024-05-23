mod board;

use board::{Board, Cell};

fn fixed(board: &mut Board) {
    let str = r"351600084
009801000
080040000
090010000
000000020
700020501
000050002
073004090
120000705";
    str.lines()
        .enumerate()
        .for_each(|(row_index, line)| parse_line(line, row_index, board));
}

fn parse_line(input: &str, row_index: usize, board: &mut Board) {
    input
        .trim()
        .chars()
        .map(|char| {
            char.to_digit(10)
                .unwrap_or_else(|| panic!("failed to parse: {char}"))
        })
        .enumerate()
        .for_each(|(col_index, number)| {
            if number != 0 {
                *board.get_mut(row_index, col_index) = Cell::Known(number as u8);
            }
        })
}

fn player_entered(board: &mut Board) {
    for row in 0..9 {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        parse_line(str.trim(), row, board);
    }
}

fn main() {
    let mut board = Board::default();

    fixed(&mut board);
    //player_entered(&mut board);

    while board.num_unsolved() > 0 {
        let mut updated = false;
        for row in 0..9 {
            if handle_collection(board.row_mut(row)) {
                updated = true;
            }
        }

        for col in 0..9 {
            if handle_collection(board.col_mut(col)) {
                updated = true;
            }
        }

        for group_y in 0..3 {
            for group_x in 0..3 {
                if handle_collection(board.group_mut(group_y, group_x)) {
                    updated = true;
                }
            }
        }

        // TODO: Handle hidden and obvious pairs
        // TODO: Handle hidden and obvious tripples

        if handle_pointing(&mut board) {
            updated = true;
        }

        if updated {
            println!("updated");
        } else {
            dbg!(&board);
            println!("Num unsolved: {}", board.num_unsolved());
            return;
        }
    }

    println!("Done!");
    if board.is_correct() {
        println!("Solution is correct!");
    } else {
        println!("Solution is invalid!");
    }
}

fn handle_pointing(board: &mut Board) -> bool {
    let mut updated = false;
    for group_row in 0..3 {
        for group_col in 0..3 {
            let present: Vec<u8> = board
                .group(group_row, group_col)
                .iter()
                .filter_map(|cell| match **cell {
                    Cell::Known(val) => Some(val),
                    Cell::Possible(_) => None,
                })
                .collect();

            for missing in 1..=9 {
                if present.contains(&missing) {
                    continue;
                }
                let found: Vec<(usize, usize)> = board
                    .enum_group(group_row, group_col)
                    .iter()
                    .filter_map(|(pos, cell)| match cell {
                        Cell::Known(_) => None,
                        Cell::Possible(values) => {
                            if values.contains(&missing) {
                                Some(*pos)
                            } else {
                                None
                            }
                        }
                    })
                    .collect();

                if found.is_empty() {
                    continue;
                }

                let row = found[0].0;
                let col = found[0].1;

                let row_only = found.iter().all(|pos| pos.0 == row);
                let col_only = found.iter().all(|pos| pos.1 == col);

                if row_only {
                    for col in 0..9 {
                        if found.iter().any(|pos| pos.1 == col) {
                            continue;
                        }
                        match board.get_mut(row, col) {
                            Cell::Known(_) => {},
                            Cell::Possible(values) => {
                                let len = values.len();
                                values.retain(|val| *val != missing);
                                if len != values.len() {
                                    updated = true;
                                }
                            },
                        }
                    }
                }
                if col_only {
                    for row in 0..9 {
                        if found.iter().any(|pos| pos.0 == row) {
                            continue;
                        }
                        match board.get_mut(row, col) {
                            Cell::Known(_) => {},
                            Cell::Possible(values) => {
                                let len = values.len();
                                values.retain(|val| *val != missing);
                                if len != values.len() {
                                    updated = true;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    updated
}

fn handle_collection(mut cells: Vec<&mut Cell>) -> bool {
    let mut updated = false;
    let mut present: Vec<u8> = cells
        .iter()
        .filter_map(|cell| match **cell {
            Cell::Known(val) => Some(val),
            Cell::Possible(_) => None,
        })
        .collect();
    for cell in &mut cells {
        let known = cell.is_known();
        match cell {
            Cell::Known(_) => {}
            Cell::Possible(possible) => {
                let len = possible.len();
                possible.retain(|num| !present.contains(num));
                if possible.len() != len {
                    updated = true;
                }
            }
        }
        cell.check();
        if known != cell.is_known() {
            present.push(cell.value().unwrap());
        }
    }

    for missing in 1..=9 {
        if present.contains(&missing) {
            continue;
        }
        let mut possible_match = 0;
        for cell in &cells {
            match cell {
                Cell::Known(_) => {}
                Cell::Possible(possible) => {
                    if possible.contains(&missing) {
                        possible_match += 1;
                    }
                }
            }
        }
        if possible_match == 1 {
            for cell in &mut cells {
                match cell {
                    Cell::Known(_) => {}
                    Cell::Possible(possible) => {
                        if possible.contains(&missing) {
                            **cell = Cell::Known(missing);
                        }
                    }
                }
            }
            present.push(missing);
        }
    }

    updated
}
