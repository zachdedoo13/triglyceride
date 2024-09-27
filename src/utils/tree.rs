use std::collections::HashMap;
use crate::StatString;

#[derive(Debug, Default)]
pub struct TreeNode {
   pub name: StatString,
   pub parent: Option<StatString>,
   pub children: Vec<StatString>,
}
impl TreeNode {
   pub fn new(id: StatString) -> Self {
      Self {
         name: id,
         parent: None,
         children: Vec::new(),
      }
   }
}

/// simple internal tree for graphing relationships
#[derive(Debug, Default)]
pub struct Tree {
   pub nodes: HashMap<StatString, TreeNode>,
   pub root: Option<StatString>,
}

impl Tree {
   pub fn set_root(&mut self, id: StatString) {
      if !self.nodes.contains_key(id) {
         let node = TreeNode::new(id);
         self.nodes.insert(id, node);
      }
      self.root = Some(id);
   }

   pub fn add_child(&mut self, parent_id: StatString, child_id: StatString) {
      if !self.nodes.contains_key(parent_id) {
         let parent_node = TreeNode::new(parent_id);
         self.nodes.insert(parent_id, parent_node);
      }

      if !self.nodes.contains_key(child_id) {
         let child_node = TreeNode {
            name: child_id,
            parent: Some(parent_id),
            children: Vec::new(),
         };
         self.nodes.insert(child_id, child_node);
      }

      if let Some(parent_node) = self.nodes.get_mut(parent_id) {
         parent_node.children.push(child_id);
      }
   }

   pub fn clear(&mut self) {
      self.nodes.clear();
      self.root = None;
   }
}