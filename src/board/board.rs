use std::{collections::HashMap, iter::Skip};



pub struct Board {
    board_array: [[Tile; 9]; 9], // defines the layout of all the tiles 
    pub solved: bool, // board state 
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
                board.board_array[row][col].position = [row as i8, col as i8];
            }
        }

        return board
    }

    pub fn set_input_tile(&mut self, set_row:i8, set_col:i8, val:i8) {
        for row in 0..9 {
            for col in 0..9 {
                if (row as i8) == set_row && (col as i8) == set_col {
                    self.board_array[row][col].val = val;
                    self.board_array[row][col].locked = true;
                }
            }
        }
    }

    // functions associated with board 
    // fn find_possible_values(&self, tile: &mut Tile) {
    //     println!("{0}", tile.position[0]);
    // }

    pub fn check_board(&mut self) -> bool{
        //each of the mini functions return a size 2 array 
        // [is board broken, is there a 0 in board]
        fn check_row(board: &Board, row_number: i8) -> [bool; 2] {
            let mut previous: HashMap<i8, i8>= HashMap::new();

            for i in 0..9 {
                let current_square:Tile = board.board_array[row_number as usize][i];
                if previous.contains_key(&current_square.val) && current_square.val != 0{
                    return [false, false]
                }
                previous.insert(current_square.val as i8, i as i8);

            }

            if previous.contains_key(&0) {
                return [true, false]
            }
            return [true, true]

        }

        fn check_col(board: &Board, col_number: i8) -> [bool; 2] {
            let mut previous: HashMap<i8, i8>= HashMap::new();

            for i in 0..9 {
                let current_square:Tile = board.board_array[i][col_number as usize];
                if previous.contains_key(&current_square.val) && current_square.val != 0{
                    return [false, false]
                }
                
                previous.insert(current_square.val as i8, i as i8);

            }

            if previous.contains_key(&0) {
                return [true, false]
            }

            return [true, true]
        }

        fn check_box(board: &Board, start_row: i8, start_col:i8) -> [bool; 2] {
            let mut previous: HashMap<i8, i8>= HashMap::new();
            for row in start_row..(start_row+3) {
                for col in start_col..(start_col+3) {
                    let current_tile:Tile = board.board_array[row as usize][col as usize];
                    if previous.contains_key(&current_tile.val) && current_tile.val != 0{
                        return [false, false]
                    }
                    previous.insert(current_tile.val as i8, row as i8);

                }

                if previous.contains_key(&0) {
                    return [true, false]
                }
                
                return [true, true]
            }
            



            return [true, true]
        }

        let mut complete_check = true;

        for i in 0..9 {
            let check_current_row = check_row(&self, i);
            let check_current_col = check_col(&self, i);
            if check_current_row[0] == false {
                return false
            }
            if check_current_col[0] == false {
                return false
            }

            if !check_current_col[1] || !check_current_row[1] {
                complete_check = false;
            }
        }

        let iterate_nums = [0, 3, 6];

        for row in iterate_nums {
            for col in iterate_nums {
                let current_check = check_box(&self, row, col);
                if current_check[0] == false  {
                    return false
                } else if current_check[1] == false {
                    complete_check = false
                }
            }
        }

        if complete_check {
            self.solved = true
        }
        
        //returns true if the whole thing is solved 
        //however i need to write it so that it can check whether it is not broken even if it isnt solved
            
        return true

    }

    // this is where the magic is gonna happen
    pub fn solve(&mut self, row: i8, col: i8) {
        let mut next_row: i8 = row;
        let mut next_col: i8 = col;
        if col == 8 {
            next_row += 1;
            next_col = 0;
        } else {
            next_col += 1
        }
        println!("{0}", row);

        if self.board_array[row as usize][col as usize].locked {
            self.solve(next_row, next_col);
        }

        for i in 1..10 {
            self.board_array[row as usize][col as usize].val = i;
            if self.check_board() {
                self.print_board();
                // solve with next
                self.solve(next_row, next_col);
            }
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