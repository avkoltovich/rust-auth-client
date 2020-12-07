use serde::Deserialize;
use crate::requests::requests::NodeType;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawNode {
    pub id: u32,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub level: u32,
    pub childs: Vec<RawNode>
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub node_type: &'a NodeType,
    pub is_expanded: bool,
    pub is_checked: bool,
    pub is_indeterminate: bool,
    pub level: u32,
    pub childs: Vec<TreeNode<'a>>
}

impl TreeNode<'_> {
    pub fn new(node: Box<& RawNode>, parent_id: Option<u32>) -> TreeNode {
        TreeNode {
            id: node.id,
            parent_id,
            node_type: &node.node_type,
            level: node.level,
            is_checked: false,
            is_expanded: false,
            is_indeterminate: false,
            childs: node.childs.iter().map(|child| TreeNode::new(Box::new(child), Some(node.id))).collect()
        }
    }
}

pub fn build_tree(node: Box<&RawNode>) -> TreeNode {
    let root = TreeNode::new(node, None);


    root
}