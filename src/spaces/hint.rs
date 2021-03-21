use super::node::Node;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct HSoln {
    offset: usize,
    length: usize,
}

pub struct Hint {
    hint: usize,
    solutions: Vec<HSoln>,
}

struct RangeQueue {
    queue: VecDeque<(usize, usize)>,
}

impl HSoln {
    pub fn is_valid(&self, nodes: &[Node], hint: usize) -> bool {
        let nodes = self.partition(nodes);
        // TODO: Might be worthwhile to cache this value until a registered change occurs
        let mut min_filled = None;
        let mut max_filled = None;

        for (i, node) in nodes.iter().enumerate() {
            if node.is_solved() {
                if node.solution_is_empty() {
                    return false;
                } else if node.solution_is_filled() {
                    match min_filled {
                        // Distance between two filled nodes is greater than hint number
                        Some(j) if i - j >= hint => return false,
                        // Distance between first filled node and start is greater than hint number
                        None if i >= hint => return false,
                        // Set value on first pass
                        None => min_filled = Some(i),
                        // Update max_value any time else
                        _ => max_filled = Some(i),
                    };
                }
            }
        }
        match max_filled {
            Some(j) if nodes.len() - j > hint || j > hint => false,
            _ => true,
        }
    }

    fn partition<'a>(&self, nodes: &'a [Node]) -> &'a [Node] {
        &nodes[self.offset..self.offset + self.length]
    }

    pub fn split(&self, nodes: &[Node], hint: usize) -> Vec<HSoln> {
        let nodes = self.partition(nodes);
        let mut splits = Vec::new();
        // Store index of first and last node in continous filled solution group
        let mut ranges = RangeQueue::new();

        // Index of the earliest node that can be included in a split
        let mut min = 0;

        for (i, node) in nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.is_solved())
        {
            // Partition around any empty nodes
            if node.solution_is_empty() {
                if i - min > hint {
                    // If the partition is all unsolved and large enough we store it
                    if ranges.is_empty() {
                        splits.push(HSoln {
                            offset: self.offset + min,
                            length: i - min,
                        });
                    } else {
                        // Clean queue
                        let (captures, new_min) = ranges.map_and_clean(hint, min, i + 1, true);
                        min = new_min;
                        // Handle splits
                        captures.iter().for_each(|&(j, length)| {
                            splits.push(HSoln {
                                offset: self.offset + j,
                                length,
                            })
                        });
                    }
                } else if i - min == hint {
                    // Exact size, can ignore filled nodes
                    splits.push(HSoln {
                        offset: self.offset + min,
                        length: hint,
                    });
                }
                min = i + 1;
            } else if node.solution_is_filled() {
                // Filled node JUST exeeds the hint size so we move up the bumper
                if i - min == hint {
                    // Move bumper further if a filled node is at the bumper
                    match ranges.front() {
                        Some(&(j, k)) if j == min => {
                            ranges.pop();
                            min = k + 1;
                        }
                        _ => min += 1,
                    }
                } else if i - min > hint {
                    // Check if we need to clean the queue or not
                    if ranges.is_empty() {
                        splits.push(HSoln {
                            offset: self.offset + min,
                            length: min - i - 1,
                        })
                    } else {
                        // Clean queue
                        let (captures, new_min) = ranges.map_and_clean(hint, min, i, false);
                        min = new_min;
                        // Handle splits
                        captures.iter().for_each(|&(j, length)| {
                            splits.push(HSoln {
                                offset: self.offset + j,
                                length,
                            })
                        });
                    }
                }

                ranges.push(i);
            }
        }

        // Last queue cleanup
        let (captures, min) = ranges.map_and_clean(hint, min, nodes.len() + 1, true);
        captures.iter().for_each(|&(j, length)| {
            splits.push(HSoln {
                offset: self.offset + j,
                length,
            })
        });

        if nodes.len() - min >= hint {
            splits.push(HSoln {
                offset: min + self.offset,
                length: nodes.len() - min,
            });
        }

        splits
    }
}

impl Hint {
    pub fn gen(hints: &[usize], nodes: usize) -> Vec<Hint> {
        let mut offset = 0;
        let mut result = Vec::with_capacity(hints.len());
        let length = nodes - (hints.iter().map(|item| item + 1).sum::<usize>() - 1);

        for &hint in hints {
            result.push(Hint {
                hint,
                solutions: vec![HSoln {
                    offset,
                    length: length + hint,
                }],
            });
            offset += hint + 1;
        }

        result
    }
}

impl RangeQueue {
    fn new() -> RangeQueue {
        RangeQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, value: usize) {
        match self.queue.back_mut() {
            Some(i) if value == i.1 + 1 => {
                i.1 = value;
            }
            Some(i) => {
                assert!(value > i.1);
                self.queue.push_back((value, value));
            }
            None => self.queue.push_back((value, value)),
        };
    }

    fn map_and_clean(
        &mut self,
        range: usize,
        min: usize,
        max: usize,
        clean_all: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut min = min;
        let mut solutions = Vec::new();
        if max - min > range {
            while let Some(&(i, j)) = self.queue.front() {
                println!("Values are: {}, {}, {}, {}", min, max, i, j);
                // Check if we have enough space to capture a range
                if range < max - min {
                    // Check if that range is constricted or not
                    if max - i > range {
                        solutions.push((min, range + i - min))
                    } else {
                        solutions.push((min, max - 1 - min))
                    }
                }
                // Pop any values that fall outside of the new range
                if i <= max - range || clean_all {
                    self.queue.pop_front();
                }

                min = if i <= max - range { j + 2 } else { i };

                // Break if the next group is within the new range
                if min >= max - range && !clean_all {
                    break;
                }
            }
        }
        (solutions, min)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn front(&self) -> Option<&(usize, usize)> {
        self.queue.front()
    }

    fn back(&self) -> Option<&(usize, usize)> {
        self.queue.back()
    }

    fn pop(&mut self) -> Option<(usize, usize)> {
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::node::Node;

    fn check_hints(hints: &[Hint], offsets: &[usize], length: usize) {
        hints.iter().enumerate().for_each(|(i, hint)| {
            let soln = hint.solutions.get(0).unwrap();
            assert_eq!(
                soln.length,
                hint.hint + length,
                "Hint {} has incorrect length",
                i
            );
            assert_eq!(soln.offset, offsets[i], "Hint {} has incorrect offset", i);
        });
    }

    #[test]
    fn gen_two_hints() {
        check_hints(&Hint::gen(vec![2, 4], 10), &[0, 3], 3);
    }

    #[test]
    fn gen_full_hints() {
        check_hints(&Hint::gen(vec![3, 3, 2], 10), &[0, 4, 8], 0);
    }

    #[test]
    fn gen_one_hint() {
        check_hints(&Hint::gen(vec![3], 10), &[0], 7);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn gen_overflow_hint() {
        check_hints(&Hint::gen(vec![3, 7], 10), &[0, 4], 0);
    }

    fn setup_hsoln_test(size: usize, filled: &[usize], empty: &[usize]) -> (HSoln, Vec<Node>) {
        let mut nodes = Vec::with_capacity(size);
        for _ in 0..size {
            nodes.push(Node::new());
        }

        for i in filled {
            nodes.get_mut(*i).unwrap().solve_filled();
        }

        for i in empty {
            nodes.get_mut(*i).unwrap().solve_empty();
        }

        return (
            HSoln {
                offset: 0,
                length: size,
            },
            nodes,
        );
    }

    fn assert_soln(soln: &HSoln, offset: usize, length: usize) {
        assert_eq!(soln.offset, offset);
        assert_eq!(soln.length, length);
    }

    #[test]
    fn out_of_reach_node_not_valid() {
        let (soln, nodes) = setup_hsoln_test(5, &[0, 3], &[]);
        assert!(!soln.is_valid(&nodes, 3));
    }
    #[test]
    fn in_reach_node_valid() {
        let (soln, nodes) = setup_hsoln_test(5, &[0, 2], &[]);

        assert!(soln.is_valid(&nodes, 3));
    }

    #[test]
    fn empty_node_not_valid() {
        let (soln, nodes) = setup_hsoln_test(5, &[], &[3]);

        assert!(!soln.is_valid(&nodes, 3));
    }

    #[test]
    fn split_empty_nodes() {
        let (soln, nodes) = setup_hsoln_test(10, &[], &[1, 6]);

        let splits = soln.split(&nodes, 2);

        assert_eq!(splits.len(), 2);
        assert_soln(splits.get(0).unwrap(), 2, 4);
        assert_soln(splits.get(1).unwrap(), 7, 3);
    }

    #[test]
    fn split_test_a() {
        // 0FF00, h = 3
        let (soln, nodes) = setup_hsoln_test(5, &[1, 2], &[]);

        let splits = soln.split(&nodes, 3);

        assert_eq!(splits.len(), 1);
        assert_soln(splits.get(0).unwrap(), 0, 4);
    }

    #[test]
    fn split_test_b() {
        // 00FF0FF0F000, h = 4
        let (soln, nodes) = setup_hsoln_test(12, &[2, 3, 5, 6, 8], &[]);

        let splits = soln.split(&nodes, 4);

        println!("{:?}", splits);

        assert_eq!(splits.len(), 3);
        assert_soln(splits.get(0).unwrap(), 0, 4);
        assert_soln(splits.get(1).unwrap(), 5, 4);
        assert_soln(splits.get(2).unwrap(), 8, 4);
    }

    #[test]
    fn split_test_c() {
        // 00F0F0F0F000, h = 5
        let (soln, nodes) = setup_hsoln_test(12, &[2, 4, 6, 8], &[]);

        let splits = soln.split(&nodes, 5);

        println!("{:?}", splits);

        assert_eq!(splits.len(), 4);
        assert_soln(splits.get(0).unwrap(), 0, 5);
        assert_soln(splits.get(1).unwrap(), 2, 5);
        assert_soln(splits.get(2).unwrap(), 4, 5);
        assert_soln(splits.get(3).unwrap(), 6, 5);
    }

    #[test]
    fn split_test_d() {
        // 0FF0F0F0F000, h = 5
        let (soln, nodes) = setup_hsoln_test(12, &[1, 2, 4, 6, 8], &[]);

        let splits = soln.split(&nodes, 5);

        println!("{:?}", splits);

        assert_eq!(splits.len(), 3);
        assert_soln(splits.get(0).unwrap(), 0, 5);
        assert_soln(splits.get(1).unwrap(), 4, 5);
        assert_soln(splits.get(2).unwrap(), 6, 5);
    }

    #[test]
    fn split_test_e() {
        // F000F0F0F, h = 5
        let (soln, nodes) = setup_hsoln_test(10, &[0, 4, 6, 8], &[]);

        let splits = soln.split(&nodes, 5);

        println!("{:?}", splits);

        assert_eq!(splits.len(), 3);
        assert_soln(splits.get(0).unwrap(), 0, 5);
        assert_soln(splits.get(1).unwrap(), 2, 5);
        assert_soln(splits.get(2).unwrap(), 4, 5);
    }
}
