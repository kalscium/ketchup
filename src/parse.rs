//! Functions for parsing and manipulating the ASA

use std::cmp::Ordering;
use crate::{asa, error::Error, node::{Node, NodeKind}};

/// Walks up the ASA from the end and finds the unary or binary node that is incomplete and is therefore causing the error
pub fn walk_incomplete_error<ASA: asa::ASA>(asa: &ASA) -> Option<&ASA::Node> { // not a perfect way to find errors
    for i in (0..asa.get_len()).rev() { // could be made more efficient with if-guards
        let node = asa.get_node(i);
        match node.get_kind() {
            // if the unary node is at the end, then it can't be complete
            NodeKind::Unary if i == asa.get_len()-1 => return Some(asa.get_node(i)),
            // the last binary node (if there is no unary node at the end) cannot be complete
            NodeKind::Binary => return Some(asa.get_node(i)),
            // move on to the next node
            _ => (),
        }
    }

    // can only occur when the ASA is empty
    None
}

/// Ensures that an ASA is completed, otherwise, returns a walked incomplete error
pub fn ensure_completed<ASA: asa::ASA>(asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete
    if *asa.completed() {
        Ok(())
    } else {
        Err(Error::ExpectedNode(walk_incomplete_error(asa)))
    }
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
            oper: walk_incomplete_error(asa),
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

    // otherwise compare against the last node (check for neighbouring unary nodes)
    let neighbour = asa.get_node(asa.get_len()-2); // alright as there must be a node before the last node at this point in the code
    match (neighbour.get_kind(), node.get_precedence().cmp(&neighbour.get_precedence())) {
        (NodeKind::Unary, Ordering::Less) => (),
        (NodeKind::Unary, Ordering::Equal) if left_recursive => (),
        _ => {
            // just insert before the last node and return
            asa.insert(asa.get_len()-1, node);
            return Ok(());
        },
    }

    // at this point, the neighbouring unary node must be of a greater precedence

    // get the last consecutive unary node of lower precedence starting from the neighboring node to the last node of the ASA
    let mut idx = asa.get_len()-2;
    for i in (0..asa.get_len()-1).rev() {
        let neighbour = asa.get_node(i);
        // break if the current one is no longer a unary node of greater precedence
        match (neighbour.get_kind(), node.get_precedence().cmp(&neighbour.get_precedence())) {
            (NodeKind::Unary, Ordering::Less) => (),
            (NodeKind::Unary, Ordering::Equal) if left_recursive => (),
            _ => {
                idx = i - 1;
                break;
            },
        }
    }

    // insert it at the index
    asa.insert(idx, node);
    Ok(())
}

/// Parses a binary node and inserts it into the ASA based on if it's left or right recursive
pub fn binary_node<ASA: asa::ASA>(node: ASA::Node, left_recursive: bool, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is incomplete, if so, throw error
    if !*asa.completed() {
        return Err(Error::UnexpectedExpectedNode {
            oper: walk_incomplete_error(asa),
            found: node,
        });
    }

    // compare against the first node of the ASA
    let first = asa.get_node(0);
    match node.get_precedence().cmp(&first.get_precedence()) {
        // if the ndoe has a lower precedence or an equal precedence with left-recursion then insert to the start of the ASA and mark it incomplete
        Ordering::Less => {
            asa.push_start(node);
            *asa.completed() = false;
            return Ok(());
        },
        Ordering::Equal if left_recursive => {
            asa.push_start(node);
            *asa.completed() = false;
            return Ok(());
        },
        _ => (),
    }

    // otherwise compare against the last node (check for neighbouring unary nodes)
    let neighbour = asa.get_node(asa.get_len()-2); // alright as there must be a node before the last node at this point in the code
    match (neighbour.get_kind(), node.get_precedence().cmp(&neighbour.get_precedence())) {
        (NodeKind::Unary, Ordering::Less) => (),
        (NodeKind::Unary, Ordering::Equal) if left_recursive => (),
        _ => {
            // just insert before the last node, mark incomplete and return
            asa.insert(asa.get_len()-1, node);
            *asa.completed() = false;
            return Ok(());
        },
    }

    // at this point, the neighbouring unary node must be of a greater precedence

    // get the last consecutive unary node of lower precedence starting from the neighboring node to the last node of the ASA
    let mut idx = asa.get_len()-2;
    for i in (0..asa.get_len()-1).rev() {
        let neighbour = asa.get_node(i);
        // break if the current one is no longer a unary node of greater precedence
        match (neighbour.get_kind(), node.get_precedence().cmp(&neighbour.get_precedence())) {
            (NodeKind::Unary, Ordering::Less) => (),
            (NodeKind::Unary, Ordering::Equal) if left_recursive => (),
            _ => {
                idx = i - 1;
                break;
            },
        }
    }

    // insert it at the index
    asa.insert(idx, node);
    *asa.completed() = false;
    Ok(())
}
