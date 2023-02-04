use std::time::{Instant, SystemTime};
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use core::cmp::{max, min};


type Vals = i8;
type Depth = u8;
type Colors = i8;
type Sigs = u64;

const SIZEX: usize = 6;
const SIZEY: usize = 6;

const FOUR: usize = 4;

const MAXDEPTH: Depth = (SIZEX * SIZEY - 1) as Depth;

const EMPTY: Colors = 0;
const WHITE: Colors = 1;
const BLACK: Colors = -WHITE;

type HVals = [[Sigs; SIZEY]; SIZEX];
type Board = [[Colors; SIZEY]; SIZEX];

const NB_BITS: u8 = 27;
const HASH_SIZE: usize = 1 << NB_BITS;
const HASH_MASK: Sigs = (1 << NB_BITS) - 1;

const VALMAX : Vals = Vals::MAX;

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

type HTable = Vec<Mutex<HashElem>>;

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

// The fastest but with unsafe array access
#[allow(dead_code)]
fn eval(x: usize, y: usize, color: Colors, tab: &Board) -> bool {
    unsafe{
	/* Vertical */
	if y >= FOUR - 1 {
            let mut d = 0;
	    let mut j = y;
            loop {
		d += 1;
		j -= 1;
		if *tab.get_unchecked(x).get_unchecked(j) != color {break;}
		if d == FOUR - 1 {return true;}
		if j == 0 {break;}
            };
	}
	
	/* Horizontal */
	{
            let mut nb = 0;
            if x < SIZEX - 1 {
		let mut d = 0;
		let mut i = x;
		nb = loop {
                    d += 1;
                    i += 1;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if i == SIZEX - 1 {break d;}
		};
            }
            if x > 0 {
		let mut d = 0;
		let mut i = x;
		loop {
                    d += 1;
                    i -= 1;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if i == 0 {break;}
		};
            }
	}
	
	/* Diag 1 */
	{
            let mut nb = 0;
            if (x > 0) && (y > 0) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		nb = loop {
                    d += 1;
                    i -= 1;
                    j -= 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if (i == 0) || (j == 0) {break d;}
		};
            }
            if (x < SIZEX - 1) && (y < SIZEY - 1) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		loop {
                    d += 1;
                    i += 1;
                    j += 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if (i == SIZEX - 1) || (j == SIZEY - 1) {break;}
		};
	    }
	}
	
	/* Diag 2 */
	{
            let mut nb = 0;
            if (x > 0) && (y < SIZEY - 1) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		nb = loop {
                    d += 1;
                    i -= 1;
                    j += 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if d == FOUR - 1 {return true;}
		    if (i == 0) || (j == SIZEY - 1) {break d;}
		};
            }
            if (x < SIZEX - 1) && (y > 0) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		loop {
                    d += 1;
                    i += 1;
                    j -= 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
                    if d + nb == FOUR - 1 {return true;}
		    if (i == SIZEX - 1) || (j == 0) {break;}
		};
            }
	}
	false
    }
}

lazy_static! {
    static ref HW:HVals = {
	let mut rng = thread_rng();
	let mut t = [[0; SIZEY]; SIZEX];
	for item in t.iter_mut() {
	    for item2 in item.iter_mut() {
		*item2 = rng.gen();
	    }
	}
	t
    };
    static ref HB:HVals = {
	let mut rng = thread_rng();
	let mut t = [[0; SIZEY]; SIZEX];
	for item in t.iter_mut() {
	    for item2 in item.iter_mut() {
		*item2 = rng.gen();
	    }
	}
	t
    };
    static ref FH:Sigs = {
	let mut rng = thread_rng();
	rng.gen()
    };
    static ref IND:[usize;SIZEX]= {
	let mut t = [0;SIZEX];
	for (ix,item) in t.iter_mut().enumerate() {
	    *item=(SIZEX - 1) / 2 + (ix + 1) / 2 * (2 * (ix % 2)) - (ix + 1) / 2;
	}
	t
    };
}

fn retrieve(hv: Sigs, hashes: &HTable) -> Option<(Vals, Vals)> {
    let ind = (hv & HASH_MASK) as usize;
    let data = hashes[ind].lock().unwrap();
    if data.sig == hv {return Some((data.v_inf, data.v_sup));}
    else {return None;};
}

fn store(hv: Sigs, alpha: Vals, beta: Vals, g: Vals, depth: Depth, hashes: &HTable) {
    let ind = (hv & HASH_MASK) as usize;
    let d = MAXDEPTH + 2 - depth;
    let mut data = hashes[ind].lock().unwrap();
    if data.d <= d {
        if data.sig != hv {
            data.d = d;
            data.v_inf = -VALMAX;
            data.v_sup = VALMAX;
            data.sig = hv;
        }
        if (g > alpha) && (g < beta) {
            data.v_inf = g;
            data.v_sup = g;
        }
	else if g <= alpha {data.v_sup = min(g, data.v_sup);}
	else if g >= beta {data.v_inf = max(g, data.v_inf);}
    }
}
struct Args<'a> {
    alpha: Vals,
    beta: Vals,
    color: Colors,
    depth: Depth,
    tab: &'a mut Board,
    first: &'a mut [usize; SIZEX],
    hv: Sigs,
    hv2: Sigs,
    hashes: &'a mut HTable
}

fn ab2(args: Args) -> Vals {
    let mut a = args.alpha;
    let mut b = args.beta;
    if let Some((v_inf,v_sup)) = retrieve(min(args.hv, args.hv2), args.hashes) {
        if v_inf == v_sup {return v_inf;}
        if v_inf >= b {return v_inf;}
        if v_sup <= a {return v_sup;}
        a = max(a, v_inf);
        b = min(b, v_sup);
    }
    for ix in 0..SIZEX {
	let x = IND[ix];
        let y = args.first[x];
	if (y != SIZEY) && eval(x, y, args.color, args.tab) {return 1;}
    }
    if args.depth == MAXDEPTH {return 0;}
    let mut g = -VALMAX;
    let hvl = if args.color==WHITE {*HW} else {*HB};
    for ix in 0..SIZEX {
	let x = IND[ix];
        let y = args.first[x];
        if y < SIZEY {
            args.tab[x][y] = args.color;
            args.first[x] += 1;
	    let args2 = Args {alpha:-b,beta:-a,color:-args.color,depth:args.depth+1,
			      tab:args.tab,first:args.first,
			      hv:args.hv^hvl[x][y],hv2:args.hv2^hvl[SIZEX-1-x][y],
			      hashes:args.hashes};
	    g=max(g,-ab2(args2));
            args.first[x] -= 1;
            args.tab[x][y] = EMPTY;
	    a = max(a,g);
	    if a >= b {break;}
        }
    }
    store(min(args.hv, args.hv2), args.alpha, args.beta, g, args.depth, args.hashes);
    g
}

fn compute_hash(tab: &mut Board) -> Sigs {
    let mut h = *FH;
    for (i,row) in tab.iter_mut().enumerate() {
	for (j,e) in row.iter_mut().enumerate() {
	    match *e {
                BLACK => {h ^= HB[i][j];}
                WHITE => {h ^= HW[i][j];}
                _ => {}
            }
        }
    }
    h
}

fn main() {
    let mut tab = [[EMPTY; SIZEY]; SIZEX];
    let mut first = [0; SIZEX];
    let mut hashes = (0..HASH_SIZE).map(|_| Mutex::new(ZHASH)).collect();
    
    let hv = compute_hash(&mut tab);
    let hv2 = *FH;
    if hv != hv2 {panic!("Why???");};
    let now = Instant::now();
    let snow = SystemTime::now();
    let args0 =
	Args{alpha:-VALMAX, beta:VALMAX, color:WHITE, depth:0, tab:&mut tab,
	     first: &mut first, hv, hv2,hashes:&mut hashes };
    let ret = ab2(args0);
    println!("wall_clock={:?}", now.elapsed());
    println!("system_clock={:?}", snow.elapsed().unwrap());
    println!("ret={}", ret);
}
