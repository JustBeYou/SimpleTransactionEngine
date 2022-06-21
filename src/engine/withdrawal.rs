use super::Engine;
use crate::client::Client;
use crate::errors::EngineError;
use crate::transaction::{Transaction, TransactionDisputeStatus};

pub fn execute(e: &mut Engine, transaction: &Transaction) -> Result<(), EngineError> {
    e.clients
        .get_mut(&transaction.client)
        .map_or(
            Err(EngineError::ClientNotFound(transaction.client)),
            |client| client.withdraw_funds(transaction.amount),
        )
        .and_then(|_| Ok(e.transactions.insert(transaction.tx, *transaction)))
        .map(|_| ())
}

pub fn dispute(_: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::NONE)?;

    transaction.dispute_status = TransactionDisputeStatus::DISPUTED;

    Ok(())
}

pub fn resolve(_: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::DISPUTED)?;

    transaction.dispute_status = TransactionDisputeStatus::NONE;

    Ok(())
}

pub fn revert(client: &mut Client, transaction: &mut Transaction) -> Result<(), EngineError> {
    transaction.assure_status(TransactionDisputeStatus::DISPUTED)?;

    client.deposit_funds(transaction.amount)?;
    client.lock()?;
    transaction.dispute_status = TransactionDisputeStatus::REVERSED;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        client::Client,
        decimal::Decimal,
        engine::Engine,
        transaction::{Transaction, TransactionType},
    };

    use super::execute;

    #[test]
    fn correct_execution() {
        let mut e = Engine::new();

        e.clients
            .entry(1)
            .or_insert(Client::new(1))
            .deposit_funds(Decimal::from(100))
            .unwrap();

        execute(
            &mut e,
            &Transaction::new(TransactionType::WITHDRAWAL, 1, 1, Decimal::from(50)),
        )
        .unwrap();

        assert_eq!(
            e.get_client(1).unwrap().get_funds().available,
            Decimal::from(50)
        );
    }
}
