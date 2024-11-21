#[derive(Debug, Clone, PartialEq, Eq)]
enum MyNode {
    Number(i32),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Pos,
}

// define the precedence and kind of the node
impl ketchup::node::Node for MyNode {
    fn get_kind(&self) -> ketchup::node::NodeKind {
        use ketchup::node::NodeKind;

        match self {
            // operands are independant nodes that don't require any other nodes to be 'complete'
            MyNode::Number(_) => NodeKind::Operand,
            // Unary nodes are nodes that require another extra node to be 'complete'
            MyNode::Pos => NodeKind::Unary,
            MyNode::Neg => NodeKind::Unary,
            // Binary nodes are nodes that require two other nodes to be 'complete'
            MyNode::Add => NodeKind::Binary,
            MyNode::Sub => NodeKind::Binary,
            MyNode::Mul => NodeKind::Binary,
            MyNode::Div => NodeKind::Binary,
        }
    }

    fn get_precedence(&self) -> ketchup::Precedence {
        use ketchup::Precedence;

        match self {
            // precedence helps determine the order in which nodes get 'evaluated';
            // the larger the precedence, the 'earlier' it will get 'evaluated'
            //
            // the precedence of a unary node should be higher than the precedence of operands,
            // and no binary node can have a precedence higher than a unary node
            // operand > unary > binary
            MyNode::Number(_) => Precedence::MAX,
            MyNode::Pos => Precedence::MAX-1,
            MyNode::Neg => Precedence::MAX-1,
            MyNode::Mul => 1,
            MyNode::Div => 1,
            MyNode::Add => 0,
            MyNode::Sub => 0,
        }
    }
}

// that's it, that's all you have to do to setup the ketchup parser

fn main() {
    use ketchup::prelude::*; // import common imports

    // initialise a new AbstractSyntaxArray
    let mut asa = VectorASA::<MyNode>::new();

    // parse a number
    let number = MyNode::Number(12);
    parse::operand(number, &mut asa).unwrap(); // shouldn't throw any errors

    // parse an add node
    parse::binary_node(MyNode::Add, true, &mut asa).unwrap();

    // parse another number
    let number = MyNode::Number(4);
    parse::operand(number, &mut asa).unwrap();

    // parse a multiply node
    parse::binary_node(MyNode::Mul, true, &mut asa).unwrap();

    // parse another number
    let number = MyNode::Number(8);
    parse::operand(number, &mut asa).unwrap();

    // verify everything is working
    assert!(asa.completed()); // verify that the ASA is not missing any expected nodes
    assert_eq!( // verify that the abstract syntax array is correct *(if it wasn't this wouldn't be a very good library would it?)*
        asa.vector[..],
        [ // the array is structured really weirdly to help you visualise the structure of the ASA as a tree
                                 MyNode::Add,
            MyNode::Number(12),               MyNode::Mul,
                                 MyNode::Number(4), MyNode::Number(8),
        ],
    );

    // so, this isn't very practical *by itself*, but when paired with a lexer (such as logos), a fancy error reporting system such as ariadne, and some hand-written parsing (like for function definitions), ketchup becomes an extremely flexible and powerful parser that's fit for any project
    //
    // if you want to see more of ketchup's power, then check the crates in `examples` directory
}
