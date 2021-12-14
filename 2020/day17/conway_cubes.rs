use itertools::iproduct;

const MAX_X: usize = 22;
const MAX_Y: usize = 22;
const MAX_Z: usize = 15;
const MAX_W: usize = 15;
const OFFSET: usize = 7;
const NEIGHBOR_OFFSETS: [isize; 3] = [0, -1, 1];

type Matrix = [[[bool; MAX_Z]; MAX_Y]; MAX_X];
type Matrix4 = [[[[bool; MAX_W]; MAX_Z]; MAX_Y]; MAX_X];

/// create cartesian_product of both the full list of simulation coordinates
/// but starting at 1 and up to max-1 to leave out the edge
/// so that no bounds check is necessary when adding the
/// displacement of coords from (-1, 0, 1) * 3 (dropping the initial (0, 0, 0))
fn step(matrix: &mut Matrix) {
    let mut changes = Vec::new();
    let coords = iproduct!(1..(MAX_X - 1), 1..(MAX_Y - 1), 1..(MAX_Z - 1));
    let displacement = iproduct!(&NEIGHBOR_OFFSETS, &NEIGHBOR_OFFSETS, &NEIGHBOR_OFFSETS).skip(1);
    for (x, y, z) in coords {
        let neighbor_sum: u8 = displacement
            .clone()
            .map(|(&i, &j, &k)| {
                if matrix[(x as isize + i) as usize][(y as isize + j) as usize]
                    [(z as isize + k) as usize]
                {
                    1
                } else {
                    0
                }
            })
            .sum();
        //println!("{} {} {} -> {}", x, y, z, neighbor_sum);
        if matrix[x][y][z] && !(2..=3).contains(&neighbor_sum) {
            changes.push((x, y, z));
        }
        if !matrix[x][y][z] && neighbor_sum == 3 {
            changes.push((x, y, z));
        }
    }
    //println!("{:?}", changes);
    for (x, y, z) in changes {
        matrix[x][y][z] = !matrix[x][y][z];
    }
}

// code duplication for now, could do w dimension as Vec but that seems messy
fn step4(matrix: &mut Matrix4) {
    let mut changes = Vec::new();
    let coords = iproduct!(
        1..(MAX_X - 1),
        1..(MAX_Y - 1),
        1..(MAX_Z - 1),
        1..(MAX_W - 1)
    );
    let displacement = iproduct!(
        &NEIGHBOR_OFFSETS,
        &NEIGHBOR_OFFSETS,
        &NEIGHBOR_OFFSETS,
        &NEIGHBOR_OFFSETS
    )
    .skip(1);
    for (x, y, z, w) in coords {
        let neighbor_sum: u8 = displacement
            .clone()
            .map(|(&i, &j, &k, &l)| {
                if matrix[(x as isize + i) as usize][(y as isize + j) as usize]
                    [(z as isize + k) as usize][(w as isize + l) as usize]
                {
                    1
                } else {
                    0
                }
            })
            .sum();
        //println!("{} {} {} -> {}", x, y, z, neighbor_sum);
        if matrix[x][y][z][w] && !(2..=3).contains(&neighbor_sum) {
            changes.push((x, y, z, w));
        }
        if !matrix[x][y][z][w] && neighbor_sum == 3 {
            changes.push((x, y, z, w));
        }
    }
    //println!("{:?}", changes);
    for (x, y, z, w) in changes {
        matrix[x][y][z][w] = !matrix[x][y][z][w];
    }
}

pub fn run() -> String {
    let input: Vec<&str> = include_str!("input")
        .split('\n')
        .map(|a| a.trim())
        .collect();
    println!("input:\n{:?}\n", input);

    // need a three dimensional data structure
    // since there's only 6 rounds, only need max initial-dimensions + 12 + 2
    // (to allow neighbour access without checking bounds, not processing cells on the edge)
    let mut matrix: Matrix = [[[false; MAX_Z]; MAX_Y]; MAX_X];
    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            matrix[x + OFFSET][y + OFFSET][OFFSET] = cell == '#';
        }
    }
    for _ in 0..6 {
        step(&mut matrix);
    }
    // count active cells:
    let active_count: u64 = matrix
        .iter()
        .flatten()
        .flatten()
        .map(|&b| if b { 1 } else { 0 })
        .sum();
    println!("Active count {}", active_count);

    let mut matrix4: Matrix4 = [[[[false; MAX_W]; MAX_Z]; MAX_Y]; MAX_X];
    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            matrix4[x + OFFSET][y + OFFSET][OFFSET][OFFSET] = cell == '#';
        }
    }
    for _ in 0..6 {
        step4(&mut matrix4);
    }
    // count active cells:
    let active_count: u64 = matrix4
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .map(|&b| if b { 1 } else { 0 })
        .sum();
    println!("Active count 4D {}", active_count);
    format!("TODO")

    // Better Ideas:

    // nested arrays with fixed dimension,
    // maybe "override the Index trait", so I can use tuple coordinates
    // with automatic translation from cartesian coords to 0-based indexing

    // generator for getting the neighbor coordinates!
}
