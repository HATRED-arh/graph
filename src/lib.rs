use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::hash::Hash;
use std::io::{Error, ErrorKind, Result, Write};
use std::option::Option;
use std::str::SplitAsciiWhitespace;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug)]
struct OpVal<V: PartialEq + Display + Hash + Clone>(Option<V>);

impl<V: PartialEq + Display + Hash + Clone> PartialEq for OpVal<V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<V: PartialEq + Display + Hash + Clone> std::fmt::Display for OpVal<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                Some(val) => val.to_string(),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Vertex<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    id: I,
    value: OpVal<V>,
    edges: Vec<Rc<RefCell<Edge<E, I, V>>>>,
}
impl<E, I, V> Vertex<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    pub fn change_value(&mut self, val: Option<V>) {
        self.value = match val {
            Some(val) => OpVal(Some(val)),
            None => OpVal(Option::None),
        }
    }
}

#[derive(Debug)]
struct Edge<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    edge_value: Rc<Option<E>>,
    child: Weak<RefCell<Vertex<E, I, V>>>,
}
impl<E, I, V> PartialEq for Vertex<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

type RcVertex<E, I, V> = Rc<RefCell<Vertex<E, I, V>>>;

#[derive(Debug)]
pub struct Graph<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    vertices: Vec<RcVertex<E, I, V>>,
}

impl<E, I, V> Default for Graph<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E, I, V> Graph<E, I, V>
where
    E: Display + Clone,
    I: PartialEq + Display + Hash + Clone,
    V: PartialEq + Display + Hash + Clone,
{
    pub fn new() -> Graph<E, I, V> {
        Graph { vertices: vec![] }
    }
    pub fn add_vertex(&mut self, id: I, value: Option<V>) -> RcVertex<E, I, V> {
        let v = Rc::new(RefCell::new(Vertex {
            id,
            value: OpVal(value),
            edges: vec![],
        }));
        self.vertices.push(Rc::clone(&v));
        v
    }
    pub fn delete_vertex(&mut self, vertex: RcVertex<E, I, V>) {
        match self.vertices.iter().position(|v1| *v1 == vertex) {
            Some(pos) => {
                self.vertices.remove(pos);
            }
            None => println!("Couldn't find vertex :/"),
        };
    }
    pub fn add_edge(&self, edge_value: Option<E>, v1: &RcVertex<E, I, V>, v2: &RcVertex<E, I, V>) {
        //  since edge is connected to both points, they should share same value
        let edge_value = Rc::new(edge_value);
        v1.as_ref()
            .borrow_mut()
            .edges
            .push(Rc::new(RefCell::new(Edge {
                edge_value: Rc::clone(&edge_value),
                child: Rc::downgrade(v2),
            })));
        v2.as_ref()
            .borrow_mut()
            .edges
            .push(Rc::new(RefCell::new(Edge {
                edge_value: Rc::clone(&edge_value),
                child: Rc::downgrade(v1),
            })));
    }
    fn check_edge(&self, v1: &RcVertex<E, I, V>, v2: &RcVertex<E, I, V>) -> Result<usize> {
        let pos = match v1.as_ref().borrow().edges.iter().position(|edge| {
            match edge.as_ref().borrow().child.upgrade() {
                Some(child) => child == *v2,
                None => false,
            }
        }) {
            Some(u) => u,
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "{} in not linked to {}",
                        v1.as_ref().borrow().id,
                        v2.as_ref().borrow().id,
                    ),
                ))
            }
        };
        Ok(pos)
    }
    // god help me with this
    pub fn delete_edge(&self, v1: &RcVertex<E, I, V>, v2: &RcVertex<E, I, V>) -> Result<()> {
        let pos1 = self.check_edge(v1, v2)?;
        let pos2 = self.check_edge(v2, v1)?;
        v1.as_ref().borrow_mut().edges.remove(pos1);
        v2.as_ref().borrow_mut().edges.remove(pos2);
        Ok(())
    }

    pub fn bfs(&self, start: Option<RcVertex<E, I, V>>) -> Result<()> {
        let mut queue: Vec<RcVertex<E, I, V>> = vec![];
        match start {
            Some(v) => queue.push(v),
            None => match self.vertices.get(0) {
                Some(v) => queue.push(Rc::clone(v)),
                None => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Start point wasn't provided and graph is empty. Returning.",
                    ));
                }
            },
        }
        let mut i = 0;
        while i < queue.len() {
            let vertex = Rc::clone(&queue[i]);
            i += 1;
            println!(
                "{} {}",
                &vertex.as_ref().borrow().id,
                &vertex.as_ref().borrow().value
            );
            for edge in &vertex.as_ref().borrow().edges {
                if let Some(child) = edge.as_ref().borrow().child.upgrade() {
                    if queue.iter().all(|node| *node != child) {
                        queue.push(child)
                    }
                }
            }
        }
        Ok(())
    }

    pub fn write_to_file(&self, filename: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .unwrap();

        let mut edges_collection: HashMap<(String, String), String> = HashMap::new();
        for node in self.vertices.iter() {
            let node_id = &node.as_ref().borrow().id.to_string();
            let point_desc = format!("{} {}\n", &node_id, &node.as_ref().borrow().value);
            file.write_all(point_desc.as_bytes())?;
            for edge in &node.as_ref().borrow().edges {
                let id = match &edge.as_ref().borrow().child.upgrade() {
                    Some(child) => child.as_ref().borrow().id.clone(),
                    None => {
                        return Err(Error::new(
                            ErrorKind::NotFound,
                            format!("Second poit of the edge {} not found.", &node_id),
                        ))
                    }
                }
                .to_string();
                // our edges are bidirectional and we have to check for inverted duplicates
                if edges_collection.contains_key(&(id.to_string(), node_id.clone())) {
                    continue;
                } else {
                    edges_collection.insert(
                        (node_id.clone(), id),
                        match edge.as_ref().borrow().edge_value.borrow() {
                            Some(val) => val.to_string(),
                            None => "".to_string(),
                        },
                    );
                }
            }
        }
        file.write_all(b"#\n")?;
        for edge in &edges_collection {
            file.write_all(format!("{} {} {}\n", edge.0 .0, edge.0 .1, edge.1).as_bytes())?;
        }
        Ok(())
    }
}
impl Graph<String, String, String> {
    pub fn create_from_file(filename: &str) -> Graph<String, String, String> {
        let mut graph = Graph::new();
        let data = fs::read_to_string(filename).unwrap();
        let mut split = data.split('#');

        let (points, edges) = (
            split.next().expect("Failed to parse vertices."),
            split.next().expect("Failed to parse edges."),
        );
        let mut data;
        let mut point_storage: HashMap<&str, RcVertex<String, String, String>> = HashMap::new();
        for point in points.lines() {
            data = point.trim().split_ascii_whitespace();
            let point = data.next().expect("Failet to parse point.");
            let value: Option<String> = extract_value(data);
            let v = graph.add_vertex(point.to_string(), value);
            point_storage.insert(point, v);
        }
        for edge in edges.lines().skip(1) {
            data = edge.trim().split_ascii_whitespace();
            let (point1, point2) = (
                data.next().expect("Falide to parse first point"),
                data.next().expect("Failed to parse second point"),
            );
            let value = extract_value(data);
            graph.add_edge(
                value,
                point_storage.get(point1).unwrap_or_else(|| {
                    panic!("Failed to add edge. Point {} does not exist", point1)
                }),
                point_storage.get(point2).unwrap_or_else(|| {
                    panic!("Failed to add edge. Point {} does not exist", point2)
                }),
            );
        }
        graph
    }
}
fn extract_value(data: SplitAsciiWhitespace) -> Option<String> {
    match data
        .map(|val| val.to_string() + " ")
        .collect::<String>()
        .trim()
    {
        val if val.is_empty() => Option::None,
        val => Some(val.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_vertex() {
        let mut graph: Graph<&str, u32, &str> = Graph::new();
        graph.add_vertex(1, Some("vertex 1"));
        graph.add_vertex(2, Some("vertex 2"));

        assert!(graph.vertices.len() == 2);
    }
    #[test]
    fn add_edge() {
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(1, Some("vertex 1"));
        let v2 = graph.add_vertex(2, Some("vertex 2"));
        graph.add_edge(Some("edge between v1 and v2"), &v1, &v2);
        assert!(v1.as_ref().borrow().edges.len() == 1);
        assert!(v2.as_ref().borrow().edges.len() == 1);
    }

    #[test]
    fn write_graph_to_file() {
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(1, Some("vertex 1"));
        let v2 = graph.add_vertex(2, Some("vertex 2"));
        graph.add_edge(Some("edge between v1 and v2"), &v1, &v2);

        graph.write_to_file("test_graph.tgf").unwrap();
    }

    #[test]
    fn delete_edge() {
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(1, Some("vertex 1"));
        let v2 = graph.add_vertex(2, Some("vertex 2"));
        graph.add_edge(Some("edge between v1 and v2"), &v1, &v2);
        graph.delete_edge(&v1, &v2).unwrap();
    }
    #[test]
    fn delete_missing_edge() {
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(1, Some("vertex 1"));
        let v2 = graph.add_vertex(2, Some("vertex 2"));
        graph.add_edge(Some("edge between v1 and v2"), &v1, &v2);
        graph.delete_edge(&v1, &v2).unwrap();
        assert!(graph.delete_edge(&v1, &v2).is_err());
    }
}
