//! Functions for parsing and manipulating the ASA

use std::cmp::Ordering;
use crate::{asa, error::Error, node::{Node, NodeKind}};

/// Returns a reference to the incomplete operation in the ASA
pub fn incomplete_error<ASA: asa::ASA>(asa: &mut ASA) -> Option<&ASA::Node> {
    // quick assert in case the user is using this function wrong
    assert!(!*asa.is_complete(), "you shouldn't be returning an error for an incomplete ASA if the ASA is complete");

    // index into and return the troublesome node (if there is one)
    let idx = asa.last_incomplete();
    if let Some(idx) = *idx {
        Some(asa.get_node(idx))
    } else {
        None
    }
}

/// Ensures that an ASA is completed, otherwise, returns a walked incomplete error
pub fn ensure_completed<ASA: asa::ASA>(asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete
    if *asa.is_complete() {
        Ok(())
    } else {
        Err(Error::ExpectedNode(incomplete_error(asa)))
    }
}

/// Parses an operand node and inserts it into the ASA
pub fn operand<ASA: asa::ASA>(node: ASA::Node, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete, if so, throw error
    if *asa.is_complete() {
        return Err(Error::UnexpectedNode(node));
    }

    // otherwise, push it to the end of the ASA and update complete-ness field
    asa.push(node);
    *asa.is_complete() = true;

    Ok(())
}

/// Parses a left-aligned unary node and inserts it into the ASA
pub fn unary_left_align<ASA: asa::ASA>(node: ASA::Node, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is complete, if so, throw error
    if *asa.is_complete() {
        return Err(Error::UnexpectedNode(node));
    }

    // otherwise push it without modifying complete-ness
    asa.push(node);

    // also update the `last_incomplete` field
    *asa.last_incomplete() = Some(asa.get_len()-1);

    Ok(())
}

/// Parses a right-aligned unary node and inserts it into the ASA based on if it's right or left recursive
pub fn unary_right_align<ASA: asa::ASA>(node: ASA::Node, left_recursive: bool, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is incomplete, if so, throw error
    if !*asa.is_complete() {
        return Err(Error::UnexpectedExpectedNode {
            oper: incomplete_error(asa),
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
    if !*asa.is_complete() {
        return Err(Error::UnexpectedExpectedNode {
            oper: incomplete_error(asa),
            found: node,
        });
    }

    // compare against the first node of the ASA
    let first = asa.get_node(0);
    match node.get_precedence().cmp(&first.get_precedence()) {
        // if the ndoe has a lower precedence or an equal precedence with left-recursion then insert to the start of the ASA and update complete-ness fields
        Ordering::Less => {
            asa.push_start(node);

            *asa.is_complete() = false;
            *asa.last_incomplete() = Some(0);

            return Ok(());
        },
        Ordering::Equal if left_recursive => {
            asa.push_start(node);

            *asa.is_complete() = false;
            *asa.last_incomplete() = Some(0);

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
            // just insert before the last node, mark incomplete, update last incomplete and return
            let idx = asa.get_len()-1;
            asa.insert(idx, node);

            *asa.is_complete() = false;
            *asa.last_incomplete() = Some(idx);

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

    // insert it at the index, and update completeness fields
    asa.insert(idx, node);
    *asa.is_complete() = false;
    *asa.last_incomplete() = Some(idx);

    Ok(())
}
