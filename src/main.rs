fn main() {
    let mut board:[[u8; 3]; 3] = [[0; 3] ; 3];

    for x in 0..3 {
        for y in 0..3 {
            board[y][x] =  (0) as u8;
        }
    }   

    display_board(&mut board);
    println!("-------------------");
    set_values(5,1,1,&mut board); 
    display_board(&mut board);
    

}

fn set_values(value: u8 ,col:usize, row:usize, board: &mut [[u8; 3];3]){
    board[row][col] = value;
}


fn display_board(board: &mut [[u8; 3];3]){
    for x in 0..3 {
      for y in 0..3{
            print!("| {} |",board[y][x]); 
        }   
        println!("", )
    }
}

