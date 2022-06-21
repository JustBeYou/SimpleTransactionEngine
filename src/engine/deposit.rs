use super::Engine;
use crate::client::Client;
use crate::errors::EngineError;
use crate::transaction::{Transaction, TransactionDisputeStatus};

pub fn execute(e: &mut Engine, transaction: &Transaction) -> Result<(), EngineError> {
    e.clients
        .entry(transaction.client)
        .or_insert(Client::new(transaction.client))
        .deposit_funds(transaction.amount)
        .and_then(|_| Ok(e.transactions.insert(transaction.tx, *transaction)))
        .map(|_| ())
}

pub fn dispute(client: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::NONE)?;

    client.hold_funds(transaction.amount)?;
    transaction.dispute_status = TransactionDisputeStatus::DISPUTED;

    Ok(())
}

pub fn resolve(client: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::DISPUTED)?;

    client.release_funds(transaction.amount)?;
    transaction.dispute_status = TransactionDisputeStatus::NONE;

    Ok(())
}

pub fn revert(client: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::DISPUTED)?;

    client.chargeback_funds(transaction.amount)?;
    client.lock()?;
    transaction.dispute_status = TransactionDisputeStatus::REVERSED;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        decimal::Decimal,
        engine::{deposit::execute, Engine},
        transaction::{Transaction, TransactionType},
    };

    #[test]
    fn correct_execution() {
        let mut e = Engine::new();

        execute(
            &mut e,
            &Transaction::new(TransactionType::DEPOSIT, 1, 1, Decimal::from(100)),
        )
        .unwrap();

        assert_eq!(
            e.get_client(1).unwrap().get_funds().available,
            Decimal::from(100)
        );
    }
}
