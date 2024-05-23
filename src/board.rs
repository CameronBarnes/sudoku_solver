use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Cell {
    Known(u8),
    Possible(Vec<u8>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Possible((1..=9).collect())
    }
}

impl Cell {
    /// If this cell has only one possible `value`, set this cell to Known(`value`)
    pub fn check(&mut self) {
        match self {
            Cell::Known(_) => {}
            Cell::Possible(values) => {
                if values.len() == 1 {
                    *self = Self::Known(values[0]);
                }
            }
        }
    }

    /// Returns true if enum is Cell::Known(_), false if enum is Cell::Possible(_)
    pub fn is_known(&self) -> bool {
        match self {
            Cell::Known(_) => true,
            Cell::Possible(_) => false,
        }
    }

    /// Unpacks the `value` of a `Cell::Known(value)` as `Some(value)`. Returns
    /// `None` if the enum is `Cell::Possible(_)`
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Known(value) => Some(*value),
            Cell::Possible(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<Cell>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: vec![vec![Cell::default(); 9]; 9],
        }
    }
}

impl Board {
    fn rows(&self) -> Vec<Vec<&Cell>> {
        (0..9).map(|row| self.row(row)).collect()
    }

    fn cols(&self) -> Vec<Vec<&Cell>> {
        (0..9).map(|col| self.col(col)).collect()
    }

    fn groups(&self) -> Vec<Vec<&Cell>> {
        (0..3)
            .flat_map(move |row| (0..3).map(move |col| self.group(row, col)))
            .collect()
    }
}

impl Board {
    /// Returns a Vec<&Cell> referencing all the Cells in the requested col
    pub fn col(&self, index: usize) -> Vec<&Cell> {
        self.board
            .iter()
            .map(|row| row.get(index).unwrap())
            .collect()
    }

    /// Returns a Vec<&mut Cell> mutably referencing all the Cells in the requested col
    pub fn col_mut(&mut self, index: usize) -> Vec<&mut Cell> {
        self.board
            .iter_mut()
            .map(|row| row.get_mut(index).unwrap())
            .collect()
    }

    pub fn enum_col(&self, col_index: usize) -> Vec<((usize, usize), &Cell)> {
        self.board
            .iter()
            .enumerate()
            .map(|(row_index, row)| ((row_index, col_index), row.get(col_index).unwrap()))
            .collect()
    }

    /// Returns a Vec<&Cell> referencing all the Cells in the requested row
    pub fn row(&self, index: usize) -> Vec<&Cell> {
        self.board.get(index).unwrap().iter().collect()
    }

    /// Returns a Vec<&mut Cell> mutably referencing all the Cells in the requested row
    pub fn row_mut(&mut self, index: usize) -> Vec<&mut Cell> {
        self.board.get_mut(index).unwrap().iter_mut().collect()
    }

    pub fn enum_row(&self, row_index: usize) -> Vec<((usize, usize), &Cell)> {
        self.board
            .get(row_index)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(col_index, cell)| ((row_index, col_index), cell))
            .collect()
    }

    /// Returns a &Cell from the requested position
    pub fn get(&self, row: usize, col: usize) -> &Cell {
        &self.board[row][col]
    }

    /// Returns a &mut Cell from the requested position
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.board[row][col]
    }

    /// Returns a Vec<&Cell> referencing all the Cells in the requested group
    pub fn group(&self, row: usize, col: usize) -> Vec<&Cell> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(row_i, vals)| {
                if row_i >= row * 3 && row_i < row * 3 + 3 {
                    Some(vals)
                } else {
                    None
                }
            })
            .flat_map(|row| row.iter().enumerate())
            .filter_map(|(col_i, vals)| {
                if col_i >= col * 3 && col_i < col * 3 + 3 {
                    Some(vals)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns a Vec<&mut Cell> mutably referencing all the Cells in the requested group
    pub fn group_mut(&mut self, row: usize, col: usize) -> Vec<&mut Cell> {
        self.board
            .iter_mut()
            .enumerate()
            .filter_map(|(row_i, vals)| {
                if row_i >= row * 3 && row_i < row * 3 + 3 {
                    Some(vals)
                } else {
                    None
                }
            })
            .flat_map(|row| row.iter_mut().enumerate())
            .filter_map(|(col_i, vals)| {
                if col_i >= col * 3 && col_i < col * 3 + 3 {
                    Some(vals)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn enum_group(&self, row: usize, col: usize) -> Vec<((usize, usize), &Cell)> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(row_i, vals)| {
                if row_i >= row * 3 && row_i < row * 3 + 3 {
                    Some((row_i, vals))
                } else {
                    None
                }
            })
            .flat_map(|(row_i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_i, cell)| ((row_i, col_i), cell))
            })
            .filter_map(|((row_i, col_i), cell)| {
                if col_i >= col * 3 && col_i < col * 3 + 3 {
                    Some(((row_i, col_i), cell))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn enum_group_mut(&mut self, row: usize, col: usize) -> Vec<((usize, usize), &mut Cell)> {
        self.board
            .iter_mut()
            .enumerate()
            .filter_map(|(row_i, vals)| {
                if row_i >= row * 3 && row_i < row * 3 + 3 {
                    Some((row_i, vals))
                } else {
                    None
                }
            })
            .flat_map(|(row_i, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(col_i, cell)| ((row_i, col_i), cell))
            })
            .filter_map(|((row_i, col_i), cell)| {
                if col_i >= col * 3 && col_i < col * 3 + 3 {
                    Some(((row_i, col_i), cell))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the number of cells that are not Cell::Known
    pub fn num_unsolved(&self) -> usize {
        self.board.iter().flatten().filter(|cell| !cell.is_known()).count()
    }

    pub fn num_possible_values(&self) -> usize {
        self.board.iter().flatten().map(|cell| {
            match cell {
                Cell::Known(_) => 0,
                Cell::Possible(values) => values.len(),
            }
        }).sum()
    }

    /// Returns true if all rows, cols, and groups contain the values 1..=9
    pub fn is_correct(&self) -> bool {
        self.rows().iter().all(|row| {
            let mut check = HashSet::new();
            row.iter()
                .filter_map(|cell| cell.value())
                .all(|val| val <= 9 && check.insert(val))
        }) && self.cols().iter().all(|col| {
            let mut check = HashSet::new();
            col.iter()
                .filter_map(|cell| cell.value())
                .all(|val| val <= 9 && check.insert(val))
        }) && self.groups().iter().all(|group| {
            let mut check = HashSet::new();
            group
                .iter()
                .filter_map(|cell| cell.value())
                .all(|val| val <= 9 && check.insert(val))
        })
    }
}
