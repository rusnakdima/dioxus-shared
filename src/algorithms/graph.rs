use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str, data: serde_json::Value) {
        if !self.nodes.iter().any(|n| n.id == node_id) {
            self.nodes.push(GraphNode {
                id: node_id.to_string(),
                data,
            });
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str, weight: f64) {
        if !self.edges.iter().any(|e| e.from == from && e.to == to) {
            self.edges.push(GraphEdge {
                from: from.to_string(),
                to: to.to_string(),
                weight,
            });
        }
    }

    pub fn dijkstra_shortest_path(&self, start: &str, end: &str) -> Option<(Vec<String>, f64)> {
        #[derive(Clone, PartialEq)]
        struct DistNode {
            dist: f64,
            node: usize,
        }

        impl Eq for DistNode {}

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl PartialOrd for DistNode {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.dist.partial_cmp(&other.dist).map(|o| o.reverse())
            }
        }

        impl Ord for DistNode {
            fn cmp(&self, other: &Self) -> Ordering {
                self.dist
                    .partial_cmp(&other.dist)
                    .expect("DistNode distances must be non-NaN (infinity or finite)")
                    .reverse()
            }
        }

        let node_ids: Vec<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
        let mut node_index: HashMap<&str, usize> = HashMap::new();
        for (i, id) in node_ids.iter().enumerate() {
            node_index.insert(*id, i);
        }

        if !node_index.contains_key(start) || !node_index.contains_key(end) {
            return None;
        }

        let n = self.nodes.len();
        let start_idx = node_index[start];
        let end_idx = node_index[end];

        let mut dist: Vec<f64> = vec![f64::INFINITY; n];
        dist[start_idx] = 0.0;

        let mut prev: Vec<Option<usize>> = vec![None; n];

        let mut pq: BinaryHeap<DistNode> = BinaryHeap::new();
        pq.push(DistNode {
            dist: 0.0,
            node: start_idx,
        });

        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
        for edge in &self.edges {
            if let (Some(&from_idx), Some(&to_idx)) = (
                node_index.get(edge.from.as_str()),
                node_index.get(edge.to.as_str()),
            ) {
                adj[from_idx].push((to_idx, edge.weight));
            }
        }

        while let Some(DistNode { dist: d, node: u }) = pq.pop() {
            if d > dist[u] {
                continue;
            }
            if u == end_idx {
                break;
            }
            for &(v, w) in &adj[u] {
                let new_dist = dist[u] + w;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    prev[v] = Some(u);
                    pq.push(DistNode {
                        dist: new_dist,
                        node: v,
                    });
                }
            }
        }

        if dist[end_idx] == f64::INFINITY {
            return None;
        }

        let mut path = Vec::new();
        let mut curr = Some(end_idx);
        while let Some(idx) = curr {
            path.push(node_ids[idx].to_string());
            curr = prev[idx];
        }
        path.reverse();

        Some((path, dist[end_idx]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_graph_new() {
        let g = Graph::new();
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    #[test]
    fn test_graph_add_node() {
        let mut g = Graph::new();
        g.add_node("A", serde_json::json!({"name": "Node A"}));
        g.add_node("B", serde_json::json!({"name": "Node B"}));
        assert_eq!(g.nodes.len(), 2);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g = Graph::new();
        g.add_node("A", serde_json::json!({}));
        g.add_node("B", serde_json::json!({}));
        g.add_edge("A", "B", 1.0);
        assert_eq!(g.edges.len(), 1);
        assert_eq!(g.edges[0].weight, 1.0);
    }

    #[test]
    fn test_dijkstra_simple() {
        let mut g = Graph::new();
        g.add_node("A", serde_json::json!({}));
        g.add_node("B", serde_json::json!({}));
        g.add_node("C", serde_json::json!({}));
        g.add_edge("A", "B", 1.0);
        g.add_edge("B", "C", 2.0);
        g.add_edge("A", "C", 5.0);

        let result = g.dijkstra_shortest_path("A", "C");
        assert!(result.is_some());
        let (path, dist) = result.unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
        assert_eq!(dist, 3.0);
    }

    #[test]
    fn test_dijkstra_no_path() {
        let mut g = Graph::new();
        g.add_node("A", serde_json::json!({}));
        g.add_node("B", serde_json::json!({}));
        assert!(g.dijkstra_shortest_path("A", "B").is_none());
    }

    #[test]
    fn test_dijkstra_same_node() {
        let mut g = Graph::new();
        g.add_node("A", serde_json::json!({}));
        let result = g.dijkstra_shortest_path("A", "A");
        assert!(result.is_some());
        let (path, dist) = result.unwrap();
        assert_eq!(path, vec!["A"]);
        assert_eq!(dist, 0.0);
    }

    proptest! {
      #[test]
      fn proptest_dijkstra_idempotent(
        node_count in 2..20usize,
        edge_count in 0..50usize,
        weights in prop::collection::vec(any::<i32>(), 0..50),
      ) {
        let mut g = Graph::new();
        let nodes: Vec<String> = (0..node_count).map(|i| format!("n{}", i)).collect();
        for id in &nodes {
          g.add_node(id, serde_json::json!({}));
        }

        for (i, weight_i) in weights.iter().take(edge_count).enumerate() {
          let from_idx = i % node_count;
          let to_idx = (i + 1) % node_count;
          if from_idx != to_idx {
            let w = (*weight_i as f64).abs().max(0.1);
            g.add_edge(&nodes[from_idx], &nodes[to_idx], w);
          }
        }

        prop_assume!(g.nodes.len() >= 2 && !g.edges.is_empty());

        let start = &nodes[0];
        let end = &nodes[node_count - 1];

        let result1 = g.dijkstra_shortest_path(start, end);
        let result2 = g.dijkstra_shortest_path(start, end);

        match (result1, result2) {
          (Some((path1, dist1)), Some((path2, dist2))) => {
            prop_assert_eq!(path1, path2);
            prop_assert_eq!(dist1, dist2);
            prop_assert!(dist1 >= 0.0);
          }
          (None, None) => {}
          _ => prop_assert!(false, "Results should be consistent across runs"),
        }
      }

      #[test]
      fn proptest_dijkstra_path_length_positive(
        node_count in 2..15usize,
        weights in prop::collection::vec(any::<i32>(), 1..14),
      ) {
        let mut g = Graph::new();
        let nodes: Vec<String> = (0..node_count).map(|i| format!("n{}", i)).collect();
        for id in &nodes {
          g.add_node(id, serde_json::json!({}));
        }

        for i in 0..node_count.saturating_sub(1) {
          let weight_i = weights.get(i).copied().unwrap_or(0);
          let w = (weight_i as f64).abs().max(0.1);
          g.add_edge(&nodes[i], &nodes[i + 1], w);
        }

        let start = &nodes[0];
        let end = &nodes[node_count - 1];

        let result = g.dijkstra_shortest_path(start, end);

        prop_assert!(result.is_some());
        let (_, dist) = result.unwrap();
        prop_assert!(dist >= 0.0, "Path distance must be non-negative, got {}", dist);
      }
    }
}
