mod board;

use board::{Board, Cell};

fn fixed(board: &mut Board) {
    *board.get_mut(0, 2) = Cell::Known(4);
    *board.get_mut(0, 3) = Cell::Known(8);
    *board.get_mut(0, 5) = Cell::Known(5);
    *board.get_mut(0, 6) = Cell::Known(7);
    *board.get_mut(0, 8) = Cell::Known(6);

    *board.get_mut(1, 3) = Cell::Known(9);
    *board.get_mut(1, 7) = Cell::Known(4);

    *board.get_mut(2, 1) = Cell::Known(2);
    *board.get_mut(2, 3) = Cell::Known(4);
    *board.get_mut(2, 8) = Cell::Known(1);

    *board.get_mut(3, 0) = Cell::Known(8);
    *board.get_mut(3, 3) = Cell::Known(7);
    *board.get_mut(3, 5) = Cell::Known(3);
    *board.get_mut(3, 7) = Cell::Known(6);

    *board.get_mut(4, 2) = Cell::Known(6);
    *board.get_mut(4, 5) = Cell::Known(1);
    *board.get_mut(4, 6) = Cell::Known(5);

    *board.get_mut(5, 0) = Cell::Known(2);
    *board.get_mut(5, 5) = Cell::Known(8);
    *board.get_mut(5, 6) = Cell::Known(4);
    *board.get_mut(5, 7) = Cell::Known(7);

    *board.get_mut(6, 1) = Cell::Known(4);
    *board.get_mut(6, 3) = Cell::Known(5);
    *board.get_mut(6, 5) = Cell::Known(6);

    *board.get_mut(7, 0) = Cell::Known(5);
    *board.get_mut(7, 6) = Cell::Known(6);
    *board.get_mut(7, 7) = Cell::Known(8);

    *board.get_mut(8, 3) = Cell::Known(1);
    *board.get_mut(8, 7) = Cell::Known(5);
    *board.get_mut(8, 8) = Cell::Known(4);
}

fn player_entered(board: &mut Board) {
    for row in 0..9 {
        for col in 0..9 {
            let mut str = String::new();
            std::io::stdin().read_line(&mut str).unwrap();
            if str.trim().eq("0") {
                continue;
            } else {
                *board.get_mut(row, col) = Cell::Known(str.trim().parse().unwrap());
            }
        }
    }
}

fn main() {
    let mut board = Board::default();

    //fixed(&mut board);
    player_entered(&mut board);

    while !board.is_solved() {
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

        if updated {
            println!("updated");
        } else {
            dbg!(&board);
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
