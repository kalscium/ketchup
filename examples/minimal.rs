#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i32),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Pos,
}

// define the precedence and kind of the node
impl ketchup::node::Node for Expr {
    fn get_kind(&self) -> ketchup::node::NodeKind {
        use ketchup::node::NodeKind;

        match self {
            // operands are independant nodes that don't require any other nodes to be 'complete'
            Expr::Number(_) => NodeKind::Operand,
            // Unary nodes are nodes that require another extra node to be 'complete'
            Expr::Pos => NodeKind::Unary,
            Expr::Neg => NodeKind::Unary,
            // Binary nodes are nodes that require two other nodes to be 'complete'
            Expr::Add => NodeKind::Binary,
            Expr::Sub => NodeKind::Binary,
            Expr::Mul => NodeKind::Binary,
            Expr::Div => NodeKind::Binary,
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
            Expr::Number(_) => Precedence::MAX,
            Expr::Pos => Precedence::MAX-1,
            Expr::Neg => Precedence::MAX-1,
            Expr::Mul => 1,
            Expr::Div => 1,
            Expr::Add => 0,
            Expr::Sub => 0,
        }
    }
}

// that's it, that's all you have to do to setup the ketchup parser

fn main() {
    use ketchup::prelude::*; // import common imports

    // initialise a new AbstractSyntaxArray
    let mut asa = VectorASA::<Expr>::new();

    // parse a number
    let number = Expr::Number(12);
    parse::operand(number, &mut asa).unwrap(); // shouldn't throw any errors

    // parse an add node
    parse::binary_node(Expr::Add, true, &mut asa).unwrap();

    // parse another number
    let number = Expr::Number(4);
    parse::operand(number, &mut asa).unwrap();

    // parse a multiply node
    parse::binary_node(Expr::Mul, true, &mut asa).unwrap();

    // parse another number
    let number = Expr::Number(8);
    parse::operand(number, &mut asa).unwrap();

    // verify everything is working
    assert!(*asa.completed()); // verify that the ASA is not missing any expected nodes
    assert_eq!( // verify that the abstract syntax array is correct *(if it wasn't this wouldn't be a very good library would it?)*
        asa.vector[..],
        [ // the array is structured really weirdly to help you visualise the structure of the ASA as a tree
                                 Expr::Add,
            Expr::Number(12),               Expr::Mul,
                                 Expr::Number(4), Expr::Number(8),
        ],
    );

    // so, this isn't very practical *by itself*, but when paired with a lexer (such as logos), a fancy error reporting system such as ariadne, and some hand-written parsing (like for function definitions), ketchup becomes an extremely flexible and powerful parser that's fit for any project
    //
    // if you want to see more of ketchup's power, then check the crates in `examples` directory
}
