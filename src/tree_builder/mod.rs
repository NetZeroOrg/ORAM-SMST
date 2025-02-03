use std::fmt::Debug;

use crate::{
    error::{ErrorKind, Result},
    node_position::NodePosition,
    nodes::TreeNode,
    secret::Secret,
};
mod multi;
pub mod single;

#[derive(Debug)]
pub struct Pair<T: TreeNode + Debug> {
    left: Option<(NodePosition, T)>,
    right: Option<(NodePosition, T)>,
}

impl<T: TreeNode + Debug> Pair<T> {
    pub fn new(left: Option<(NodePosition, T)>, right: Option<(NodePosition, T)>) -> Self {
        if left.is_none() && right.is_none() {
            panic!("Both node empty in pair")
        }
        Self { left, right }
    }

    pub fn is_matched(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    pub fn pad_if_not_match<F: Fn(&NodePosition) -> PaddingNodeContent>(
        &mut self,
        padding_node_content: &F,
    ) -> Result<()> {
        match self.left {
            Some((left_pos, _)) => match self.right {
                Some(_) => (),
                None => {
                    let pos = left_pos.get_sibling_pos();
                    let content = padding_node_content(&pos);
                    self.right = Some((pos, T::new_pad(content, pos)))
                }
            },
            None => match self.right {
                Some((right_pos, _)) => {
                    let pos = right_pos.get_sibling_pos();
                    let content = padding_node_content(&pos);
                    self.left = Some((pos, T::new_pad(content, pos)))
                }
                None => return Err(ErrorKind::BothNodesEmpty),
            },
        }
        Ok(())
    }

    pub fn merge(&self) -> Result<(NodePosition, T)> {
        match (&self.left, &self.right) {
            (Some((left_pos, left)), Some((_, right))) => {
                let parent_pos = left_pos.get_parent_node_pos();
                let parent = T::merge(left, right);
                Ok((parent_pos, parent))
            }
            _ => Err(ErrorKind::FoundUnmatchedNodes),
        }
    }
}

/// Padding node content
/// 1 - blinding factor
/// 2 - user secret
#[derive(Clone)]
pub struct PaddingNodeContent(Secret, Secret);

impl PaddingNodeContent {
    pub fn bliding_factor(&self) -> Secret {
        self.0
    }
    pub fn user_secret(&self) -> Secret {
        self.1
    }

    pub fn new(blinding_factor: Secret, user_secret: Secret) -> Self {
        PaddingNodeContent(blinding_factor, user_secret)
    }
}
