


#[cfg(test)]
pub mod tests {

    extern crate gomoku;

    use gomoku::*;


    #[test]
    pub fn test_board() {
        
        let mut board = Board::new(BOARDSIZE);
        let mut first = board.get(0,0);
        let last = board.get(8, 8);
        assert_eq!(last, 0b00);
        assert_eq!(first, 0b00);
        for _ in 0..9000000 {
            board.set(8, 8, Players::PlayerTwo);
            first = board.get(8, 8);
        }
        for n in board.get_neighbors(0, 0) {
            println!("{} {}", n.0, n.1);
        }
        assert_eq!(first, 0b10);
    }



} 