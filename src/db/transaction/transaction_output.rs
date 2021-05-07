use crate::ServiceError;

/// Output of a [`TransactionDatabaseConnection`][`safe_execute`]
///
/// [`TransactionDatabaseConnection`]: transaction/struct.TransactionDatabaseConnection.html
/// [`safe_execute`]: transaction/struct.TransactionDatabaseConnection.html#method.safe_execute
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
        matches!(self, TransactionOutput::Committed(_))
    }

    /// Was the transaction aborted
    pub fn is_aborted(&self) -> bool {
        !self.is_committed()
    }

    /// Returns the contained [Committed] value, consuming the self value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [Aborted].
    ///
    /// [Committed]: enum.TransactionOutput.html#variant.Committed
    /// [Aborted]: enum.TransactionOutput.html#variant.Aborted
    pub fn unwrap(self) -> T {
        match self {
            TransactionOutput::Committed(v) => v,
            TransactionOutput::Aborted(err) => panic!(
                "called `TransactionOuput::unwrap()` on an `Aborted` value {}",
                err
            ),
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

    /// Returns the contained [Committed] value, consuming the self value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [Aborted] with a custom panic message provided by msg
    ///
    /// [Committed]: enum.TransactionOutput.html#variant.Committed
    /// [Aborted]: enum.TransactionOutput.html#variant.Aborted
    pub fn expect(self, msg: &str) -> T {
        match self {
            TransactionOutput::Committed(v) => v,
            TransactionOutput::Aborted(e) => panic!("{}: {:?}", msg, e),
        }
    }
}

impl<T> From<TransactionOutput<T>> for Result<T, ServiceError> {
    fn from(output: TransactionOutput<T>) -> Self {
        match output {
            TransactionOutput::Committed(v) => Ok(v),
            TransactionOutput::Aborted(err) => Err(err),
        }
    }
}
