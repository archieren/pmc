use crate::graph::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct HashMapGraph {
    indices: HashMap<usize, usize>,
    adjacency: Vec<Vec<usize>>,
    ids: Vec<usize>,
    edges: Vec<(usize, usize)>,
}

impl HashMapGraph {
    pub fn new() -> Self {
        HashMapGraph {
            indices: HashMap::new(),
            adjacency: Vec::new(),
            ids: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, id: usize) -> Result<(), Error> {
        match self.indices.entry(id) {
            Entry::Occupied(_) => return Err(Error::DuplicateId(id)),
            Entry::Vacant(entry) => {
                entry.insert(self.ids.len());
            }
        }

        self.ids.push(id);
        self.adjacency.push(vec![]);

        Ok(())
    }

    pub fn add_edge(&mut self, sid: usize, tid: usize) -> Result<(), Error> {
        let &source_index = match self.indices.get(&sid) {
            Some(index) => index,
            None => return Err(Error::UnknownId(sid)),
        };
        let &target_index = match self.indices.get(&tid) {
            Some(index) => index,
            None => return Err(Error::UnknownId(tid)),
        };

        if self.adjacency[source_index].contains(&tid) {
            return Err(Error::DuplicateEdge(sid, tid));
        }

        self.adjacency[source_index].push(tid);
        self.adjacency[target_index].push(sid);
        self.edges.push((sid, tid));

        Ok(())
    }

    fn index_for(&self, id: usize) -> Result<usize, Error> {
        match self.indices.get(&id) {
            Some(index) => Ok(*index),
            None => Err(Error::UnknownId(id)),
        }
    }
}

impl Graph for HashMapGraph {
    fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    fn order(&self) -> usize {
        self.ids.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn ids(&self) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.ids.iter().cloned())
    }

    fn neighbors(&self, id: usize) -> Result<Box<dyn Iterator<Item = usize> + '_>, Error> {
        let index = self.index_for(id)?;

        Ok(Box::new(self.adjacency[index].iter().cloned()))
    }

    fn has_id(&self, id: usize) -> bool {
        self.indices.contains_key(&id)
    }

    fn degree(&self, id: usize) -> Result<usize, Error> {
        let index = self.index_for(id)?;

        Ok(self.adjacency[index].len())
    }

    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize)> + '_> {
        Box::new(self.edges.iter().cloned())
    }

    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error> {
        let index = self.index_for(sid)?;

        if self.indices.contains_key(&tid) {
            Ok(self.adjacency[index].contains(&tid))
        } else {
            return Err(Error::UnknownId(tid));
        }
    }
}

impl TryFrom<Vec<Vec<usize>>> for HashMapGraph {
    type Error = Error;
    fn try_from(adjacency: Vec<Vec<usize>>) -> Result<Self, Self::Error> {
        let mut result = Self::new();
        for (source_node_id, neighbors) in adjacency.iter().enumerate() {
            for (index, &target_node_id) in neighbors.iter().enumerate() {
                if target_node_id >= adjacency.len() {
                    return Err(Error::UnknownId(target_node_id));
                } else if neighbors[index + 1..].contains(&target_node_id) {
                    return Err(Error::DuplicateEdge(source_node_id, target_node_id));
                } else if !adjacency[target_node_id].contains(&target_node_id) {
                    return Err(Error::MissingEdge(target_node_id, source_node_id));
                }
                if source_node_id < target_node_id {
                    result.edges.push((source_node_id, target_node_id))
                }
            }
            result.ids.push(source_node_id);
            result.indices.insert(source_node_id, source_node_id);
        }
        result.adjacency = adjacency;
        Ok(result)
    }
}

impl TryFrom<Vec<(usize, usize)>> for HashMapGraph {
    type Error = Error;

    fn try_from(edges: Vec<(usize, usize)>) -> Result<Self, Self::Error> {
        let mut result = HashMapGraph::new();

        for (source_node_id, target_node_id) in edges {
            if !result.has_id(source_node_id) {
                result.add_node(source_node_id)?;
            }

            if !result.has_id(target_node_id) {
                result.add_node(target_node_id)?;
            }

            result.add_edge(source_node_id, target_node_id)?;
        }

        Ok(result)
    }
}

impl<'a, G: Graph> TryFrom<DepthFirst<'a, G>> for HashMapGraph {
    type Error = Error;

    fn try_from(traversal: DepthFirst<'a, G>) -> Result<Self, Self::Error> {
        let mut result = HashMapGraph::new();

        for step in traversal {
            if result.is_empty() {
                result.add_node(step.sid)?;
            }

            if !step.cut {
                result.add_node(step.tid)?;
            }

            result.add_edge(step.sid, step.tid)?;
        }

        Ok(result)
    }
}

impl PartialEq for HashMapGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        } else if self.order() != other.order() {
            return false;
        }

        for id in self.ids() {
            if !other.has_id(id) {
                return false;
            }
        }

        for (sid, tid) in self.edges() {
            match other.has_edge(sid, tid) {
                Ok(result) => {
                    if !result {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }

        true
    }
}
