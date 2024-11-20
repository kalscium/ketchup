//! Functinos for parsing and manipulating the ASA

use std::cmp::Ordering;

use crate::{asa, error::Error, node::{Node, NodeKind}};

/// Finds the last binary or unary node in the ASA
pub fn last_unary_binary<ASA: asa::ASA>(asa: &mut ASA) -> Option<&ASA::Node> {
    for i in (0..asa.get_len()).rev() {
        let node = asa.get_node(i);
        match node.get_kind() {
            NodeKind::Unary | NodeKind::Binary => return Some(asa.get_node(i)),
            _ => (),
        }
    }

    None
}

/// Parses an operand node and inserts it into the ASA
pub fn operand<ASA: asa::ASA>(node: ASA::Node, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete, if so, throw error
    if *asa.completed() {
        return Err(Error::UnexpectedNode(node));
    }

    // otherwise, push it to the end of the ASA and update complete-ness field
    asa.push(node);
    *asa.completed() = true;

    Ok(())
}

/// Parses a left-aligned unary node and inserts it into the ASA
pub fn unary_left_align<ASA: asa::ASA>(node: ASA::Node, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete, if so, throw error
    if *asa.completed() {
        return Err(Error::UnexpectedNode(node));
    }
    // otherwise push it without modifying complete-ness
    asa.push(node);
    Ok(())
}

/// Parses a right-aligned unary node and inserts it into the ASA based on if it's right or left recursive
pub fn unary_right_align<ASA: asa::ASA>(node: ASA::Node, left_recursive: bool, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is incomplete, if so, throw error
    if !*asa.completed() {
        return Err(Error::UnexpectedExpectedNode {
            oper: last_unary_binary(asa),
            found: node,
        });
    }

    // compare against the first node of the ASA
    let first = asa.get_node(0);
    match node.get_precedence().cmp(&first.get_precedence()) {
        // if the ndoe has a lower precedence or an equal precedence with left-recursion then insert to the start of the ASA
        Ordering::Less => {
            asa.push_start(node);
            return Ok(());
        },
        Ordering::Equal if left_recursive => {
            asa.push_start(node);
            return Ok(());
        },
        _ => (),
    }

    // otherwise compare against the last node (check for neighboring unary nodes)
    if asa.get_node(asa.get_len()-2).get_kind() == NodeKind::Unary {
        // get the last consecutive unary node starting from the neighboring node to the last node of the ASA
        let mut idx = asa.get_len()-2;
        for i in (0..asa.get_len()-1).rev() {
            let current = asa.get_node(i);
            if current.get_kind() != NodeKind::Unary {
                idx = i-1;
            }
        }

        // insert it at the index
        asa.insert(idx, node);
        return Ok(());
    }

    // otherwise just insert at the location of the last node
    asa.insert(asa.get_len()-1, node);
    Ok(())
}
