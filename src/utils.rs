
pub fn is_valid_coords(x : i32, y : i32, size : i32) -> bool {
    if x < 0 || x >= size {
        return false;
    }
    if y < 0 || y >= size {
        return false;
    }
    return true;
}



pub fn get_neighbours(x : i32, y : i32, size : i32) -> impl Iterator<Item = (i32, i32)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
        if y > 0 {
            neighbors.push((x - 1, y - 1));
        }
        if y < size - 1 {
            neighbors.push((x - 1, y + 1));
        }
    }
    if y > 0 {
        neighbors.push((x, y - 1));
        if x < size - 1 {
            neighbors.push((x + 1, y - 1));
        }
    }
    if x < size - 1 {
        neighbors.push((x + 1, y));
        if y < size - 1 {
            neighbors.push((x + 1, y + 1));
        }
    }
    if y < size - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors.into_iter()
}
