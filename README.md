# Example function usage
```rust
    let mut graph = Graph::new();
    let v1 = graph.add_vertex(1, Some("Test vertex".to_string()));
    let v2 = graph.add_vertex(2, None);
    let v3 = graph.add_vertex(3, None);
    let v4 = graph.add_vertex(4, None);
    let v5 = graph.add_vertex(5, None);

    graph.add_edge(Some("edge 1".to_string()), &v1, &v2);
    graph.add_edge(Some("edge 2".to_string()), &v1, &v3);
    graph.add_edge(None, &v2, &v5);
    graph.add_edge(Some("edge 4".to_string()), &v3, &v3);
    graph.add_edge(Some("edge 5".to_string()), &v3, &v4);
    graph.add_edge(Some("edge 6".to_string()), &v4, &v5);
    graph.add_edge(Some("edge 7".to_string()), &v1, &v4);
    graph.bfs(Some(v1));
```
