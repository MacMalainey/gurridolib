#[derive(PartialEq, Debug)]
enum NodeSoln {
    UNKNOWN,
    EMPTY,
    FILLED,
}

#[derive(Debug)]
pub struct Node {
    solution: NodeSoln,
}

impl Node {
    pub fn new() -> Node {
        Node {
            solution: NodeSoln::UNKNOWN,
        }
    }

    pub fn solve_filled(&mut self) {
        self.solve(true);
    }

    pub fn solve_empty(&mut self) {
        self.solve(false);
    }

    pub fn solve(&mut self, filled: bool) {
        assert!(!self.is_solved()); // Cannot solve twice

        self.solution = match filled {
            true => NodeSoln::FILLED,
            false => NodeSoln::EMPTY,
        };
    }

    pub fn is_solved(&self) -> bool {
        self.solution != NodeSoln::UNKNOWN
    }

    pub fn solution_is_filled(&self) -> bool {
        assert!(self.is_solved());
        self.solution == NodeSoln::FILLED
    }

    pub fn solution_is_empty(&self) -> bool {
        assert!(self.is_solved());
        self.solution == NodeSoln::EMPTY
    }
}
