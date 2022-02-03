// put the board here!
fn init_sudo(sudo: &mut [[u8; 9]; 9], poss: &mut [[u16; 9]; 9])
{
    let puzzle: [[u8; 9]; 9] = [[5, 3, 0, 0, 7, 0, 0, 0, 0],
                                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                                [0, 0, 0, 0, 8, 0, 0, 7, 9]];

    for i in 0..9
    {
        for j in 0..9
        {
            set_space(sudo, poss, i, j, puzzle[i][j]);
        }
    }
}

// set all possibilities to default (anything)
fn init_poss(poss: &mut [[u16; 9]; 9])
{
    for i in 0..9
    {
        for j in 0..9
        {
            poss[i][j] = 0x1ff;
        }
    }
}

// mark a spot as taken and then update the possibilities
fn set_poss(poss: &mut [[u16; 9]; 9], row: usize, col: usize, set: u8)
{
    poss[row][col] = 0x8000;

    // update the possibles for the square
    let sr = row / 3 * 3;
    let sc = col / 3 * 3;
    for i in 0..3
    {
        for j in 0..3
        {
            if is_valid(poss[sr + i][sc + j], set) == 1
            {
                poss[sr + i][sc + j] = poss[sr + i][sc + j] ^ (0x1 << (set - 1));
            }
        }
    }

    // update poss for row and col
    for i in 0..9
    {
        if is_valid(poss[row][i], set) == 1
        {
            poss[row][i] = poss[row][i] ^ (0x1 << (set - 1));
        }

        if is_valid(poss[i][col], set) == 1
        {
            poss[i][col] = poss[i][col] ^ (0x1 << (set - 1));
        }
    }
}

// check if the given number is valid for the given spot. 1 if yes, 0 if no
fn is_valid(space: u16, n: u8) -> u8
{
    if (space & (0x1 << n - 1)) > 0 { 1 } else { 0 }
}

// sets the space to the given number if it is valid, then updates the possibilities
fn set_space(sudo : &mut [[u8; 9]; 9], poss: &mut [[u16; 9]; 9], row: usize, col: usize, set: u8) -> u8
{
    if set == 0 || set > 9
        { return 1; }

    if is_valid(poss[row][col], set) == 1
    {
        set_poss(&mut *poss, row, col, set);
        sudo[row][col] = set;
        return 0;
    }
    1
}

// prints out the sudoku board with some format lines
fn print_sudo(sudo: &[[u8; 9]; 9])
{
    for i in 0..9
    {
        for j in 0..9
        {
            print!("{} ", sudo[i][j]);
            if j == 2 || j == 5
            {
                print!("| ");
            }
        }
        print!("\n");
        if i == 2 || i == 5
        {
            println!("------+-------+------");
        }
    }
}

// if the space can only be one number, returns that number, otherwise 0
fn is_solo(space: u16) -> u8
{
    if space == 0x8000
    {
        return 0;
    }

    let mut p: u16 = 1;
    for i in 0..9
    {
        if space == p
        {
            return i + 1;
        }
        p = p << 1;
    }
    0
}

// if the given space is the only place for a number, returns that number. Otherwise 0.
fn is_only(poss: &[[u16; 9]; 9], row: usize, col: usize) -> u8
{
    if poss[row][col] == 0x8000
        { return 0; }

    let sr = row / 3 * 3;
    let sc = col / 3 * 3;
    let mut cont;

    // for each number
    for k in 1..10
    {
        // check the space itself
        if is_valid(poss[row][col], k) == 0
            { continue; }

        cont = 0;

        // check the possibles for the square
        for i in 0..3
        {
            for j in 0..3
            {
                if (sr + i != row || sc + j != col) && is_valid(poss[sr + i][sc + j], k) == 1
                    { cont = 1; }
            }
        }

        if cont == 0
            { return k; }

        // check poss for row and col
        for i in 0..9
        {
            if i != col && is_valid(poss[row][i], k) == 1
                { cont = 1; }
        }

        if cont == 0
            { return k; }

        for i in 0..9
        {
            if i != row && is_valid(poss[i][col], k) == 1
                { cont = 1; }
        }

        if cont == 0
            { return k; }
    }
    0
}

// uses process of elimination to solve the sudoku
fn eliminate(sudo: &mut [[u8; 9]; 9], poss: &mut [[u16; 9]; 9])
{
    let mut r = 0;
    let mut c = 0;
    let mut res1;
    let mut res2;

    while r < 9
    {
        while c < 9
        {
            res1 = is_solo(poss[r][c]);
            res2 = is_only(&poss, r, c);
            if res1 > 0 || res2 > 0
            {
                //println!("Found a number! [{}, {}] is {}", r, c, if res1 > 0 {res1} else {res2});
                if set_space(sudo, poss, r, c, if res1 > 0 {res1} else {res2}) > 0
                {
                    println!("{:b}\nsolo: {}\nonly: {}", poss[r][c], res1, res2);
                    println!("Miscalculation in eliminate!");
                    return;
                }
                r = 0;
                c = 0;
            }
            else
                { c += 1; }
        }
        c = 0;
        r += 1;
    }
}

fn main() {
    let mut sudo  = [[0u8; 9]; 9];

    // bitmap of possible values for each square
    // 0000 0000 0000 0000
    // T       9 8765 4321
    let mut possible = [[0u16; 9]; 9];

    init_poss(&mut possible);

    init_sudo(&mut sudo, &mut possible);

    print_sudo(&sudo);

    // begin solving
    println!("================================================");
    println!("Solving...");
    println!("================================================");

    eliminate(&mut sudo, &mut possible);

    print_sudo(&sudo);
}
