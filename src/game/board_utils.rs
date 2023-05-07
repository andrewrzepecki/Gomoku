use crate::*;

pub fn get_u64_state(pattern: u64, idx: usize) -> Players {
    let bit_pos = idx as u32 * 2;
    let cell_state = (pattern >> bit_pos) & 0b11;
    match cell_state {
        0b00 => Players::Unplayed,
        0b01 => Players::PlayerOne,
        0b10 => Players::PlayerTwo,
        _ => Players::Unplayed,
    }
}

pub fn set_u64_state(pattern: &mut u64, idx: usize, to_set: Players) {
    let bit_pos = idx as u32 * 2;
    let ts = match to_set {
        Players::PlayerOne => 1u64,
        Players::PlayerTwo =>  2u64,
        Players::Unplayed => 0u64,
    };
    let mask = ts << bit_pos;
    if ts != 0u64 {
        *pattern |= mask;
    }
    else {
        let zmask = 0b11 << bit_pos;
        *pattern &= !zmask;
    }
}

pub fn is_symmetrical(pattern: u64, len: usize) -> bool {
    let mut i = 0;
    while i < len / 2 {
        if get_u64_state(pattern, i) != get_u64_state(pattern, len - i - 1) {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn invert_pattern(pattern: u64, len: usize) -> u64 {
    let mut inverted = 0u64;
    for idx in 0..len {
        set_u64_state(&mut inverted, idx, get_opponent(get_u64_state(pattern, idx)));
    }
    inverted
}

pub fn make_inverted_table() -> HashMap<String, HashMap<u64, (usize, i32, bool)>> {
    let pattern_table = make_pattern_table();
    let mut inverted = HashMap::new();
    for (name, map) in &pattern_table {
        let mut imap = HashMap::new();
        for (pattern, (len, score, is_live)) in map {
            imap.insert(invert_pattern(*pattern, *len), (*len, *score, *is_live));
        }
        inverted.insert((*name).clone().into(), imap);
    }
    inverted 
}

pub fn make_pattern_table() -> HashMap<String, HashMap<u64, (usize, i32, bool)>> {
    let mut hmap = HashMap::new();

    // Make Illegal Three patterns
    let mut free_three_map = HashMap::new();
    free_three_map.insert(21 as u64, (4 as usize, 1 as i32, true));
    free_three_map.insert(81 as u64, (5 as usize, 1 as i32, true));
    // free_three_map.insert(324 as u64, (5 as usize, 1 as i32, true));
    hmap.insert("free_threes".into(), free_three_map);

    // Five in a row pattern
    let mut five_map = HashMap::new();
    five_map.insert(341 as u64, (5 as usize, 1 as i32, false));
    hmap.insert("five_in_a_row".into(), five_map);

    // Pattern Table for table scoring
    let mut score_map = HashMap::new();

    // Five in a row
    score_map.insert(341 as u64, (5 as usize, 100000 as i32, false));
    hmap.insert("score_table".into(), score_map);

    hmap
}

pub fn get_opponent(player: Players) -> Players {
    match player {
        Players::PlayerOne => Players::PlayerTwo,
        Players::PlayerTwo => Players::PlayerOne,
        Players::Unplayed => Players::Unplayed
    }
}