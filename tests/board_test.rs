


#[cfg(test)]
pub mod tests {

    extern crate gomoku;

    use gomoku::*;


    #[test]
    pub fn test_board_get_and_set() {
        
        let mut board = Board::new(BOARDSIZE);
        
        let mut first = board.get_state(0,0);
        let mut last = board.get_state(18, 18);
        assert_eq!(last, Players::Unplayed);
        assert_eq!(first, Players::Unplayed);
        board.print();

        board.set_state(0, 0, Players::PlayerOne);
        first = board.get_state(0, 0);
        assert_eq!(first, Players::PlayerOne);
        board.print();
        
        board.set_state(18, 18, Players::PlayerTwo);
        last = board.get_state(18, 18);
        assert_eq!(last, Players::PlayerTwo);
        board.print();

        board.set_state(0, 0, Players::Unplayed);
        first = board.get_state(0, 0);
        assert_eq!(first, Players::Unplayed);
        board.print();
        
        for _ in 0..1000000 {
            board.set_state(8, 8, Players::PlayerTwo);
            first = board.get_state(8, 8);
            let _ = board.is_legal(0, 0, Players::PlayerOne);
        }
        assert_eq!(first, Players::PlayerTwo);
        board.print();
    }
    #[test]
    
    pub fn test_scanning() {
        let mut board = Board::new(BOARDSIZE);

        board.set_state(1, 0, Players::PlayerOne);
        board.set_state(2, 0, Players::PlayerOne);
        board.set_state(3, 0, Players::PlayerOne);
        let pattern = 84u64;
        let len = 5usize;

        let found = board.scan_position(0, 0, pattern, len);
        assert_eq!(found, 1);
        board.print();
    }

} 