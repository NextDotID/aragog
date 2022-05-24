#![allow(clippy::missing_const_for_fn)]
use crate::Error;

/// Output of a [`TransactionDatabaseConnection`]`::safe_execute`
///
/// [`TransactionDatabaseConnection`]: crate::transaction::TransactionDatabaseConnection
#[must_use]
pub enum TransactionOutput<T> {
    /// The transaction was committed
    Committed(T),
    /// The transaction was aborted due to an error
    Aborted(Error),
}

impl<T> TransactionOutput<T> {
    /// Was the transaction committed
    #[inline]
    pub const fn is_committed(&self) -> bool {
        matches!(self, TransactionOutput::Committed(_))
    }

    /// Was the transaction aborted
    #[inline]
    pub fn is_aborted(&self) -> bool {
        !self.is_committed()
    }

    /// Returns the contained [`Self::Committed`] value, consuming the self value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Self::Aborted`].
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Committed(v) => v,
            Self::Aborted(err) => panic!(
                "called `TransactionOuput::unwrap()` on an `Aborted` value {}",
                err
            ),
        }
    }

    /// transform the output to a `Option<T>`
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Committed(v) => Some(v),
            Self::Aborted(_) => None,
        }
    }

    /// transform the output to a `Option<Error>`
    #[inline]
    pub fn err(self) -> Option<Error> {
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
    /// [Committed]: Self::Committed
    /// [Aborted]: Self::Aborted
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            TransactionOutput::Committed(v) => v,
            TransactionOutput::Aborted(e) => panic!("{}: {:?}", msg, e),
        }
    }
}

impl<T> From<TransactionOutput<T>> for Result<T, Error> {
    fn from(output: TransactionOutput<T>) -> Self {
        match output {
            TransactionOutput::Committed(v) => Ok(v),
            TransactionOutput::Aborted(err) => Err(err),
        }
    }
}
