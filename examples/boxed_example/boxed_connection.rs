use std::ops::Deref;

use aragog::DatabaseAccess;

pub struct BoxedConnection {
    pub connection: Box<dyn DatabaseAccess>,
}

impl BoxedConnection {
    pub fn connection(&self) -> &dyn DatabaseAccess {
        self.connection.deref()
    }
}
