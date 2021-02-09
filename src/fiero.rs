use array2d::Array2D;

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum Fiero {
    SPACE, DASH,
    E, I, II, O, OO, A, AA,
    P, B, T, D, K, G, CH, J, M, N, S, Z, SH, ZH, Y,
    W, H,
    L, R, TH, FV,
}

mod syllabics;
pub use syllabics::to_syllabics;

use Fiero::*;

impl Fiero {
    pub fn ascii(self) -> &'static str {
        match self {
            SPACE => " ", DASH => "-",
            E => "e", I => "i", II => "ii", O => "o", OO => "oo", A => "a", AA => "aa",
            P => "p", B => "b", T => "t", D => "d", K => "k", G => "g", CH => "ch", J => "j",
            M => "m", N => "n", S => "s", Z => "z", SH => "sh", ZH => "zh", Y => "y",
            W => "w", H => "h",
            L => "l", R => "r", TH => "th", FV => "f",
        }
    }

    pub fn needs_break(self, next: Fiero) -> bool {
        match self {
            E => next == E,
            I | II => next == I || next == II,
            O | OO => next == O || next == OO,
            A | AA => next == A || next == AA,
            T => next == H,
            CH => next == H,
            _ => false
        }
    }

    pub fn parse(input: &str) -> Vec<Self> {
        let word = input.to_ascii_lowercase();
        let mut out = Vec::new();
        let mut iter = word.chars().peekable();

        while let Some(x) = iter.next() {
            if let Some(y) = iter.peek() {
                let mc = match (x,y) {
                    ('i','i') => Some(II), ('o','o') => Some(OO), ('a','a') => Some(AA),
                    ('c','h') => Some(CH), ('s','h') => Some(SH), ('z','h') => Some(ZH), ('t','h') => Some(TH),
                    _ => None
                };
                if let Some(c) = mc {
                    out.push(c);
                    iter.next();
                    continue
                }
            }
            let c = match x {
                ' ' => SPACE, '-' => DASH, '=' => DASH,
                'e' => E, 'i' => I,  'o' => O,  'a' => A, 
                'p' => P, 'b' => B, 't' => T, 'd' => D, 'k' => K, 'q' => K, 'g' => G, 'c' => CH,  'j' => J,
                'm' => M, 'n' => N, 's' => S, 'z' => Z, 'y' => Y,
                'w' => W, 'h' => H,
                'l' => L, 'r' => R,  'f' => FV, 'v' => FV,
                _ => continue
            };
            out.push(c);
        }
        out
    }

    pub fn to_string(word: &[Self]) -> String {
        let mut out = String::new();
        let mut last = SPACE;
        for c in word {
            if last.needs_break(*c) {
                out.push('\'');
            }
            last = *c;
            out.push_str(c.ascii());
        }
        out
    }

    fn indel_dist(self) -> u32 {
        match self {
            DASH => 1,
            I | O | A | H | N => 500,
            _ => 1000
        }
    }

    fn mod_dist(self, other: Self) -> u32 {
        match (self, other) {
            _ if self == other => 0,
            (P,B) | (B,P) | (T,D) | (D,T) | (K,G) | (G,K) | (J,CH) | (CH,J) | 
            (S,Z) | (Z,S) | (SH,ZH) | (ZH,SH) => 100,
            (I,II) | (II,I) | (O,OO) | (OO,O) | (A,AA) | (AA,A) => 700,
            _ => 1000
        }
    }
}

pub const BASE_EDIT_DIST: u32 = 1000;

pub fn edit_distance(xs: &[Fiero], ys: &[Fiero], x_substr_y: Option<u32>) -> u32 {
    let xl = xs.len(); let yl = ys.len();

    // edge cases of indels at start of word
    let mut dists: Array2D<u32> = Array2D::filled_with(0, xl + 1, yl + 1);
    for x in 1..=xl {dists[(x,0)] = dists[(x-1,0)] + xs[x-1].indel_dist()}
    for y in 1..=yl {
        dists[(0,y)] = if let Some(tail_step) = x_substr_y {
            match ys[y-1] {
                SPACE | DASH => tail_step,
                _ => dists[(0,y-1)] + tail_step
            }
        } else {dists[(0,y-1)] + ys[y-1].indel_dist()}
    }

    // recurrence cases
    for x in 1..=xl {
        for y in 1..=yl {
            dists[(x,y)] = 
            (dists[(x-1,y-1)] + xs[x-1].mod_dist(ys[y-1]))
            .min(dists[(x,y-1)] + ys[y-1].indel_dist())
            .min(dists[(x-1,y)] + xs[x-1].indel_dist())
        }
    }

    if let Some(tail_step) = x_substr_y {
        let mut taildist = 0;
        let mut smallest = dists[(xl,yl)];
        for y in (1..=yl).rev(){
            smallest = smallest.min(dists[(xl,y)] + taildist);
            taildist = match ys[y-1] {
                SPACE | DASH => tail_step,
                _ => taildist + tail_step
            }
        }
        smallest
    } else {dists[(xl,yl)]}
}

