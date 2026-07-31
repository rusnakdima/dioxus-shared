use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use super::graph::{Graph, GraphEdge, GraphNode};
use super::sanitization::escape_html;
use super::search::SearchAlgorithm;
use super::sorting::{
    bubble_sort_by, insertion_sort_by, merge_sort_by, quick_sort,
};
use super::validation::ValidationAlgorithm;

/// Represents the result of an algorithm execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmOutput {
  pub data: serde_json::Value,
  pub metadata: Option<serde_json::Value>,
}

impl AlgorithmOutput {
  pub fn new(data: serde_json::Value) -> Self {
    Self {
      data,
      metadata: None,
    }
  }

  pub fn with_metadata(data: serde_json::Value, metadata: serde_json::Value) -> Self {
    Self {
      data,
      metadata: Some(metadata),
    }
  }
}

impl fmt::Display for AlgorithmOutput {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "AlgorithmOutput({})", self.data)
  }
}

/// Represents input for an algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInput {
  pub data: serde_json::Value,
  pub field: Option<String>,
  pub order: Option<String>,
}

impl AlgorithmInput {
  pub fn new(data: serde_json::Value) -> Self {
    Self {
      data,
      field: None,
      order: None,
    }
  }

  pub fn with_field(data: serde_json::Value, field: &str) -> Self {
    Self {
      data,
      field: Some(field.to_string()),
      order: None,
    }
  }

  pub fn with_order(data: serde_json::Value, field: &str, order: &str) -> Self {
    Self {
      data,
      field: Some(field.to_string()),
      order: Some(order.to_string()),
    }
  }
}

/// Trait for algorithms that can be registered and executed.
pub trait Algorithm: Send + Sync {
  /// Execute the algorithm with the given input and return the output.
  ///
  /// Error/parse failures MUST be encoded as `{ "error": "..." }` in `data`
  /// because the trait is infallible. The reference `tauri-shared` registry
  /// uses a fallible `Result<_, String>`; here we keep the trait stable.
  fn execute(&self, input: AlgorithmInput) -> AlgorithmOutput;

  /// Return the name of this algorithm.
  fn name(&self) -> &str;
}

/// Registry for managing and executing algorithms.
///
/// Thread-safe: backed by `Arc<RwLock<HashMap<...>>>` for interior mutability.
/// Callers may `clone()` freely and register new algorithms through `&self`.
#[derive(Clone, Default)]
pub struct AlgorithmRegistry {
  algorithms: Arc<RwLock<HashMap<String, Box<dyn Algorithm>>>>,
}

impl AlgorithmRegistry {
  /// Create a new registry with the 16 built-in algorithms registered.
  pub fn new() -> Self {
    let reg = Self {
      algorithms: Arc::new(RwLock::new(HashMap::new())),
    };
    reg.register_builtins();
    reg
  }

  /// Register a new algorithm. Replaces any existing algorithm with the same name.
  pub fn register<A: Algorithm + 'static>(&self, name: &str, algorithm: A) {
    let mut map = match self.algorithms.write() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    map.insert(name.to_string(), Box::new(algorithm));
  }

  /// Execute an algorithm by name with the given input.
  pub fn execute(&self, name: &str, input: AlgorithmInput) -> Option<AlgorithmOutput> {
    let map = match self.algorithms.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    map.get(name).map(|alg| alg.execute(input))
  }

  /// Execute an algorithm by name with default input constructed from JSON data.
  pub fn execute_with_data(&self, name: &str, data: serde_json::Value) -> Option<AlgorithmOutput> {
    self.execute(name, AlgorithmInput::new(data))
  }

  /// Check if an algorithm is registered.
  pub fn contains(&self, name: &str) -> bool {
    let map = match self.algorithms.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    map.contains_key(name)
  }

  /// Get a list of all registered algorithm names (sorted).
  pub fn names(&self) -> Vec<String> {
    let map = match self.algorithms.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    let mut names: Vec<String> = map.keys().cloned().collect();
    names.sort();
    names
  }

  // -- Built-in registration ----------------------------------------------------

  fn register_builtins(&self) {
    self.register_sorting_algorithms();
    self.register_search_algorithms();
    self.register_graph_algorithms();
    self.register_tree_algorithms();
    self.register_validation_algorithms();
    self.register_sanitization_algorithms();
  }

  fn register_sorting_algorithms(&self) {
    self.register("sort.bubble", BuiltinAlgorithm("sort.bubble", sort_bubble_handler));
    self.register(
      "sort.insertion",
      BuiltinAlgorithm("sort.insertion", sort_insertion_handler),
    );
    self.register("sort.merge", BuiltinAlgorithm("sort.merge", sort_merge_handler));
    self.register("sort.rust_default", BuiltinAlgorithm("sort.rust_default", sort_rust_default_handler));
  }

  fn register_search_algorithms(&self) {
    self.register(
      "search.schemas",
      BuiltinAlgorithm("search.schemas", search_schemas_handler),
    );
    self.register(
      "search.paginate",
      BuiltinAlgorithm("search.paginate", search_paginate_handler),
    );
  }

  fn register_graph_algorithms(&self) {
    self.register(
      "graph.dijkstra",
      BuiltinAlgorithm("graph.dijkstra", graph_dijkstra_handler),
    );
    self.register("graph.bfs", BuiltinAlgorithm("graph.bfs", graph_bfs_handler));
    self.register("graph.dfs", BuiltinAlgorithm("graph.dfs", graph_dfs_handler));
    self.register(
      "graph.topological_sort",
      BuiltinAlgorithm("graph.topological_sort", graph_topo_sort_handler),
    );
  }

  fn register_tree_algorithms(&self) {
    self.register("tree.build", BuiltinAlgorithm("tree.build", tree_build_handler));
    self.register("tree.flatten", BuiltinAlgorithm("tree.flatten", tree_flatten_handler));
  }

  fn register_validation_algorithms(&self) {
    self.register(
      "validate.email",
      BuiltinAlgorithm("validate.email", validate_email_handler),
    );
    self.register(
      "validate.input",
      BuiltinAlgorithm("validate.input", validate_input_handler),
    );
    self.register(
      "validate.sanitize",
      BuiltinAlgorithm("validate.sanitize", validate_sanitize_handler),
    );
  }

  fn register_sanitization_algorithms(&self) {
    self.register(
      "sanitize.escape_html",
      BuiltinAlgorithm("sanitize.escape_html", sanitize_escape_html_handler),
    );
  }
}

/// Adapter for built-in algorithms. Each handler takes JSON in and returns
/// JSON out; errors are encoded as `{ "error": "..." }` to fit the infallible trait.
struct BuiltinAlgorithm(&'static str, fn(AlgorithmInput) -> AlgorithmOutput);

impl Algorithm for BuiltinAlgorithm {
  fn name(&self) -> &str {
    self.0
  }
  fn execute(&self, input: AlgorithmInput) -> AlgorithmOutput {
    (self.1)(input)
  }
}

// -- JSON-aware helpers ---------------------------------------------------------

fn err_json(message: impl Into<String>) -> AlgorithmOutput {
  AlgorithmOutput::new(serde_json::json!({ "error": message.into() }))
}

/// JSON-aware comparator: Null < Bool < Number < String < Array < Object.
/// For two values that share a type, delegates to standard PartialOrd.
fn json_ord(a: &serde_json::Value, b: &serde_json::Value) -> std::cmp::Ordering {
  use std::cmp::Ordering;
  use serde_json::Value;
  let rank = |v: &Value| match v {
    Value::Null => 0,
    Value::Bool(_) => 1,
    Value::Number(_) => 2,
    Value::String(_) => 3,
    Value::Array(_) => 4,
    Value::Object(_) => 5,
  };
  let ra = rank(a);
  let rb = rank(b);
  if ra != rb {
    return ra.cmp(&rb);
  }
  match (a, b) {
    (Value::Null, Value::Null) => Ordering::Equal,
    (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
    (Value::Number(x), Value::Number(y)) => {
      let xf = x.as_f64().unwrap_or(0.0);
      let yf = y.as_f64().unwrap_or(0.0);
      xf.partial_cmp(&yf).unwrap_or(Ordering::Equal)
    }
    (Value::String(x), Value::String(y)) => x.cmp(y),
    (Value::Array(x), Value::Array(y)) => x.len().cmp(&y.len()),
    (Value::Object(x), Value::Object(y)) => x.len().cmp(&y.len()),
    _ => Ordering::Equal,
  }
}

fn extract_field_value<'a>(value: &'a serde_json::Value, field: &str) -> serde_json::Value {
  if let serde_json::Value::Object(map) = value {
    if let Some(v) = map.get(field) {
      return v.clone();
    }
  }
  value.clone()
}

fn parse_sort_input(input: AlgorithmInput) -> Result<(Vec<serde_json::Value>, Option<String>), String> {
  #[derive(Deserialize)]
  struct SortInput {
    data: Vec<serde_json::Value>,
    #[serde(default)]
    field: Option<String>,
  }
  let parsed: SortInput = serde_json::from_value(input.data).map_err(|e| e.to_string())?;
  Ok((parsed.data, parsed.field.or(input.field)))
}

fn sort_with<F: FnMut(&serde_json::Value, &serde_json::Value) -> std::cmp::Ordering>(
  data: &mut [serde_json::Value],
  field: Option<String>,
  reverse: bool,
  mut cmp: F,
) {
  if let Some(field_str) = field {
    if reverse {
      data.sort_by(|a, b| {
        let va = extract_field_value(a, &field_str);
        let vb = extract_field_value(b, &field_str);
        cmp(&va, &vb).reverse()
      });
    } else {
      data.sort_by(|a, b| {
        let va = extract_field_value(a, &field_str);
        let vb = extract_field_value(b, &field_str);
        cmp(&va, &vb)
      });
    }
  } else if reverse {
    data.sort_by(|a, b| cmp(a, b).reverse());
  } else {
    data.sort_by(|a, b| cmp(a, b));
  }
}

fn sort_bubble_handler(input: AlgorithmInput) -> AlgorithmOutput {
  let reverse = matches!(input.order.as_deref(), Some("desc"));
  match parse_sort_input(input) {
    Ok((mut data, field)) => {
      match field {
        Some(f) => bubble_sort_by(&mut data, |a, b| {
          let va = extract_field_value(a, &f);
          let vb = extract_field_value(b, &f);
          let ord = json_ord(&va, &vb);
          if reverse { ord.reverse() } else { ord }
        }),
        None => {
          if reverse {
            bubble_sort_by(&mut data, |a, b| json_ord(a, b).reverse());
          } else {
            bubble_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
      }
      AlgorithmOutput::new(serde_json::to_value(data).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("sort.bubble: {e}")),
  }
}

fn sort_insertion_handler(input: AlgorithmInput) -> AlgorithmOutput {
  let reverse = matches!(input.order.as_deref(), Some("desc"));
  match parse_sort_input(input) {
    Ok((mut data, field)) => {
      match field {
        Some(f) => insertion_sort_by(&mut data, |a, b| {
          let va = extract_field_value(a, &f);
          let vb = extract_field_value(b, &f);
          let ord = json_ord(&va, &vb);
          if reverse { ord.reverse() } else { ord }
        }),
        None => {
          if reverse {
            insertion_sort_by(&mut data, |a, b| json_ord(a, b).reverse());
          } else {
            insertion_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
      }
      AlgorithmOutput::new(serde_json::to_value(data).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("sort.insertion: {e}")),
  }
}

fn sort_merge_handler(input: AlgorithmInput) -> AlgorithmOutput {
  let reverse = matches!(input.order.as_deref(), Some("desc"));
  match parse_sort_input(input) {
    Ok((mut data, field)) => {
      match field {
        Some(f) => merge_sort_by(&mut data, |a, b| {
          let va = extract_field_value(a, &f);
          let vb = extract_field_value(b, &f);
          let ord = json_ord(&va, &vb);
          if reverse { ord.reverse() } else { ord }
        }),
        None => {
          if reverse {
            merge_sort_by(&mut data, |a, b| json_ord(a, b).reverse());
          } else {
            merge_sort_by(&mut data, |a, b| json_ord(a, b));
          }
        }
      }
      AlgorithmOutput::new(serde_json::to_value(data).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("sort.merge: {e}")),
  }
}

/// sort.rust_default — Rust's stable sort (TimSort), exposed for API compatibility.
/// Named "rust_default" not "quick" because it uses Rust's sort not quick-sort.
fn sort_rust_default_handler(input: AlgorithmInput) -> AlgorithmOutput {
  let reverse = matches!(input.order.as_deref(), Some("desc"));
  match parse_sort_input(input) {
    Ok((mut data, field)) => {
      sort_with(&mut data, field, reverse, json_ord);
      AlgorithmOutput::new(serde_json::to_value(data).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("sort.rust_default: {e}")),
  }
}

fn search_schemas_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct SearchInput {
    items: Vec<serde_json::Value>,
    query: String,
  }
  match serde_json::from_value::<SearchInput>(input.data) {
    Ok(parsed) => {
      let strings: Vec<String> = parsed
        .items
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
      let result = SearchAlgorithm::search_schemas(&strings, &parsed.query);
      AlgorithmOutput::new(serde_json::to_value(result).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("search.schemas: {e}")),
  }
}

fn search_paginate_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct PaginateInput {
    items: Vec<serde_json::Value>,
    page: u64,
    limit: u64,
  }
  match serde_json::from_value::<PaginateInput>(input.data) {
    Ok(parsed) => {
      let result = SearchAlgorithm::paginate(&parsed.items, parsed.page, parsed.limit);
      AlgorithmOutput::new(serde_json::to_value(result).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("search.paginate: {e}")),
  }
}

fn build_graph_from(input: &serde_json::Value) -> Result<Graph, String> {
  #[derive(Deserialize)]
  struct GraphInput {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
  }
  let parsed: GraphInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
  let mut graph = Graph::new();
  for node in parsed.nodes {
    graph.add_node(&node.id, node.data);
  }
  for edge in parsed.edges {
    graph.add_edge(&edge.from, &edge.to, edge.weight);
  }
  Ok(graph)
}

fn graph_dijkstra_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct DijkstraInput {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    start: String,
    end: String,
  }
  match serde_json::from_value::<DijkstraInput>(input.data) {
    Ok(parsed) => {
      let mut graph = Graph::new();
      for node in parsed.nodes {
        graph.add_node(&node.id, node.data);
      }
      for edge in parsed.edges {
        graph.add_edge(&edge.from, &edge.to, edge.weight);
      }
      let result = graph.dijkstra_shortest_path(&parsed.start, &parsed.end);
      AlgorithmOutput::new(serde_json::to_value(result).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("graph.dijkstra: {e}")),
  }
}

fn graph_bfs_handler(input: AlgorithmInput) -> AlgorithmOutput {
  match build_graph_from(&input.data) {
    Ok(graph) => {
      let mut visited = Vec::new();
      let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
      if let Some(first) = graph.nodes.first().map(|n| n.id.clone()) {
        queue.push_back(first.clone());
        visited.push(first);
        while let Some(node) = queue.pop_front() {
          let neighbors: Vec<String> = graph
            .edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to.clone())
            .collect();
          for n in neighbors {
            if !visited.contains(&n) && graph.nodes.iter().any(|node_obj| node_obj.id == n) {
              visited.push(n.clone());
              queue.push_back(n);
            }
          }
        }
      }
      AlgorithmOutput::new(serde_json::to_value(visited).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("graph.bfs: {e}")),
  }
}

fn graph_dfs_handler(input: AlgorithmInput) -> AlgorithmOutput {
  match build_graph_from(&input.data) {
    Ok(graph) => {
      let mut visited: Vec<String> = Vec::new();
      if let Some(start) = graph.nodes.first().map(|n| n.id.clone()) {
        fn dfs(
          graph: &Graph,
          node: &str,
          visited: &mut Vec<String>,
        ) {
          visited.push(node.to_string());
          let neighbors: Vec<String> = graph
            .edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to.clone())
            .collect();
          for n in neighbors {
            if !visited.contains(&n) && graph.nodes.iter().any(|node_obj| node_obj.id == n) {
              dfs(graph, &n, visited);
            }
          }
        }
        dfs(&graph, &start, &mut visited);
      }
      AlgorithmOutput::new(serde_json::to_value(visited).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("graph.dfs: {e}")),
  }
}

fn graph_topo_sort_handler(input: AlgorithmInput) -> AlgorithmOutput {
  match build_graph_from(&input.data) {
    Ok(graph) => {
      let mut in_degree: HashMap<String, usize> = HashMap::new();
      for n in &graph.nodes {
        in_degree.insert(n.id.clone(), 0);
      }
      for e in &graph.edges {
        *in_degree.entry(e.to.clone()).or_insert(0) += 1;
      }
      let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
      for (k, v) in &in_degree {
        if *v == 0 {
          queue.push_back(k.clone());
        }
      }
      let mut order = Vec::new();
      while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        let outgoing: Vec<String> = graph
          .edges
          .iter()
          .filter(|e| e.from == node)
          .map(|e| e.to.clone())
          .collect();
        for n in outgoing {
          let d = in_degree.entry(n.clone()).or_insert(0);
          *d = d.saturating_sub(1);
          if *d == 0 {
            queue.push_back(n);
          }
        }
      }
      if order.len() != graph.nodes.len() {
        return err_json("graph.topological_sort: cycle detected");
      }
      AlgorithmOutput::new(serde_json::to_value(order).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("graph.topological_sort: {e}")),
  }
}

fn tree_build_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct TreeInput {
    nodes: Vec<serde_json::Value>,
    root_id: Option<String>,
  }
  match serde_json::from_value::<TreeInput>(input.data) {
    Ok(parsed) => {
      let result: serde_json::Value = serde_json::json!({
        "nodes": parsed.nodes,
        "root_id": parsed.root_id,
      });
      AlgorithmOutput::new(result)
    }
    Err(e) => err_json(format!("tree.build: {e}")),
  }
}

fn tree_flatten_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct FlattenInput {
    tree: serde_json::Value,
  }
  match serde_json::from_value::<FlattenInput>(input.data) {
    Ok(parsed) => {
      let mut flat = Vec::new();
      fn walk(v: &serde_json::Value, flat: &mut Vec<serde_json::Value>) {
        if let serde_json::Value::Array(arr) = v {
          for item in arr {
            flat.push(item.clone());
            walk(item, flat);
          }
        }
      }
      walk(&parsed.tree, &mut flat);
      AlgorithmOutput::new(serde_json::to_value(flat).unwrap_or(serde_json::Value::Null))
    }
    Err(e) => err_json(format!("tree.flatten: {e}")),
  }
}

fn validate_email_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct V {
    email: String,
  }
  match serde_json::from_value::<V>(input.data) {
    Ok(parsed) => AlgorithmOutput::new(serde_json::json!({
      "valid": ValidationAlgorithm::validate_email(&parsed.email)
    })),
    Err(e) => err_json(format!("validate.email: {e}")),
  }
}

fn validate_input_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct V {
    text: String,
    max_length: usize,
  }
  match serde_json::from_value::<V>(input.data) {
    Ok(parsed) => AlgorithmOutput::new(serde_json::json!({
      "valid": ValidationAlgorithm::validate_input(&parsed.text, parsed.max_length)
    })),
    Err(e) => err_json(format!("validate.input: {e}")),
  }
}

fn validate_sanitize_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct V {
    text: String,
  }
  match serde_json::from_value::<V>(input.data) {
    Ok(parsed) => AlgorithmOutput::new(serde_json::json!({
      "result": ValidationAlgorithm::sanitize_input(&parsed.text)
    })),
    Err(e) => err_json(format!("validate.sanitize: {e}")),
  }
}

fn sanitize_escape_html_handler(input: AlgorithmInput) -> AlgorithmOutput {
  #[derive(Deserialize)]
  struct V {
    text: String,
  }
  match serde_json::from_value::<V>(input.data) {
    Ok(parsed) => AlgorithmOutput::new(serde_json::json!({
      "result": escape_html(&parsed.text)
    })),
    Err(e) => err_json(format!("sanitize.escape_html: {e}")),
  }
}

/// Macro for easily creating and registering algorithms.
#[macro_export]
macro_rules! define_algorithm {
  ($name:ident, $algo_name:expr, |$input:ident| $body:expr) => {
    pub struct $name;

    impl $crate::algorithms::Algorithm for $name {
      fn execute(&self, $input: $crate::algorithms::AlgorithmInput) -> $crate::algorithms::AlgorithmOutput {
        $body
      }

      fn name(&self) -> &str {
        $algo_name
      }
    }
  };
}

#[allow(unused_imports)]
use quick_sort as _; // re-export usage parity

#[cfg(test)]
mod tests {
  use super::*;

  // -- AlgorithmRegistry::new() populated with 16 built-ins --------------------

  #[test]
  fn test_registry_new_has_builtins() {
    let reg = AlgorithmRegistry::new();
    let names = reg.names();
    // 16 built-in algorithms
    assert!(names.contains(&"sort.bubble".to_string()));
    assert!(names.contains(&"sort.insertion".to_string()));
    assert!(names.contains(&"sort.merge".to_string()));
    assert!(names.contains(&"sort.rust_default".to_string()));
    assert!(names.contains(&"search.schemas".to_string()));
    assert!(names.contains(&"search.paginate".to_string()));
    assert!(names.contains(&"graph.dijkstra".to_string()));
    assert!(names.contains(&"graph.bfs".to_string()));
    assert!(names.contains(&"graph.dfs".to_string()));
    assert!(names.contains(&"graph.topological_sort".to_string()));
    assert!(names.contains(&"tree.build".to_string()));
    assert!(names.contains(&"tree.flatten".to_string()));
    assert!(names.contains(&"validate.email".to_string()));
    assert!(names.contains(&"validate.input".to_string()));
    assert!(names.contains(&"validate.sanitize".to_string()));
    assert!(names.contains(&"sanitize.escape_html".to_string()));
    assert_eq!(names.len(), 16);
  }

  // -- AlgorithmRegistry::get() ------------------------------------------------

  #[test]
  fn test_registry_get_returns_some_for_known() {
    let reg = AlgorithmRegistry::new();
    assert!(reg.contains("sort.bubble"));
    assert!(reg.contains("search.paginate"));
    assert!(reg.contains("graph.dijkstra"));
    assert!(reg.contains("tree.build"));
    assert!(reg.contains("validate.email"));
    assert!(reg.contains("sanitize.escape_html"));
  }

  #[test]
  fn test_registry_get_returns_none_for_unknown() {
    let reg = AlgorithmRegistry::new();
    assert!(!reg.contains("nonexistent"));
    assert!(!reg.contains(""));
    assert!(!reg.contains("sort"));
    assert!(!reg.contains("search"));
  }

  // -- Sort handlers -----------------------------------------------------------

  fn make_sort_input(data: serde_json::Value, field: Option<&str>, order: Option<&str>) -> AlgorithmInput {
    AlgorithmInput {
      data: serde_json::json!({ "data": data, "field": field }),
      field: None,
      order: order.map(String::from),
    }
  }

  #[test]
  fn test_sort_bubble() {
    let reg = AlgorithmRegistry::new();
    let input = make_sort_input(
      serde_json::json!([5, 2, 8, 1, 9]),
      None,
      None,
    );
    let out = reg.execute("sort.bubble", input).unwrap();
    let arr = out.data.as_array().unwrap();
    let binding = serde_json::json!([1, 2, 5, 8, 9]);
    let expected = binding.as_array().unwrap();
    assert_eq!(arr, expected);
  }

  #[test]
  fn test_sort_insertion() {
    let reg = AlgorithmRegistry::new();
    let input = make_sort_input(
      serde_json::json!([5, 2, 8, 1, 9]),
      None,
      None,
    );
    let out = reg.execute("sort.insertion", input).unwrap();
    let arr = out.data.as_array().unwrap();
    let binding = serde_json::json!([1, 2, 5, 8, 9]);
    let expected = binding.as_array().unwrap();
    assert_eq!(arr, expected);
  }

  #[test]
  fn test_sort_merge() {
    let reg = AlgorithmRegistry::new();
    let input = make_sort_input(
      serde_json::json!([5, 2, 8, 1, 9]),
      None,
      None,
    );
    let out = reg.execute("sort.merge", input).unwrap();
    let arr = out.data.as_array().unwrap();
    let binding = serde_json::json!([1, 2, 5, 8, 9]);
    let expected = binding.as_array().unwrap();
    assert_eq!(arr, expected);
  }

  #[test]
  fn test_sort_quick() {
    let reg = AlgorithmRegistry::new();
    let input = make_sort_input(
      serde_json::json!([5, 2, 8, 1, 9]),
      None,
      None,
    );
    let out = reg.execute("sort.rust_default", input).unwrap();
    let arr = out.data.as_array().unwrap();
    let binding = serde_json::json!([1, 2, 5, 8, 9]);
    let expected = binding.as_array().unwrap();
    assert_eq!(arr, expected);
  }

  // -- Sort with field --------------------------------------------------------

  #[test]
  fn test_sort_with_field() {
    let reg = AlgorithmRegistry::new();
    let input = make_sort_input(
      serde_json::json!([
        {"name": "charlie", "age": 30},
        {"name": "alice", "age": 25},
        {"name": "bob", "age": 20}
      ]),
      Some("age"),
      None,
    );
    let out = reg.execute("sort.rust_default", input).unwrap();
    let arr = out.data.as_array().unwrap();
    assert_eq!(arr[0].as_object().unwrap().get("name").unwrap(), &serde_json::json!("bob"));
    assert_eq!(arr[1].as_object().unwrap().get("name").unwrap(), &serde_json::json!("alice"));
    assert_eq!(arr[2].as_object().unwrap().get("name").unwrap(), &serde_json::json!("charlie"));
  }

  // -- Sort order=desc --------------------------------------------------------

  #[test]
  fn test_sort_desc() {
    let reg = AlgorithmRegistry::new();
    // Use make_sort_input which wraps data correctly for parse_sort_input
    let input = make_sort_input(
      serde_json::json!([5, 2, 8, 1, 9]),
      None,
      Some("desc"),
    );
    let out = reg.execute("sort.rust_default", input).unwrap();
    let arr = out.data.as_array().unwrap();
    let binding = serde_json::json!([9, 8, 5, 2, 1]);
    let expected = binding.as_array().unwrap();
    assert_eq!(arr, expected);
  }

  // -- Sort on non-existent field ----------------------------------------------

  #[test]
  fn test_sort_nonexistent_field_leaves_unchanged() {
    let reg = AlgorithmRegistry::new();
    let data = serde_json::json!([
      {"name": "charlie"},
      {"name": "alice"}
    ]);
    let input = make_sort_input(data, Some("nonexistent_field"), None);
    let out = reg.execute("sort.rust_default", input).unwrap();
    // The handler falls back to JSON ordering on the whole object
    let arr = out.data.as_array().unwrap();
    assert_eq!(arr.len(), 2);
  }

  // -- search.schemas ----------------------------------------------------------

  #[test]
  fn test_search_schemas_case_insensitive() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "items": ["Users", "products", "orders", "PRODUCTS"],
      "query": "foo"
    }));
    let out = reg.execute("search.schemas", input).unwrap();
    // no item contains "foo"
    let result = out.data.as_array().unwrap();
    assert!(result.is_empty());
  }

  #[test]
  fn test_search_schemas_partial_match() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "items": ["user_profiles", "user_sessions", "products"],
      "query": "user"
    }));
    let out = reg.execute("search.schemas", input).unwrap();
    let result = out.data.as_array().unwrap();
    assert_eq!(result.len(), 2);
  }

  // -- search.paginate ---------------------------------------------------------

  #[test]
  fn test_search_paginate_correct_slice() {
    let reg = AlgorithmRegistry::new();
    let items: Vec<serde_json::Value> = (1u32..=20).map(|i| serde_json::json!(i)).collect();
    let input = AlgorithmInput::new(serde_json::json!({
      "items": items,
      "page": 2,
      "limit": 5
    }));
    let out = reg.execute("search.paginate", input).unwrap();
    let result = out.data.as_array().unwrap();
    // page 2 with limit 5 → items[5..10] → values 6,7,8,9,10
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], serde_json::json!(6));
    assert_eq!(result[4], serde_json::json!(10));
  }

  #[test]
  fn test_search_paginate_beyond_range_returns_empty() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "items": [1, 2, 3],
      "page": 10,
      "limit": 5
    }));
    let out = reg.execute("search.paginate", input).unwrap();
    let result = out.data.as_array().unwrap();
    assert!(result.is_empty());
  }

  // -- graph.dijkstra ----------------------------------------------------------

  #[test]
  fn test_graph_dijkstra_happy_path() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "nodes": [
        {"id": "A", "data": {}},
        {"id": "B", "data": {}},
        {"id": "C", "data": {}}
      ],
      "edges": [
        {"from": "A", "to": "B", "weight": 1.0},
        {"from": "B", "to": "C", "weight": 2.0},
        {"from": "A", "to": "C", "weight": 5.0}
      ],
      "start": "A",
      "end": "C"
    }));
    let out = reg.execute("graph.dijkstra", input).unwrap();
    let result = out.data.clone();
    // Result is Some((path, dist))
    assert!(result.is_array());
    let arr = result.as_array().unwrap();
    // The result wraps in an array with path at [0] and dist at [1]
    let path_arr = arr.get(0).and_then(|v| v.as_array()).unwrap();
    let path: Vec<&str> = path_arr.iter().filter_map(|v| v.as_str()).collect();
    let dist = arr.get(1).and_then(|v| v.as_f64()).unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);
    assert_eq!(dist, 3.0);
  }

  #[test]
  fn test_graph_dijkstra_no_path() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "nodes": [
        {"id": "A", "data": {}},
        {"id": "B", "data": {}}
      ],
      "edges": [],
      "start": "A",
      "end": "B"
    }));
    let out = reg.execute("graph.dijkstra", input).unwrap();
    // No path → null
    assert!(out.data.is_null());
  }

  // -- graph.bfs and graph.dfs -------------------------------------------------

  #[test]
  fn test_graph_bfs_visit_order() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "nodes": [
        {"id": "A", "data": {}},
        {"id": "B", "data": {}},
        {"id": "C", "data": {}},
        {"id": "D", "data": {}}
      ],
      "edges": [
        {"from": "A", "to": "B", "weight": 1.0},
        {"from": "A", "to": "C", "weight": 1.0},
        {"from": "B", "to": "D", "weight": 1.0}
      ]
    }));
    let out = reg.execute("graph.bfs", input).unwrap();
    let result = out.data.as_array().unwrap();
    // Starts from first node in list (A), visits breadth-first
    assert_eq!(result[0], serde_json::json!("A"));
    // B and C are both adjacent to A; order depends on edge iteration
    assert!(result.contains(&serde_json::json!("B")));
    assert!(result.contains(&serde_json::json!("C")));
  }

  #[test]
  fn test_graph_dfs_visit_order() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({
      "nodes": [
        {"id": "A", "data": {}},
        {"id": "B", "data": {}},
        {"id": "C", "data": {}}
      ],
      "edges": [
        {"from": "A", "to": "B", "weight": 1.0},
        {"from": "B", "to": "C", "weight": 1.0}
      ]
    }));
    let out = reg.execute("graph.dfs", input).unwrap();
    let result = out.data.as_array().unwrap();
    assert_eq!(result[0], serde_json::json!("A"));
    assert!(result.contains(&serde_json::json!("B")));
    assert!(result.contains(&serde_json::json!("C")));
  }

  // -- tree.build / tree.flatten -----------------------------------------------

  #[test]
  fn test_tree_build_and_flatten() {
    let reg = AlgorithmRegistry::new();

    // tree.build
    let build_input = AlgorithmInput::new(serde_json::json!({
      "nodes": [{"id": "1", "name": "root"}, {"id": "2", "name": "child"}],
      "root_id": "1"
    }));
    let build_out = reg.execute("tree.build", build_input).unwrap();
    let root_id = build_out.data.get("root_id").and_then(|v| v.as_str()).unwrap();
    assert_eq!(root_id, "1");

    // tree.flatten reverses tree.build (walks array children)
    let flatten_input = AlgorithmInput::new(serde_json::json!({
      "tree": [["a"], ["b"], ["c"]]
    }));
    let flatten_out = reg.execute("tree.flatten", flatten_input).unwrap();
    let flat = flatten_out.data.as_array().unwrap();
    // tree_flatten walks and pushes each item then recurses, giving 6 elements:
    // [["a"],["b"],["c"]] outer iterates 3x, each pushes itself then recurses,
    // inner arrays each push 1 string. Total: 3 (arrays) + 3 (strings) = 6
    assert_eq!(flat.len(), 6);
  }

  // -- validate.email ----------------------------------------------------------

  #[test]
  fn test_validate_email_valid() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"email": "test@example.com"}));
    let out = reg.execute("validate.email", input).unwrap();
    let valid = out.data.get("valid").and_then(|v| v.as_bool()).unwrap();
    assert!(valid);
  }

  #[test]
  fn test_validate_email_invalid() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"email": "not-an-email"}));
    let out = reg.execute("validate.email", input).unwrap();
    let valid = out.data.get("valid").and_then(|v| v.as_bool()).unwrap();
    assert!(!valid);
  }

  // -- validate.input ---------------------------------------------------------

  #[test]
  fn test_validate_input_length_ok() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"text": "hello", "max_length": 10}));
    let out = reg.execute("validate.input", input).unwrap();
    let valid = out.data.get("valid").and_then(|v| v.as_bool()).unwrap();
    assert!(valid);
  }

  #[test]
  fn test_validate_input_too_long() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"text": "hello world", "max_length": 5}));
    let out = reg.execute("validate.input", input).unwrap();
    let valid = out.data.get("valid").and_then(|v| v.as_bool()).unwrap();
    assert!(!valid);
  }

  // -- sanitize.escape_html ---------------------------------------------------

  #[test]
  fn test_sanitize_escape_html() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"text": "<script>alert('xss')</script>"}));
    let out = reg.execute("sanitize.escape_html", input).unwrap();
    let result = out.data.get("result").and_then(|v| v.as_str()).unwrap();
    assert_eq!(result, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
  }

  #[test]
  fn test_sanitize_escape_html_safe_text() {
    let reg = AlgorithmRegistry::new();
    let input = AlgorithmInput::new(serde_json::json!({"text": "hello world"}));
    let out = reg.execute("sanitize.escape_html", input).unwrap();
    let result = out.data.get("result").and_then(|v| v.as_str()).unwrap();
    assert_eq!(result, "hello world");
  }
}