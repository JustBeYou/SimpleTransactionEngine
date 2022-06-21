use crate::{
    client::{Client, ClientId},
    errors::EngineError,
    transaction::{Transaction, TransactionId, TransactionType},
};
use std::collections::{hash_map::Iter, HashMap};

mod chargeback;
mod deposit;
mod dispute;
mod resolve;
mod withdrawal;

pub struct Engine {
    clients: HashMap<ClientId, Client>,
    transactions: HashMap<TransactionId, Transaction>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn execute(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        let result = match transaction.kind {
            TransactionType::DEPOSIT => deposit::execute(self, transaction),
            TransactionType::WITHDRAWAL => withdrawal::execute(self, transaction),
            TransactionType::DISPUTE => dispute::execute(self, transaction),
            TransactionType::RESOLVE => resolve::execute(self, transaction),
            TransactionType::CHARGEBACK => chargeback::execute(self, transaction),
        };

        result
    }

    pub fn get_client<'a>(&'a self, id: ClientId) -> Option<&'a Client> {
        self.clients.get(&id)
    }

    pub fn get_mut_transaction_client_pair<'a>(
        &'a mut self,
        tx: TransactionId,
    ) -> Result<(&'a mut Transaction, &'a mut Client), EngineError> {
        let t = self
            .transactions
            .get_mut(&tx)
            .ok_or(EngineError::TransactionNotFound(tx))?;

        let c = self
            .clients
            .get_mut(&t.client)
            .ok_or(EngineError::ClientNotFound(t.client))?;

        Ok((t, c))
    }

    pub fn iter_clients<'a>(&'a self) -> Iter<'a, ClientId, Client> {
        self.clients.iter()
    }
}
