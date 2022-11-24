use tree::NodeId;

pub mod layout_attributes;
pub mod node;
pub mod node_ref;
pub mod real_dom;
pub mod state;
#[doc(hidden)]
pub mod traversable;
pub mod tree;
pub mod utils;

/// A id for a node that lives in the real dom.
type RealNodeId = NodeId;

/// Used in derived state macros
#[derive(Eq, PartialEq)]
#[doc(hidden)]
pub struct HeightOrdering {
    pub height: u16,
    pub id: RealNodeId,
}

impl HeightOrdering {
    pub fn new(height: u16, id: RealNodeId) -> Self {
        HeightOrdering { height, id }
    }
}

// not the ordering after height is just for deduplication it can be any ordering as long as it is consistent
impl Ord for HeightOrdering {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height).then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for HeightOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
