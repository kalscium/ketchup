use ketchup::{asa::{VectorASA, ASA}, node::{self, NodeKind}, parse, Precedence};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Number(i32),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Pos,
    Call(i32),
}

impl node::Node for Node {
    fn get_kind(&self) -> NodeKind {
        match self {
            Node::Number(_) => NodeKind::Operand,
            Node::Call(_) => NodeKind::Unary,
            Node::Pos => NodeKind::Unary,
            Node::Neg => NodeKind::Unary,
            Node::Add => NodeKind::Binary,
            Node::Sub => NodeKind::Binary,
            Node::Mul => NodeKind::Binary,
            Node::Div => NodeKind::Binary,
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            Node::Number(_) => Precedence::MAX,
            Node::Call(_) => Precedence::MAX-1,
            Node::Pos => Precedence::MAX-2,
            Node::Neg => Precedence::MAX-2,
            Node::Mul => 1,
            Node::Div => 1,
            Node::Add => 0,
            Node::Sub => 0,
        }
    }
}

#[test]
fn operand() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(1), &mut asa).unwrap();
}

#[test]
#[should_panic]
fn completed_operand() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::operand(Node::Number(2), &mut asa).unwrap(); // should panic
}

#[test]
#[should_panic]
fn completed_unary_left_align() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap(); // should panic
}

#[test]
fn unary_left_align() {
    let mut asa = VectorASA::<Node>::new();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap();
    parse::operand(Node::Number(1), &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Neg, Node::Number(1)]);
}

#[test]
#[should_panic]
fn incomplete_unary_right_align() {
    let mut asa = VectorASA::<Node>::new();
    parse::unary_right_align(Node::Neg, true, &mut asa).unwrap(); // should panic
}

#[test]
fn unary_right_align() {
    let mut asa = VectorASA::<Node>::new();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap();
    parse::unary_left_align(Node::Pos, &mut asa).unwrap();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::unary_right_align(Node::Call(2), true, &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Neg, Node::Pos, Node::Neg, Node::Call(2), Node::Number(1)]);
}

#[test]
fn unary_right_align_left_recursive() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::unary_right_align(Node::Call(2), true, &mut asa).unwrap();
    parse::unary_right_align(Node::Call(3), true, &mut asa).unwrap();
    parse::unary_right_align(Node::Call(4), true, &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Call(4), Node::Call(3), Node::Call(2), Node::Number(1)]);
}

#[test]
fn unary_right_align_right_recursive() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::unary_right_align(Node::Call(2), false, &mut asa).unwrap();
    parse::unary_right_align(Node::Call(3), false, &mut asa).unwrap();
    parse::unary_right_align(Node::Call(4), false, &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Call(2), Node::Call(3), Node::Call(4), Node::Number(1)]);
}

#[test]
fn incomplete_binary() {
    let mut asa = VectorASA::<Node>::new();
    parse::binary_node(Node::Add, true, &mut asa).unwrap_err();
    parse::operand(Node::Number(1), &mut asa).unwrap();
    parse::binary_node(Node::Add, true, &mut asa).unwrap();

    assert!(!*asa.completed());
}

#[test]
fn binary_left_recursive() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(11), &mut asa).unwrap();
    parse::binary_node(Node::Add, true, &mut asa).unwrap();
    parse::operand(Node::Number(2), &mut asa).unwrap();
    parse::binary_node(Node::Sub, true, &mut asa).unwrap();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap();
    parse::operand(Node::Number(4), &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Sub, Node::Add, Node::Number(11), Node::Number(2), Node::Neg, Node::Number(4)]);
}

#[test]
fn binary_right_recursive() {
    let mut asa = VectorASA::<Node>::new();
    parse::operand(Node::Number(11), &mut asa).unwrap();
    parse::binary_node(Node::Add, false, &mut asa).unwrap();
    parse::operand(Node::Number(2), &mut asa).unwrap();
    parse::binary_node(Node::Sub, false, &mut asa).unwrap();
    parse::unary_left_align(Node::Neg, &mut asa).unwrap();
    parse::operand(Node::Number(4), &mut asa).unwrap();

    assert!(*asa.completed());
    assert_eq!(asa.vector[..], [Node::Add, Node::Number(11), Node::Sub, Node::Number(2), Node::Neg, Node::Number(4)]);
}
