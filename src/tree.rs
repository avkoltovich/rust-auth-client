use serde::Deserialize;
use crate::requests::requests::NodeType;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawNode<'a> {
    pub id: u32,
    #[serde(rename = "type")]
    pub node_type: &'a NodeType,
    pub level: u32,
    pub childs: Vec<RawNode<'a>>
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    pub id: u32,
    pub node_type: Box<&'a NodeType>,
    pub is_expanded: bool,
    pub is_checked: bool,
    pub is_indeterminate: bool,
    pub level: u32,
    pub childs: Vec<TreeNode<'a>>
}

impl TreeNode<'a> {
    pub fn new(node: Box<& RawNode>) -> TreeNode<'a> {
        TreeNode {
            id: node.id,
            node_type: Box::new(&node.node_type),
            level: node.level,
            is_checked: false,
            is_expanded: false,
            is_indeterminate: false,
            childs: node.childs.iter().map(|child| TreeNode::new(Box::new(child))).collect()
        }
    }
}

pub fn build_tree(node: Box<&'static RawNode>) -> TreeNode {
    let root = TreeNode::new(node);


    root
}