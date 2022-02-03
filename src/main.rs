use graph::Graph;
fn main() {
    let mut graph = Graph::new();
    let v1 = graph.add_vertex(1, Some("mogus".to_string()));
    let v2 = graph.add_vertex(2, Option::None);
    let v3 = graph.add_vertex(3, Option::None);
    let v4 = graph.add_vertex(4, Option::None);
    let v5 = graph.add_vertex(5, Option::None);

    graph.add_edge(Some("22".to_string()), &v1, &v3);
    graph.add_edge(Some("".to_string()), &v1, &v4);
    graph.add_edge(Some("".to_string()), &v2, &v5);
    graph.add_edge(Some("".to_string()), &v3, &v3);
    graph.add_edge(Some("".to_string()), &v3, &v4);
    graph.add_edge(Some("".to_string()), &v4, &v5);

    graph.delete_edge(&v1, &v2).unwrap();
    graph.delete_edge(&v1, &v2).unwrap();
}
