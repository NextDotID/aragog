use crate::ServiceError;

/// Output of a [`TransactionPool`][`safe_execute`]
///
/// [`TransactionPool`]: transaction/struct.TransactionPool.html
/// [`safe_execute`]: transaction/struct.TransactionPool.html#method.safe_execute
#[must_use]
pub enum TransactionOutput<T> {
    /// The transaction was committed
    Committed(T),
    /// The transaction was aborted due to an error
    Aborted(ServiceError),
}

impl<T> TransactionOutput<T> {
    /// Was the transaction committed
    pub fn is_committed(&self) -> bool {
        if let TransactionOutput::Committed(_) = self {
            true
        } else {
            false
        }
    }

    /// Was the transaction aborted
    pub fn is_aborted(&self) -> bool {
        !self.is_committed()
    }

    /// Unwraps the output, returning the result
    ///
    /// # Panics
    ///
    /// The method panics if the transaction was aborted
    pub fn unwrap(self) -> T {
        match self {
            TransactionOutput::Committed(v) => v,
            TransactionOutput::Aborted(err) => panic!("Transaction was aborted: {}", err),
        }
    }

    /// transform the output to a `Option<T>`
    pub fn ok(self) -> Option<T> {
        match self {
            TransactionOutput::Committed(v) => Some(v),
            TransactionOutput::Aborted(_) => None,
        }
    }

    /// transform the output to a `Option<ServiceError>`
    pub fn err(self) -> Option<ServiceError> {
        match self {
            TransactionOutput::Committed(_) => None,
            TransactionOutput::Aborted(err) => Some(err),
        }
    }
}

impl<T> Into<Result<T, ServiceError>> for TransactionOutput<T> {
    fn into(self) -> Result<T, ServiceError> {
        match self {
            TransactionOutput::Committed(v) => Ok(v),
            TransactionOutput::Aborted(err) => Err(err),
        }
    }
}
