Simple Transaction Engine
===

# Assumptions
* Reversing a `withdrawal` will result in a `deposit`
* Disputing a deposit transaction can make an account balance go below 0
* A dispute can be stared only on a withdrawal or despoit
* Overflows and invalid transactions are not handled

# Tech
* rust-analyzer with VSCode - linting and formatting
* serde + csv crates - serialization and reading/writing from/to files
* quickcheck - verifying properties

# Error handling
* fatal errors (like failed IO) will result in a panic as we have no way of recovering
* logic errors inside the transaction engine will cause transaction abortion, but errors won't be propagated or logged (we ignore them)

# Testing & Correctness
* there are unit tests for simple base cases
* asset handling (deposit, withdrawal etc) is checked using `quickcheck` for properties like `deposit(withdrawal(x)) == x`, `anything(lock(x)) -> fail`, etc.
* assuming that the properites above hold in our implementation, we can be sure that the asssets of a client can't go to an invalid state (assets can be manipulated only through methods)
* NOTE: we could enforce some corectness properties using the type system (for example, locking an account will convert `Client` to a `LockedClient` struct), but I considered this is too much for this simple example
* `check.py` will run the engine for a few sample inputs (see `sample/`)