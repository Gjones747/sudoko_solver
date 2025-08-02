use std::collections::HashMap;

use crate::board;


pub struct Board {
    board_array: [[Tile; 9]; 9], // defines the layout of all the tiles 
    solved: bool, // board state 
}

impl Board {
    pub fn print_board(&self) {
        fn print_row(row: [Tile; 9]) {
            print!("|");
            for i in 0..9 {
                if i == 3 || i == 6 {
                    print!("|")
                }
                if row[i].val == 0 {
                    print!(" _ ")
                } else {
                    print!(" {0} ", row[i].val)
                }
            }
            print!("|")
        }

        println!("");
        println!("-------------------------------");

        for i in 0..9 {
            if i == 3 || i == 6 {
                println!("-------------------------------")
            }
            print_row(self.board_array[i]);
            println!("");

        }
        println!("-------------------------------")

    }

    pub fn make_board() -> Board {

        let uninit_tile = Tile {
            locked: false, 
            position: [-1, -1],
            val: 0,
            //possible_values: [0; 9]
        };

        let mut board = Board {
            board_array: [[uninit_tile; 9]; 9],
            solved: false,
        };

        for row in 0..9 {
            for col in 0..9 {
                board.board_array[row][col].position = [row as i8, col as i8]
            }
        }

        return board
    }

    pub fn set_input_tile(&mut self, set_row:i8, set_col:i8, val:i8) {
        for row in 0..9 {
            for col in 0..9 {
                if (row as i8) == set_row && (col as i8) == set_col {
                    self.board_array[row][col].val = val;
                    self.board_array[row][col].locked = true
                    
                }
            }
        }
    }

    // functions associated with board 
    // fn find_possible_values(&self, tile: &mut Tile) {
    //     println!("{0}", tile.position[0]);
    // }

    fn check_board(&self) {
        fn check_row(board: Board, row_number: i8) -> bool {
            let mut previous: HashMap<i8, i8>= HashMap::new();

            for i in 0..9 {
                let current_square:Tile = board.board_array[row_number as usize][i];
                if previous.contains_key(&current_square.val){
                    return false
                }
                previous.insert(current_square.val as i8, i as i8);

            }

            return true

        }

        fn check_col(board: Board, col_number: i8) -> bool {
            let mut previous: HashMap<i8, i8>= HashMap::new();

            for i in 0..9 {
                let current_square:Tile = board.board_array[i][col_number as usize];
                if previous.contains_key(&current_square.val){
                    return false
                }
                previous.insert(current_square.val as i8, i as i8);

            }

            return true
        }

        fn check_box(board: Board, row: i8, col:i8) -> bool {

            fn individual_box(board:Board, start_row:i8, start_col:i8) -> bool {
                let mut previous: HashMap<i8, i8>= HashMap::new();
                for row in start_row..(start_row+3) {
                    for col in start_col..(start_col+3) {
                        let current_tile:Tile = board.board_array[row as usize][col as usize];
                        if previous.contains_key(&current_tile.val){
                            return false
                        }
                        previous.insert(current_tile.val as i8, row as i8);

                    }
                }

                return true
            }

            if row/3 >= 2 &&  col/3 >= 2 {
                return individual_box(board, 6, 6)
            } else if row/3 >= 2 && col/3 >= 1 {
                return individual_box(board, 6, 3);
            }

            return true
        }


    }
}

#[derive(Clone, Copy)]
struct Tile {
    val: i8, // 0 if null
    locked: bool,
    // possible_values: [i8; 9],
    position: [i8; 2] // each tile will store its own position in the board ROW COL
}

impl Tile {

}