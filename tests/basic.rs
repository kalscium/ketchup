use ketchup::{asa::{VectorASA, ASA}, error::Error, node::{Node, NodeKind}, parse, Precedence};

#[derive(Debug, Clone, PartialEq, Eq)]
enum MyNode {
    Number(i32),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Pos,
    Call(i32),
}

impl Node for MyNode {
    const MAX_PRECEDENCE: Precedence = 3;

    fn get_kind(&self) -> NodeKind {
        match self {
            MyNode::Number(_) => NodeKind::Operand,
            MyNode::Call(_) => NodeKind::Unary,
            MyNode::Pos => NodeKind::Unary,
            MyNode::Neg => NodeKind::Unary,
            MyNode::Add => NodeKind::Binary,
            MyNode::Sub => NodeKind::Binary,
            MyNode::Mul => NodeKind::Binary,
            MyNode::Div => NodeKind::Binary,
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            MyNode::Number(_) => Precedence::MAX,
            MyNode::Call(_) => 3,
            MyNode::Pos => 2,
            MyNode::Neg => 2,
            MyNode::Mul => 1,
            MyNode::Div => 1,
            MyNode::Add => 0,
            MyNode::Sub => 0,
        }
    }
}

#[test]
fn operand() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
}

#[test]
#[should_panic]
fn completed_operand() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::operand(MyNode::Number(2), &mut asa).unwrap(); // should panic
}

#[test]
#[should_panic]
fn completed_unary_left_align() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap(); // should panic
}

#[test]
fn unary_left_align() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::operand(MyNode::Number(1), &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Neg, MyNode::Number(1)]);
}

#[test]
#[should_panic]
fn incomplete_unary_right_align() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::unary_right_align(MyNode::Neg, true, &mut asa).unwrap(); // should panic
}

#[test]
fn unary_right_align() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::unary_left_align(MyNode::Pos, &mut asa).unwrap();
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(2), true, &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Neg, MyNode::Pos, MyNode::Neg, MyNode::Call(2), MyNode::Number(1)]);
}

#[test]
fn unary_right_align_left_recursive() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(2), true, &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(3), true, &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(4), true, &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Call(4), MyNode::Call(3), MyNode::Call(2), MyNode::Number(1)]);
}

#[test]
fn unary_right_align_right_recursive() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(2), false, &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(3), false, &mut asa).unwrap();
    parse::unary_right_align(MyNode::Call(4), false, &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Call(2), MyNode::Call(3), MyNode::Call(4), MyNode::Number(1)]);
}

#[test]
fn incomplete_binary() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::binary_node(MyNode::Mul, true, &mut asa).unwrap_err();
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::binary_node(MyNode::Add, true, &mut asa).unwrap();

    assert!(!*asa.is_complete());
}

#[test]
fn binary_left_recursive() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(11), &mut asa).unwrap();
    parse::binary_node(MyNode::Add, true, &mut asa).unwrap();
    parse::operand(MyNode::Number(2), &mut asa).unwrap();
    parse::binary_node(MyNode::Sub, true, &mut asa).unwrap();
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::operand(MyNode::Number(4), &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Sub, MyNode::Add, MyNode::Number(11), MyNode::Number(2), MyNode::Neg, MyNode::Number(4)]);
}

#[test]
fn binary_right_recursive() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::operand(MyNode::Number(11), &mut asa).unwrap();
    parse::binary_node(MyNode::Add, false, &mut asa).unwrap();
    parse::operand(MyNode::Number(2), &mut asa).unwrap();
    parse::binary_node(MyNode::Sub, false, &mut asa).unwrap();
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::operand(MyNode::Number(4), &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [MyNode::Add, MyNode::Number(11), MyNode::Sub, MyNode::Number(2), MyNode::Neg, MyNode::Number(4)]);
}

#[test]
fn incomplete_error_walking() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);
    parse::unary_left_align(MyNode::Neg, &mut asa).unwrap();
    parse::unary_left_align(MyNode::Pos, &mut asa).unwrap();
    parse::operand(MyNode::Number(12), &mut asa).unwrap();
    parse::binary_node(MyNode::Div, true, &mut asa).unwrap();

    assert!(!*asa.is_complete());
    assert_eq!(MyNode::Div, *parse::incomplete_error(&mut asa).unwrap());

    let Err(Error::ExpectedNode(Some(MyNode::Div))) = parse::ensure_completed(&mut asa)
    else {
        panic!("assert failed");
    };
}

#[test]
fn mix_of_precedences() {
    let mut asa = VectorASA::<MyNode>::new(MyNode::MAX_PRECEDENCE);

    // 1 + 2 * 3 / 4
    parse::operand(MyNode::Number(1), &mut asa).unwrap();
    parse::binary_node(MyNode::Add, true, &mut asa).unwrap();
    parse::operand(MyNode::Number(2), &mut asa).unwrap();
    parse::binary_node(MyNode::Mul, true, &mut asa).unwrap();
    parse::operand(MyNode::Number(3), &mut asa).unwrap();
    parse::binary_node(MyNode::Div, true, &mut asa).unwrap();
    parse::operand(MyNode::Number(4), &mut asa).unwrap();

    assert!(*asa.is_complete());
    assert_eq!(asa.vector[..], [
        MyNode::Add,
        MyNode::Number(1),
        MyNode::Div,
        MyNode::Mul,
        MyNode::Number(2),
        MyNode::Number(3),
        MyNode::Number(4),
    ]);
}
