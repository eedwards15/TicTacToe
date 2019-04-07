use std::io;
mod board; 

fn main() {
    let mut board:[[u8; 3]; 3] = [[0; 3] ; 3];
    let mut Player: u8 = 1; 


    for x in 0..3 {
        for y in 0..3 {
            board[y][x] =  (0) as u8;
        }
    }   

    loop 
    {

        println!("Please Enter row");
        let mut row = String::new(); 
        io::stdin().read_line(&mut row)
            .expect("Failed to read line"); 

        let row: usize = match row.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Please Enter column"); 
        let mut col = String::new(); 
        io::stdin().read_line(&mut col)
            .expect("Failed to read line"); 

            
        let col: usize = match col.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        board::board_helper::set_values(Player,col,row,&mut board);
        board::board_helper::display_board(&mut board);

        Player = set_player(Player);    

        if(board::board_helper::did_win(board) || board::board_helper::check_board(board)){
            break; 
        }     
    }

    fn set_player(player: u8) -> u8
    {
        if player ==  1
        {
            return 2;
        }
        else
        {
            return 1; 
        }
    }



}