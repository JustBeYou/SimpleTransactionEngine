use super::{deposit, withdrawal, Engine};
use crate::{errors::EngineError, transaction::Transaction};

pub fn execute(e: &mut Engine, transaction: &Transaction) -> Result<(), EngineError> {
    let (target, client) = e.get_mut_transaction_client_pair(transaction.tx)?;

    match target.kind {
        crate::transaction::TransactionType::DEPOSIT => deposit::dispute(client, target),
        crate::transaction::TransactionType::WITHDRAWAL => withdrawal::dispute(client, target),
        _ => Err(EngineError::InvalidTransactionType),
    }
}
