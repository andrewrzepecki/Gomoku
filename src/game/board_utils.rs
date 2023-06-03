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
    // Make sure bit shifting operations are done on a 00 index.
    if to_set != Players::Unplayed {
        set_u64_state(pattern, idx, Players::Unplayed);
    }
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

pub fn make_inverted_table() -> HashMap<String, HashMap<String, (u64, usize, bool, i32)>> {
    let pattern_table = make_pattern_table();
    let mut inverted = HashMap::new();
    for (name, map) in &pattern_table {
        let mut imap = HashMap::new();
        for (pattern_name, (pattern, len, is_live, score)) in map {
            imap.insert((*pattern_name).clone().into(), (invert_pattern(*pattern, *len), *len, *is_live, *score));
        }
        inverted.insert((*name).clone().into(), imap);
    }
    inverted 
}

// Patterns as follow:
pub fn make_pattern_table() -> HashMap<String, HashMap<String, (u64, usize, bool, i32)>> {
    let mut hmap = HashMap::new();

    // Make Illegal Three patterns [pattern_name: (pattern, size, is_live, score)]
    let mut free_three_map = HashMap::new();
    free_three_map.insert("_three".into(), (21 as u64, 4 as usize, true, 1 as i32));
    free_three_map.insert("open_three_1".into(), (69 as u64, 5 as usize, true, 1 as i32));
    hmap.insert("free_threes".into(), free_three_map);

    // Five in a row pattern
    let mut five_map = HashMap::new();
    five_map.insert("winner_pattern".into(), (341 as u64, 5 as usize, false, 1 as i32));
    hmap.insert("five_in_a_row".into(), five_map);

    // Pattern Table for table scoring
    let mut score_map = HashMap::new();
    score_map.insert("winner_pattern".into(), (341 as u64, 5 as usize, false, 100000 as i32));

    score_map.insert("live_four".into(), (85 as u64, 5 as usize, true, 100000 as i32));
    score_map.insert("dead_four_0".into(), (85 as u64, 5 as usize, false, 30000 as i32));
    score_map.insert("dead_four_1".into(), (337 as u64, 6 as usize, true, 30000 as i32));
    score_map.insert("dead_four_2".into(), (325 as u64, 6 as usize, true, 30000 as i32));

    score_map.insert("live_three".into(), (21 as u64, 4 as usize, true, 10000 as i32));
    score_map.insert("dead_three_0".into(), (81 as u64, 5 as usize, true, 1000 as i32));
    score_map.insert("dead_three_1".into(), (21 as u64, 4 as usize, false, 1000 as i32));
    score_map.insert("dead_three_2".into(), (81 as u64, 5 as usize, false, 1000 as i32));
    score_map.insert("dead_three_3".into(), (69 as u64, 5 as usize, false, 1000 as i32));
    score_map.insert("dead_three_4".into(), (321 as u64, 6 as usize, true, 1000 as i32));
    score_map.insert("dead_three_5".into(), (273 as u64, 6 as usize, true, 1000 as i32));
    score_map.insert("dead_three_5".into(), (2132 as u64, 6 as usize, false, 1000 as i32));

    score_map.insert("live_two_0".into(), (257 as u64, 6 as usize, true, 500 as i32));
    score_map.insert("live_two_1".into(), (65 as u64, 5 as usize, true, 500 as i32));
    score_map.insert("live_two_2".into(), (17 as u64, 4 as usize, true, 500 as i32));
    score_map.insert("live_two_3".into(), (5 as u64, 3 as usize, true, 100 as i32));
    score_map.insert("dead_two_0".into(), (5 as u64, 3 as usize, false, 100 as i32));
    score_map.insert("dead_two_1".into(), (17 as u64, 4 as usize, false, 100 as i32));
    score_map.insert("dead_two_2".into(), (65 as u64, 5 as usize, false, 100 as i32));

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