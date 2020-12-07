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
pub struct TreeNode {
    pub id: u32,
    pub is_expanded: bool,
    pub is_checked: bool,
    pub is_indeterminate: bool,
    pub level: u32,
    pub childs: Vec<TreeNode>
}

impl TreeNode {
    pub fn new(node: Box<& RawNode>) -> TreeNode {
        TreeNode {
            id: node.id,
            level: node.level,
            is_checked: false,
            is_expanded: false,
            is_indeterminate: false,
            childs: node.childs.iter().map(|child| TreeNode::new(Box::new(child))).collect()
        }
    }
}

pub fn build_tree(node: Box<&RawNode>) -> TreeNode {
    let root = TreeNode::new(node);


    root
}