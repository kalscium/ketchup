# ketchup
---
> A blazingly fast parser that can *ketch - up* with your parsing needs.

## Freedom from Frameworks
---
> The **best** parser is always a **hand-written** parser

I won't bore you to death with the details of the philosophy behind ketchup.

All you need to know to get started is that:
- ketchup is **not** a parsing framework, and it is not a *complete* be-all end-all solution.
- ketchup is **library**, that's designed in a way as to be used alongside other parsing techniques; it's designed to be embedded wherever it is deemed most effective to automate parsing that would be otherwise too arduous to hand-write and maintain.
- ketchup should not, and does not, define how your project is structured or works and it should never be used alone.

In short, ketchup gives you the **freedom to choose**.

# Examples
---
*for more complete examples, see the `examples` folder*

## A minimal maths demo
```rust
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
    const MAX_PRECEDENCE: ketchup::Precedence = 2;

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
        match self {
            // precedence helps determine the order in which nodes get 'evaluated';
            // the larger the precedence, the 'earlier' it will get 'evaluated'
            //
            // it is not possible to query the precedence of an operand and it won't ever happen unless ketchup has a critical bug so it's arlight to just panic

            // operands
            Expr::Number(_) => unreachable!(),

            // unary nodes (left-align)
            Expr::Pos => 2,
            Expr::Neg => 2,

            // binary nodes
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
    let mut asa = VectorASA::<Expr>::new(Expr::MAX_PRECEDENCE);

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
    assert!(*asa.is_complete()); // verify that the ASA is not missing any expected nodes
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
```
