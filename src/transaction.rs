use serde::{Deserialize, Serialize};

use crate::{client::ClientId, decimal::Decimal, errors::EngineError};

pub type TransactionId = u32;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    DISPUTE,
    RESOLVE,
    CHARGEBACK,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionDisputeStatus {
    NONE,
    DISPUTED,
    REVERSED,
}

impl TransactionDisputeStatus {
    fn default() -> TransactionDisputeStatus {
        TransactionDisputeStatus::NONE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub kind: TransactionType,
    pub client: ClientId,
    pub tx: TransactionId,
    #[serde(default = "Decimal::zero")]
    pub amount: Decimal<4>,

    #[serde(skip)]
    #[serde(default = "TransactionDisputeStatus::default")]
    pub dispute_status: TransactionDisputeStatus,
}

impl Transaction {
    pub fn new(
        kind: TransactionType,
        client: ClientId,
        tx: TransactionId,
        amount: Decimal<4>,
    ) -> Self {
        Self {
            kind: kind,
            client: client,
            tx: tx,
            amount: amount,
            dispute_status: TransactionDisputeStatus::default(),
        }
    }

    pub fn assure_status(&self, status: TransactionDisputeStatus) -> Result<(), EngineError> {
        if self.dispute_status == status {
            Ok(())
        } else {
            Err(EngineError::TransactionInvalidStatus(self.tx))
        }
    }
}
