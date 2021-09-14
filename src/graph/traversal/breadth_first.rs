use std::collections::HashSet;
use std::collections::VecDeque;

use crate::{Error, Graph, Step};

/// Implements a breadth-first traversal as a Step Iterator.

/// #[derive(Debug,PartialEq)]
pub struct BreadthFirst<'a, G> {
    nodes: HashSet<usize>,
    queue: VecDeque<(usize, usize)>,
    graph: &'a G,
}

impl<'a, G: Graph> BreadthFirst<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Result<Self, Error> {
        let mut nodes = HashSet::new();
        let mut queue = VecDeque::new();

        for neighbor in graph.neighbors(root)? {
            queue.push_front((root, neighbor));
        }

        nodes.insert(root);

        Ok(Self {
            nodes,
            queue,
            graph,
        })
    }
}

impl<'a, G> Iterator for BreadthFirst<'a, G>
where
    G: Graph,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_back() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(&node) {
                    Some(Step::new(parent, node, true))
                } else {
                    for neighbor in self.graph.neighbors(node).unwrap() {
                        if neighbor == parent || self.nodes.contains(&neighbor) {
                            continue;
                        }

                        self.queue.push_front((node, neighbor));
                    }

                    self.nodes.insert(node);

                    Some(Step::new(parent, node, false))
                }
            }
        }
    }
}
