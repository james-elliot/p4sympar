type Vals = i8;
type Depth = u8;
type Colors = i8;
type Sigs = u64;

const SIZEX: usize = 6;
const SIZEY: usize = 7;

const FOUR: usize = 4;

const MAXDEPTH: Depth = (SIZEX * SIZEY - 1) as Depth;

const EMPTY: Colors = 0;
const WHITE: Colors = 1;
const BLACK: Colors = -WHITE;

type HVals = [[Sigs; SIZEY]; SIZEX];
type Board = [[Colors; SIZEY]; SIZEX];

const NB_BITS: u8 = 30;
const HASH_SIZE: usize = 1 << NB_BITS;
const HASH_MASK: Sigs = (1 << NB_BITS) - 1;
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct HashElem {
    sig: Sigs,
    v_inf: Vals,
    v_sup: Vals,
    d: Depth,
}
const ZHASH: HashElem = HashElem {
    sig: 0,
    v_inf: 0,
    v_sup: 0,
    d: 0,
};

//type HTable = Box<[HashElem; HASH_SIZE]>;

use std::sync::Mutex;
type HTable2 = Box<[Mutex<HashElem>; HASH_SIZE]>;
const ZHASH_M: Mutex<HashElem> = Mutex::new(ZHASH);
//static GLOBAL_VARIABLE: [Mutex<HashElem>; HASH_SIZE] = [ZHASH_M; HASH_SIZE];

#[allow(dead_code)]
fn eval3(x: usize, y: usize, color: Colors, tab: &Board) -> Vals {
    // Below
    let mut nb = 1;
    if y >= 3 {
        for d in 1..FOUR {
            if tab[x][y - d] == color {
                nb = nb + 1;
            } else {
                break;
            }
        }
        if nb >= FOUR {
            return color as Vals;
        }
    }

    // Horizontal
    nb = 1;
    for d in 1..min(FOUR, SIZEX - x) {
        if tab[x + d][y] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    for d in 1..min(FOUR, x + 1) {
        if tab[x - d][y] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    if nb >= FOUR {
        return color as Vals;
    }

    // Diag 1
    nb = 1;
    for d in 1..min(FOUR, min(SIZEX - x, SIZEY - y)) {
        if tab[x + d][y + d] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    for d in 1..min(FOUR, min(x + 1, y + 1)) {
        if tab[x - d][y - d] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    if nb >= FOUR {
        return color as Vals;
    }

    // Diag 2
    nb = 1;
    for d in 1..min(FOUR, min(SIZEX - x, y + 1)) {
        if tab[x + d][y - d] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    for d in 1..min(4, min(x + 1, SIZEY - y)) {
        if tab[x - d][y + d] == color {
            nb = nb + 1;
        } else {
            break;
        }
    }
    if nb >= FOUR {
        return color as Vals;
    }

    return 0;
}

// eval2 is slightly faster
#[allow(dead_code)]
fn eval2(x: usize, y: usize, color: Colors, tab: &Board) -> Vals {
    /* Vertical */
    if y >= FOUR - 1 {
        let mut d = 0;
        let nb = loop {
            d = d + 1;
            let j = y - d;
            if tab[x][j] != color {
                break d - 1;
            }
            if (j == 0) || (d == FOUR - 1) {
                break d;
            }
        };
        if nb >= FOUR - 1 {
            return color as Vals;
        }
    }

    /* Horizontal */
    {
        let mut nb = 0;
        if x < SIZEX - 1 {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x + d;
                if tab[i][y] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if x > 0 {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x - d;
                if tab[i][y] != color {
                    break d - 1;
                }
                if (i == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if nb >= FOUR - 1 {
            return color as Vals;
        }
    }

    /* Diag 1 */
    {
        let mut nb = 0;
        if (x > 0) && (y > 0) {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x - d;
                let j = y - d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == 0) || (j == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if (x < SIZEX - 1) && (y < SIZEY - 1) {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x + d;
                let j = y + d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (j == SIZEY - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if nb >= FOUR - 1 {
            return color as Vals;
        }
    }

    /* Diag 2 */
    {
        let mut nb = 0;
        if (x > 0) && (y < SIZEY - 1) {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x - d;
                let j = y + d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == 0) || (j == SIZEY - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if (x < SIZEX - 1) && (y > 0) {
            let mut d = 0;
            let res = loop {
                d = d + 1;
                let i = x + d;
                let j = y - d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (j == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb = nb + res;
        }
        if nb >= FOUR - 1 {
            return color as Vals;
        }
    }
    return 0;
}

fn build_hashes() -> (Sigs, Sigs, HVals, HVals) {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut hashesw = [[0; SIZEY]; SIZEX];
    let mut hashesb = [[0; SIZEY]; SIZEX];
    let turn_hash = rng.gen();
    let first_hash = rng.gen();
    for i in 0..SIZEX {
        for j in 0..SIZEY {
            hashesw[i][j] = rng.gen();
            hashesb[i][j] = rng.gen();
        }
    }
    return (turn_hash, first_hash, hashesw, hashesb);
}

fn retrieve(hv: Sigs, hashes: &HTable2) -> Option<(Vals, Vals)> {
    let ind = (hv & HASH_MASK) as usize;
    let data = hashes[ind].lock().unwrap();
    if data.sig == hv {
        return Some((data.v_inf, data.v_sup));
    } else {
        return None;
    };
}

use core::cmp::{max, min};

fn store(hv: Sigs, alpha: Vals, beta: Vals, g: Vals, depth: Depth, hashes: &HTable2) {
    let ind = (hv & HASH_MASK) as usize;
    let d = MAXDEPTH + 2 - depth;
    let mut data = hashes[ind].lock().unwrap();
    if data.d <= d {
        if data.sig != hv {
            data.d = d;
            data.v_inf = Vals::MIN;
            data.v_sup = Vals::MAX;
            data.sig = hv;
        }
        if (g > alpha) && (g < beta) {
            data.v_inf = g;
            data.v_sup = g;
        } else if g <= alpha {
            data.v_sup = min(g, data.v_sup);
        } else if g >= beta {
            data.v_inf = max(g, data.v_inf);
        }
    }
}

fn ab(
    alpha: Vals,
    beta: Vals,
    color: Colors,
    depth: Depth,
    tab: &mut Board,
    first: &mut [usize; SIZEX],
    nodes: &mut u64,
    hv: Sigs,
    hv2: Sigs,
    turn_hash: Sigs,
    first_hash: Sigs,
    hashesw: HVals,
    hashesb: HVals,
    hashes: &HTable2,
) -> Vals {
    *nodes = *nodes + 1;

    //   if hv != compute_hash(color,tab,first_hash,turn_hash,hashesw,hashesb) {panic!("Bad hash");}

    let mut a = alpha;
    let mut b = beta;

    match retrieve(min(hv, hv2), hashes) {
        Some((v_inf, v_sup)) => {
            if v_inf == v_sup {
                return v_inf;
            }
            if v_inf >= b {
                return v_inf;
            }
            if v_sup <= a {
                return v_sup;
            }
            a = max(a, v_inf);
            b = min(b, v_sup);
        }
        None => {}
    }

    for x in 0..SIZEX {
        let y = first[x];
        if y != SIZEY {
            let v = eval2(x, y, color, tab);
            if v != 0 {
                return v;
            }
        }
    }
    if depth == MAXDEPTH {
        return 0;
    }
    let mut g;
    if color == WHITE {
        g = Vals::MIN;
    } else {
        g = Vals::MAX;
    }

    for ix in 0..SIZEX {
        let x = (SIZEX - 1) / 2 + (ix + 1) / 2 * (2 * (ix % 2)) - (ix + 1) / 2;
        let y = first[x];
        if y < SIZEY {
            tab[x][y] = color;
            first[x] = first[x] + 1;
            let nhv;
            let nhv2;
            if color == WHITE {
                nhv = hv ^ hashesw[x][y];
                nhv2 = hv2 ^ hashesw[SIZEX - 1 - x][y];
            } else {
                nhv = hv ^ hashesb[x][y];
                nhv2 = hv2 ^ hashesb[SIZEX - 1 - x][y];
            }
            let v = ab(
                a,
                b,
                -color,
                depth + 1,
                tab,
                first,
                nodes,
		// turn_hash is useless in connect4
                nhv ^ turn_hash,nhv2 ^ turn_hash,
//                nhv,nhv2,
                turn_hash,
                first_hash,
                hashesw,
                hashesb,
                hashes,
            );
            first[x] = first[x] - 1;
            tab[x][y] = EMPTY;
            if color == WHITE {
                if v > g {
                    g = v;
                    if g > a {
                        a = g;
                        if a >= b {
                            break;
                        }
                    }
                }
            } else {
                if v < g {
                    g = v;
                    if g < b {
                        b = g;
                        if a >= b {
                            break;
                        }
                    }
                }
            }
        }
    }
    store(min(hv, hv2), alpha, beta, g, depth, hashes);
    return g;
}

fn compute_hash(
    color: Colors,
    tab: &mut Board,
    first_hash: Sigs,
    turn_hash: Sigs,
    hashesw: HVals,
    hashesb: HVals,
) -> Sigs {
    let mut h = first_hash;
    if color == BLACK {
        h = h ^ turn_hash;
    }
    for i in 0..SIZEX {
        for j in 0..SIZEY {
            match tab[i][j] {
                BLACK => {
                    h = h ^ hashesb[i][j];
                }
                WHITE => {
                    h = h ^ hashesw[i][j];
                }
                _ => {}
            }
        }
    }
    return h;
}

fn main() {
    use std::time::{Instant, SystemTime};
    let mut tab = [[EMPTY; SIZEY]; SIZEX];
    let mut first = [0; SIZEX];
    let mut nodes = 0;
//    let mut hashes = Box::new([ZHASH; HASH_SIZE]);
    let hashes = Box::new([ZHASH_M; HASH_SIZE]);

    let (turn_hash, first_hash, hashesw, hashesb) = build_hashes();
    let hv = compute_hash(WHITE, &mut tab, first_hash, turn_hash, hashesw, hashesb);
    let hv2 = first_hash;
    if hv != hv2 {
        panic!("Why???");
    };
    let now = Instant::now();
    let snow = SystemTime::now();
    let ret = ab(
        Vals::MIN,
        Vals::MAX,
        WHITE,
        0,
        &mut tab,
        &mut first,
        &mut nodes,
        hv,
        hv2,
        turn_hash,
        first_hash,
        hashesw,
        hashesb,
        &hashes,
    );
    println!("wall_clock={:?}", now.elapsed());
    println!("system_clock={:?}", snow.elapsed().unwrap());
    println!("ret={}", ret);
    println!("nodes={}", nodes);
}
