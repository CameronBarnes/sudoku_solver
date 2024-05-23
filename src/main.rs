mod board;

use board::{Board, Cell};

fn fixed(board: &mut Board) {
    let str = r"060000040
090170000
005000001
000030500
000059600
000000824
802500000
000002000
400008006";
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
        // TODO: Handle X-wing
        // TODO: Handle Y-wing
        // TODO: Handle Swordfish
        // TODO: Handle guessing

        if handle_pointing(&mut board) {
            updated = true;
        }

        if handle_blocking_row(&mut board) {
            updated = true;
        }

        if handle_blocking_col(&mut board) {
            updated = true;
        }

        if updated {
            println!("updated");
        } else {
            dbg!(&board);
            println!("Num unsolved: {}", board.num_unsolved());
            println!("Num possible values: {}", board.num_possible_values());
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

// If the only possible positions for a value in a row are in the same group, remove that
// possible number from all cells in the group outside the row
// This strategy is the row equivalent of pointing pairs and tripples
fn handle_blocking_row(board: &mut Board) -> bool {
    let mut updated = false;

    for row in 0..9 {
        // List all known values in the row
        let present: Vec<u8> = board
            .row(row)
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
            // List of all cells in the row that contain the missing value as a possible value
            let found: Vec<(usize, usize)> = board
                .enum_row(row)
                .iter()
                .filter_map(|(pos, cell)| match cell {
                    Cell::Known(_) => None,
                    Cell::Possible(possible) => {
                        if possible.contains(&missing) {
                            Some(*pos)
                        } else {
                            None
                        }
                    }
                })
                .collect();

            let group_row = found[0].0 / 3;
            let group_col = found[0].1 / 3;

            // If all the cells we found are in the same group, then we can remove that possible
            // value from all cells in the group that are not in that row
            if found
                .iter()
                .all(|(row, col)| row / 3 == group_row && col / 3 == group_col)
            {
                board.enum_group_mut(group_row, group_col).iter_mut().for_each(|(pos, cell)| {
                    if pos.0 == row {
                        return;
                    }
                    match cell {
                        Cell::Known(_) => {},
                        Cell::Possible(possible) => {
                            let len = possible.len();
                            possible.retain(|val| *val != missing);
                            if len != possible.len() {
                                updated = true;
                                cell.check();
                            }
                        },
                    }
                })
            }
        }
    }

    updated
}

// If the only possible positions for a value in a col are in the same group, remove that
// possible number from all cells in the group outside the col
// This strategy is the col equivalent of pointing pairs and tripples
fn handle_blocking_col(board: &mut Board) -> bool {
    let mut updated = false;

    for col in 0..9 {
        // List all known values in the col
        let present: Vec<u8> = board
            .col(col)
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
            // List of all cells in the col that contain the missing value as a possible value
            let found: Vec<(usize, usize)> = board
                .enum_col(col)
                .iter()
                .filter_map(|(pos, cell)| match cell {
                    Cell::Known(_) => None,
                    Cell::Possible(possible) => {
                        if possible.contains(&missing) {
                            Some(*pos)
                        } else {
                            None
                        }
                    }
                })
                .collect();

            let group_row = found[0].0 / 3;
            let group_col = found[0].1 / 3;

            // If all the cells we found are in the same group, then we can remove that possible
            // value from all cells in the group that are not in that col
            if found
                .iter()
                .all(|(row, col)| row / 3 == group_row && col / 3 == group_col)
            {
                board.enum_group_mut(group_row, group_col).iter_mut().for_each(|(pos, cell)| {
                    if pos.1 == col {
                        return;
                    }
                    match cell {
                        Cell::Known(_) => {},
                        Cell::Possible(possible) => {
                            let len = possible.len();
                            possible.retain(|val| *val != missing);
                            if len != possible.len() {
                                updated = true;
                                cell.check();
                            }
                        },
                    }
                })
            }
        }
    }

    updated
}

/// If only a single row or col in a group contains cells with a possible number, remove that
/// possible number from all cells in that row or col outside the group
/// This strategy is called pointing pairs and tripples
fn handle_pointing(board: &mut Board) -> bool {
    let mut updated = false;
    for group_row in 0..3 {
        for group_col in 0..3 {
            // List of all known values in the group
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
                // List of all cells in the group that contain the missing value as a possible value
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

                let row = found[0].0;
                let col = found[0].1;

                // If all the cells found are in the same row or col, then they're a 'pointing'
                // pair or tripple and we'll remove them from all other cells in the row or col
                let row_only = found.iter().all(|pos| pos.0 == row);
                let col_only = found.iter().all(|pos| pos.1 == col);

                // Handle a pointing pair/tripple in a row
                if row_only {
                    for col in 0..9 {
                        if found.iter().any(|pos| pos.1 == col) {
                            continue;
                        }
                        let cell = board.get_mut(row, col);
                        match cell {
                            Cell::Known(_) => {}
                            Cell::Possible(values) => {
                                let len = values.len();
                                values.retain(|val| *val != missing);
                                if len != values.len() {
                                    updated = true;
                                    cell.check();
                                }
                            }
                        }
                    }
                }
                // Handle a pointing pair/tripple in a col
                if col_only {
                    for row in 0..9 {
                        if found.iter().any(|pos| pos.0 == row) {
                            continue;
                        }
                        let cell = board.get_mut(row, col);
                        match cell {
                            Cell::Known(_) => {}
                            Cell::Possible(values) => {
                                let len = values.len();
                                values.retain(|val| *val != missing);
                                if len != values.len() {
                                    updated = true;
                                    cell.check();
                                }
                            }
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
    // Get a list of all values currently known in the collection
    let mut present: Vec<u8> = cells
        .iter()
        .filter_map(|cell| match **cell {
            Cell::Known(val) => Some(val),
            Cell::Possible(_) => None,
        })
        .collect();
    // Remove the known present values from the list of possible values for all cells in this
    // collection
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

    // If only one cell in a collection has a value listed as possible, that cell must be that
    // value
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
