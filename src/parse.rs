//! Functinos for parsing and manipulating the ASA

use crate::{asa, error::Error};

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
