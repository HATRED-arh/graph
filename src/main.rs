use graph::Graph;
fn main() {
    let graph = Graph::create_from_file("graph.tgf");

    graph.bfs(Option::None).unwrap();
}
