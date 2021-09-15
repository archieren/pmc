use std::collections::HashSet;

use crate::graph::{Error, Graph, Step};

/// Iterates edges of graph in depth-first order. To perform a depth-first
/// search, use the `depth_first` function instead.
#[derive(Debug, PartialEq)]
pub struct DepthFirst<'a, G> {
    nodes: HashSet<usize>,
    stack: Vec<(usize, usize)>,
    graph: &'a G,
}

impl<'a, G: Graph> DepthFirst<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Result<Self, Error> {
        let mut nodes = HashSet::new();
        let mut stack = Vec::new();

        for neighbor in graph.neighbors(root)? {
            stack.push((root, neighbor));
        }

        nodes.insert(root);
        stack.reverse();

        Ok(Self {
            nodes,
            stack,
            graph,
        })
    }

    pub fn into_table(self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for step in self {
            if nodes.is_empty() {
                nodes.push(step.sid);
            }

            if !step.cut {
                nodes.push(step.tid)
            }

            edges.push((step.sid, step.tid));
        }

        (nodes, edges)
    }
}

impl<'a, G> Iterator for DepthFirst<'a, G>
where
    G: Graph,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(&node) {
                    Some(Step::new(parent, node, true))
                } else {
                    let neighbors = self.graph.neighbors(node).unwrap().collect::<Vec<_>>();

                    for neighbor in neighbors.into_iter().rev() {
                        if neighbor == parent {
                            continue;
                        }

                        if self.nodes.contains(&neighbor) {
                            self.stack
                                .retain(|edge| edge.0 != neighbor && edge.1 != node);
                        }

                        self.stack.push((node, neighbor));
                    }

                    self.nodes.insert(node);

                    Some(Step::new(parent, node, false))
                }
            }
        }
    }
}
