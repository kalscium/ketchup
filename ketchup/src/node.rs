use crate::Span;
use core::fmt::Debug;

#[derive(Debug)]
pub enum Node<Oper: Debug> {
    /// a normal node
    Node(RawNode<Oper>),
    /// a node that is scoped (other nodes cannot interact with the asa within it)
    Scoped(Vec<Node<Oper>>, NodeInfo),
}

#[derive(Debug)]
pub struct NodeInfo {
    /// the location of that node's token
    pub span: Span,
    /// the parent of the node in the `ASA`
    pub parent: Option<usize>,
    /// the precedence of the node's token type in the `ASA`
    pub precedence: u8,
    /// if there is space left for inputs for this node
    pub space: bool,
}

#[derive(Debug)]
/// a single node in the `ASA`
pub struct RawNode<Oper: Debug> {
    /// the interal type of the node
    pub oper: Oper,
    /// the node's information
    pub node_info: NodeInfo,
}

impl<Oper: Debug> Node<Oper> {
    /// creates a new node
    #[inline]
    pub fn new_node(oper: Oper, span: Span, parent: Option<usize>, precedence: u8, space: bool) -> Self {
        Self::Node(RawNode {
            oper,
            node_info: NodeInfo {
                span,
                parent,
                precedence,
                space,
            },
        })
    }


    /// returns the node's information
    #[inline]
    pub fn take_info(self) -> NodeInfo {
        match self {
            Node::Node(node) => node.node_info,
            Node::Scoped(_, node_info) => node_info,
        }
    }

    /// gets the node's information
    #[inline]
    pub fn info(&self) -> &NodeInfo {
        match self {
           Node::Node(node) => &node.node_info,
           Node::Scoped(_, node_info) => node_info,
        }
    }

    // gets the node's information (mutable)
    #[inline]
    pub fn info_mut(&mut self) -> &mut NodeInfo {
        match self {
           Node::Node(node) => &mut node.node_info,
           Node::Scoped(_, node_info) => node_info,
        }
    }
}
