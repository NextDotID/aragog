use arangors::document::options::{InsertOptions, RemoveOptions, UpdateOptions};

#[derive(Clone, Debug)]
/// Struct defining some options for database `write` operations (create, update, delete)
pub struct OperationOptions {
    /// Defines if aragog should wait for the operation to be written on disk
    ///
    /// If set on `true` the requests might be slower. By default, the collection behavior is picked
    pub wait_for_sync: Option<bool>,
    /// Defines if aragog should ignore the ArangoDB document revision system (`_rev` field)
    ///
    /// If set on `false` the requests might be slower. By default, `true` is used as it is the
    /// default ArangoDB behaviour
    pub ignore_revs: bool,
    /// Defines if the operation should ignore [`Record`] hooks. By default set to `true`
    ///
    /// [`Record`]: trait.Record.html
    pub ignore_hooks: bool,
}

impl OperationOptions {
    /// Sets the `wait_for_sync` value
    pub fn wait_for_sync(mut self, value: bool) -> Self {
        self.wait_for_sync = Some(value);
        self
    }

    /// Sets the `ignore_revs` value
    pub fn ignore_revs(mut self, value: bool) -> Self {
        self.ignore_revs = value;
        self
    }

    /// Sets the `ignore_hooks` value
    pub fn ignore_hooks(mut self, value: bool) -> Self {
        self.ignore_hooks = value;
        self
    }
}

impl Default for OperationOptions {
    fn default() -> Self {
        Self {
            wait_for_sync: None, // We keep it at None to use the collection value
            ignore_revs: true,
            ignore_hooks: false,
        }
    }
}

impl Into<InsertOptions> for OperationOptions {
    fn into(self) -> InsertOptions {
        // TODO: remove this builder pattern whe arangors resolves https://github.com/fMeow/arangors/issues/71
        if let Some(value) = self.wait_for_sync {
            InsertOptions::builder()
                .wait_for_sync(value)
                .return_new(true) // TODO: allow customization on this option
                .return_old(false)
                .silent(false)
                .build()
        } else {
            InsertOptions::builder()
                .return_new(true) // TODO: allow customization on this option
                .return_old(false)
                .silent(false)
                .build()
        }
    }
}

impl Into<UpdateOptions> for OperationOptions {
    fn into(self) -> UpdateOptions {
        // TODO: remove this builder pattern whe arangors resolves https://github.com/fMeow/arangors/issues/71
        if let Some(value) = self.wait_for_sync {
            UpdateOptions::builder()
                .keep_null(true)
                .wait_for_sync(value)
                .ignore_revs(self.ignore_revs)
                .return_new(true) // TODO: allow customization on this option
                .return_old(false)
                .silent(false)
                .build()
        } else {
            UpdateOptions::builder()
                .keep_null(true)
                .ignore_revs(self.ignore_revs)
                .return_new(true) // TODO: allow customization on this option
                .return_old(false)
                .silent(false)
                .build()
        }
    }
}

impl Into<RemoveOptions> for OperationOptions {
    fn into(self) -> RemoveOptions {
        // TODO: remove this builder pattern whe arangors resolves https://github.com/fMeow/arangors/issues/71
        if let Some(value) = self.wait_for_sync {
            RemoveOptions::builder()
                .wait_for_sync(value)
                .return_old(false)
                .silent(true) // On deletion we don't need meta data
                .build()
        } else {
            RemoveOptions::builder()
                .return_old(false)
                .silent(true) // On deletion we don't need meta data
                .build()
        }
    }
}
