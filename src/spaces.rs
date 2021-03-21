pub mod hint;
pub mod node;

use hint::Hint;

pub struct Line {
    hints: Vec<Hint>,
}

impl Line {
    pub fn new(hints: &[usize], length: usize) -> Line {
        Line {
            hints: Hint::gen(hints, length),
        }
    }
}
