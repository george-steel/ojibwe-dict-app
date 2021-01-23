#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum Fiero {
    SPACE, DASH,
    E, I, II, O, OO, A, AA,
    P, B, T, D, K, G, CH, J, M, N, S, Z, SH, ZH, Y,
    W, H,
    L, R, TH, FV,
}

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
}

