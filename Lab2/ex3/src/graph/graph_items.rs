pub mod node{
    use std::collections::HashMap;

    #[derive(Debug,Clone, PartialEq)]
    pub struct Node{
        pub name: String,
        pub attrs: HashMap<String,String>
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Node{
                name: name.to_string(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self{
            Node{
                attrs: attrs.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect(),
                ..self
            }
        }

        pub fn get_attr(&self, key: &str) -> Option<&str>{
            self.attrs.get(key).map(|value| value.as_str())
        }
    }
}


pub mod edge {
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Edge{
        pub node1: String,
        pub node2: String,
        attrs: HashMap<String,String>,
    }

    impl Edge{
        pub fn new(node1: &str, node2: &str) -> Self {
            Edge{
                node1: node1.to_string(),
                node2: node2.to_string(),
                attrs: HashMap::new()
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self{
            Edge{
                attrs: attrs.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect(),
                ..self
            }
        }
    }

}


