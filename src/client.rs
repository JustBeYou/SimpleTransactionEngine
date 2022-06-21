use crate::{decimal::Decimal, errors::EngineError};
use serde::{ser::SerializeStruct, Serialize};

pub type ClientId = u16;

#[derive(Debug)]
pub struct Client {
    id: ClientId,
    available: Decimal<4>,
    held: Decimal<4>,
    locked: bool,
}

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Client", 5)?;
        s.serialize_field("client", &self.id)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("held", &self.held)?;
        s.serialize_field("total", &(self.available + self.held))?;
        s.serialize_field("locked", &self.locked)?;
        s.end()
    }
}

pub struct Funds {
    pub available: Decimal<4>,
    pub held: Decimal<4>,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id: id,
            available: Decimal::zero(),
            held: Decimal::zero(),
            locked: false,
        }
    }

    pub fn get_funds(&self) -> Funds {
        Funds {
            available: self.available,
            held: self.held,
        }
    }

    pub fn lock(&mut self) -> Result<(), EngineError> {
        self.not_locked()?;
        self.locked = true;
        Ok(())
    }

    pub fn deposit_funds(&mut self, amount: Decimal<4>) -> Result<Decimal<4>, EngineError> {
        self.not_locked()?;
        Self::amount_not_negative(amount)?;

        self.available += amount;

        Ok(self.available)
    }

    pub fn withdraw_funds(&mut self, amount: Decimal<4>) -> Result<Decimal<4>, EngineError> {
        self.not_locked()?;
        Self::amount_not_negative(amount)?;
        Self::sufficient_funds(self.id, self.available, amount)?;

        self.available -= amount;

        Ok(self.available)
    }

    pub fn hold_funds(&mut self, amount: Decimal<4>) -> Result<Decimal<4>, EngineError> {
        self.not_locked()?;
        Self::amount_not_negative(amount)?;

        self.available -= amount;
        self.held += amount;

        Ok(self.available)
    }

    pub fn release_funds(&mut self, amount: Decimal<4>) -> Result<Decimal<4>, EngineError> {
        self.not_locked()?;
        Self::amount_not_negative(amount)?;
        Self::sufficient_funds(self.id, self.held, amount)?;

        self.available += amount;
        self.held -= amount;

        Ok(self.available)
    }

    pub fn chargeback_funds(&mut self, amount: Decimal<4>) -> Result<Decimal<4>, EngineError> {
        self.not_locked()?;
        Self::amount_not_negative(amount)?;
        Self::sufficient_funds(self.id, self.held, amount)?;

        self.locked = true;
        self.held -= amount;

        Ok(self.available)
    }

    fn amount_not_negative(amount: Decimal<4>) -> Result<(), EngineError> {
        if amount < Decimal::zero() {
            Err(EngineError::NegativeAmount(amount))
        } else {
            Ok(())
        }
    }

    fn sufficient_funds(
        id: ClientId,
        funds: Decimal<4>,
        amount: Decimal<4>,
    ) -> Result<(), EngineError> {
        if funds - amount < Decimal::zero() {
            Err(EngineError::InsufficientFunds(id, funds, amount))
        } else {
            Ok(())
        }
    }

    fn not_locked(&self) -> Result<(), EngineError> {
        if self.locked {
            Err(EngineError::AccountLocked(self.id))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::decimal::Decimal;
    use quickcheck::TestResult;

    #[quickcheck]
    fn deposit_and_withdraw_equals_zero(amount: u32) -> TestResult {
        let mut c = Client::new(0);
        let d = Decimal::<4>::from(amount as i64);

        c.deposit_funds(d).unwrap();
        c.withdraw_funds(d).unwrap();

        TestResult::from_bool(c.get_funds().available == Decimal::zero())
    }

    #[quickcheck]
    fn hold_and_release_equals_zero(amount: u32) -> TestResult {
        let mut c = Client::new(0);
        let d = Decimal::<4>::from(amount as i64);

        c.deposit_funds(d).unwrap();
        c.hold_funds(d).unwrap();
        c.release_funds(d).unwrap();

        TestResult::from_bool(c.get_funds().held == Decimal::zero() && c.get_funds().available == d)
    }

    #[quickcheck]
    fn hold_and_chargeback_equals_zero(amount: u32) -> TestResult {
        let mut c = Client::new(0);
        let d = Decimal::<4>::from(amount as i64);

        c.deposit_funds(d).unwrap();
        c.hold_funds(d).unwrap();
        c.chargeback_funds(d).unwrap();

        TestResult::from_bool(
            c.get_funds().held == Decimal::zero() && c.get_funds().available == Decimal::zero(),
        )
    }

    #[quickcheck]
    fn lock_wont_allow_ops() -> TestResult {
        let mut c = Client::new(0);

        c.lock().unwrap();
        if let Err(_) = c.deposit_funds(Decimal::zero()) {
            TestResult::passed()
        } else {
            TestResult::failed()
        }
    }
}
