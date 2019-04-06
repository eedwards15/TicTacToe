use std::io;

fn main() {
    let mut board:[[u8; 3]; 3] = [[0; 3] ; 3];

    for x in 0..3 {
        for y in 0..3 {
            board[y][x] =  (0) as u8;
        }
    }   

    loop 
    {
        if(check_board(board)){
            break; 
        }

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

        println!("Please Enter Player"); 
        let mut player = String::new(); 
        io::stdin().read_line(&mut player)
            .expect("Failed to read line"); 

        let player: u8 = match player.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        set_values(player,col,row,&mut board);
        display_board(&mut board);

    }

}

fn set_values(value: u8 ,col:usize, row:usize, board: &mut [[u8; 3];3]) -> bool{
    if board[col][row] == 0 
    {
        board[col][row] = value;
        return true; 
    }
    return false; 
}


fn display_board(board: &mut [[u8; 3];3]){
    for x in 0..3 {
      for y in 0..3{
            print!("| {} |",board[y][x]); 
        }   
        println!("", )
    }
}

fn check_board(board: [[u8;3];3] ) -> bool{
    for x in 0..3 {
        for y in 0..3
        {
            if board[y][x] == 0
            {
            return  false; 
            }
        }   
    }

    return true; 