use std::{error::Error, fmt::Display};

use crate::{client::ClientId, decimal::Decimal, transaction::TransactionId};

#[derive(Debug)]
pub enum EngineError {
    ClientNotFound(ClientId),
    TransactionNotFound(TransactionId),
    TransactionInvalidStatus(TransactionId),
    InsufficientFunds(ClientId, Decimal<4>, Decimal<4>),
    AccountLocked(ClientId),
    NegativeAmount(Decimal<4>),
    InvalidTransactionType,
    IOError(&'static str),
    DeserializationError(&'static str),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EngineError::ClientNotFound(c) => write!(f, "Client with id {} not found.", c),
            EngineError::TransactionNotFound(tx) => {
                write!(f, "Transaction with id {} not found.", tx)
            }
            EngineError::InsufficientFunds(c, balance, amount) => write!(
                f,
                "Client {} balance (= {}) < requested amount (= {})",
                c, balance, amount
            ),
            EngineError::AccountLocked(c) => {
                write!(f, "Client with id {} has the account locked.", c)
            }
            EngineError::NegativeAmount(a) => {
                write!(f, "Amount {} must be greater or equal to zero.", a)
            }
            EngineError::InvalidTransactionType => write!(f, "Invalid transaction type"),
            EngineError::IOError(m) => write!(f, "IO Error: {}", m),
            EngineError::DeserializationError(m) => write!(f, "Deserialization error: {}.", m),
            EngineError::TransactionInvalidStatus(id) => {
                write!(f, "Invalid transaction {} status.", id)
            }
        }
    }
}

impl Error for EngineError {}
