pub mod graph {
    use std::collections::HashMap;
    use std::fmt;
    use crate::graph::graph_items::{edge, node};

    pub mod graph_items;
    //use graph_items::{edge,node}

    pub struct Graph{
        pub nodes: Vec<node::Node>,
        pub edges: Vec<edge::Edge>,
        pub attrs: HashMap<String,String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(self, nodes: &[node::Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[edge::Edge]) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect(),
                ..self
            }
        }
        pub fn get_node(&self, name: &str) -> Option<&node::Node> {
            self.nodes.iter().find(|node| node.name == name)
            //self.nodes.iter().find(|node| node.name == name)
        }
    }

    impl fmt::Display for Graph{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let edges = self.edges
                .iter()
                .map(|edge|(edge.to_string()))
                .collect::<Vec<String>>().join(" ");
            write!(f, "strict graph {{{}}}",edges)
        }
    }
}
