use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
}

#[derive(Debug)]
pub enum NodeType {
  Text(String),
  Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

impl Node {
  pub fn print(self) {
    match self.node_type {
      NodeType::Text(data) => println!("NodeType = Text, Value = {:?}", data),
      NodeType::Element(data) => println!("NodeType = Element, Value = {:?}", data),
    }

    for node in self.children {
      node.print();
    }
  }
}

pub fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data),
  }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    }),
  }
}
