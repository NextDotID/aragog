use crate::{DatabaseRecord, Error, Record};

/// The `AuthorizeAction` trait of the Aragog library.
/// This traits allows provides the ability to authorize a [`Record`] to execute a custom action on
/// an other one.
///
/// # Example
/// ```rust
/// # use aragog::{AuthorizeAction, DatabaseRecord, Record, Validate};
/// # use serde::{Deserialize, Serialize};
/// #
/// #[derive(Serialize, Deserialize, Clone, Record, Validate)]
/// pub struct Employee {
///     pub is_cook: bool,
///     pub is_accountant: bool,
/// }
///
/// #[derive(Serialize, Deserialize, Clone, Record, Validate)]
/// pub struct Company {
///     pub taxes_payed: bool,
///     pub is_cooking_done: bool,
/// }
///
/// pub enum EmployeeAction {
///     Cook,
///     PayTaxes
/// }
///
/// impl AuthorizeAction<Company> for Employee {
/// type Action = EmployeeAction;
///
///     fn is_action_authorized(&self, action: Self::Action, target: Option<&DatabaseRecord<Company>>) -> bool {
///         if target.is_none() { return false; }
///         let target = target.unwrap();
///         match action {
///             EmployeeAction::Cook => self.is_cook && !target.is_cooking_done,
///             EmployeeAction::PayTaxes => self.is_accountant && !target.taxes_payed,
///         }
///     }
/// }
/// ```
///
/// [`Record`]: trait.Record.html
pub trait AuthorizeAction<T: Record> {
    /// The action type to be authorized, like a custom enum of ACL actions (write, read) or more
    /// logic operations.
    type Action;

    /// If the object is authorized to do `action` on `target` then the method will return `Ok(())`,
    /// otherwise an [`Error`]::[`Forbidden`] is returned.
    ///
    /// [`Error`]: enum.Error.html
    /// [`Forbidden`]: enum.Error.html#variant.Forbidden
    fn authorize_action(
        &self,
        action: Self::Action,
        target: Option<&DatabaseRecord<T>>,
    ) -> Result<(), Error> {
        if self.is_action_authorized(action, target) {
            return Ok(());
        }
        Err(Error::Forbidden(None))
    }

    /// Returns true if the object is authorized to do `action` on `target`
    fn is_action_authorized(
        &self,
        action: Self::Action,
        target: Option<&DatabaseRecord<T>>,
    ) -> bool;
}
