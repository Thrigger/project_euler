const BORDER: u64 = 20;

pub fn solve() -> u64 {
    let mut mem = [[0u64; BORDER as usize + 1]; BORDER as usize +1];
    paths_from_node(&mut mem, 0,0)

}

fn paths_from_node(mem: &mut [[u64; BORDER as usize + 1]; BORDER as usize +1], x: u64, y: u64) -> u64 {
    let mut paths = 0;

    if x == BORDER && y == BORDER {
        return 1;
    } else if mem[x as usize][y as usize] != 0 {
        return mem[x as usize][y as usize];
    } else {
        if x < BORDER {
            paths += paths_from_node(mem, x+1, y);
        } 
        if y < BORDER {
            paths += paths_from_node(mem, x, y+1);
        }
    }
    mem[x as usize][y as usize]=paths;
    paths
}

