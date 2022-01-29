use crate::graph::{Error, Graph};
use std::collections::HashMap;

pub struct Kcores<'a, G> {
    core_numbers: HashMap<usize, usize>,
    graph: &'a G,
}

impl<'a, G: Graph> Kcores<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Kcores {
            core_numbers: HashMap::new(),
            graph: graph,
        }
    }

    pub fn cores_by_batagelj(&mut self) -> Result<(), Error> {
        let ids = self.graph.ids();
        let mut ids_index: Vec<usize> = Vec::new();
        let mut ids_degrees: Vec<usize> = Vec::new();
        for id in ids {
            match self.graph.degree(id) {
                Ok(deg) => {
                    ids_degrees.push(deg);
                    ids_index.push(id);
                    self.core_numbers.insert(id, deg);
                }
                Err(_) => return Err(Error::UnknownId(id)),
            }
        }
        Ok(())
    }
}
