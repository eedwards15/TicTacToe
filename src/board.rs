pub mod board_helper
{
    pub fn display_board(board: &mut [[u8; 3];3]){
        for x in 0..3 {
        for y in 0..3{
                print!("| {} |",board[x][y]); 
            }   
            println!("", )
        }
    }

    pub fn check_board(board: [[u8;3];3] ) -> bool{
        for row in 0..3 {
            for col in 0..3
            {
                if board[row][col] == 0
                {
                return  false; 
                }
            }   
        }

        return true; 
    }

    pub fn set_values(value: u8 ,col:usize, row:usize, board: &mut [[u8; 3];3]) -> bool{
        if board[row][col] == 0 
        {
            board[row][col] = value;
            return true; 
        }
        return false; 
    }

    pub fn did_win(board: [[u8; 3];3]) -> bool
    {
        println!("{}", (board[0][0] == board[0][1]));
        if(board[0][0] == board[0][1] && board[0][1] == board[0][2] && board[0][2] != 0)
        {
            return true; 
        }

        if(board[1][0] == board[1][1] && board[1][1] == board[1][2] && board[1][2] != 0)
        {
            return true; 
        }


        if(board[2][0] == board[2][1] && board[2][1] == board[2][2]&& board[2][2] != 0)
        {
            return true; 
        }

        if(board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[2][0] != 0)
        {
            return true; 
        }

        if(board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[2][2] != 0)
        {
            return true; 
        }

        return false; 
    }



}