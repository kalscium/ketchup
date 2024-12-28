//! Functions for parsing and manipulating the ASA

use crate::{asa, error::Error, node::Node};

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

    // otherwise update the lookup-table and
    asa.lookuptable()[node.get_precedence()] = Some(asa.get_len());

    // push it without modifying complete-ness
    asa.push(node);

    // also update the `last_incomplete` field
    *asa.last_incomplete() = Some(asa.get_len()-1);

    Ok(())
}

/// Inserts a node into the ASA based on a precedence index lookup-table and it's association (left or right), returns the index of which the node was inserted at
fn insert_lookuptable<ASA: asa::ASA>(node: ASA::Node, left_associative: bool, asa: &mut ASA) -> usize {
    // determine the range of precedences lower than the current one
    let range = if left_associative {
        node.get_precedence().. // treat equal precedence as lesser than
    } else {
        node.get_precedence()+1.. // treat equal precedence as greater than
    };

    // iterate through the lookup-table and find any indexes of greater precedence
    let lower_prec = asa.lookuptable()[range]
        .iter()
        .find_map(|idx| idx.clone());

    // if there is one, then simply insert at that index and update the lookup-table and return
    if let Some(idx) = lower_prec {
        // update the lookup-table
        asa.lookuptable()[node.get_precedence()] = Some(idx);

        // update the indexes of greater precedence through incrementing them
        for idx in asa.lookuptable()[node.get_precedence()+1..].iter_mut() {
            if let Some(idx) = idx {
                *idx += 1;
            }
        }

        asa.insert(idx, node);
        return idx;
    }

    // otherwise, simply insert at the end of the ASA and update the lookup-table
    let end = asa.get_len() - 1;
    asa.lookuptable()[node.get_precedence()] = Some(end);
    asa.insert(end, node);

    end
}

/// Parses a right-aligned unary node and inserts it into the ASA based on if it's right or left associative
pub fn unary_right_align<ASA: asa::ASA>(node: ASA::Node, left_associative: bool, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is incomplete, if so, throw error
    if !*asa.is_complete() {
        return Err(Error::UnexpectedExpectedNode {
            oper: incomplete_error(asa),
            found: node,
        });
    }

    // insert into the ASA based upon the lookup-table
    insert_lookuptable(node, left_associative, asa);

    Ok(())
}

/// Parses a binary node and inserts it into the ASA based on if it's left or right associative
pub fn binary_node<ASA: asa::ASA>(node: ASA::Node, left_associative: bool, asa: &mut ASA) -> Result<(), Error<ASA::Node>> {
    // check if the asa is incomplete, if so, throw error
    if !*asa.is_complete() {
        return Err(Error::UnexpectedExpectedNode {
            oper: incomplete_error(asa),
            found: node,
        });
    }

    // insert into the ASA based upon the lookuptable
    let idx = insert_lookuptable(node, left_associative, asa);

    // update the completeness fields
    *asa.is_complete() = false;
    *asa.last_incomplete() = Some(idx);

    Ok(())
}
