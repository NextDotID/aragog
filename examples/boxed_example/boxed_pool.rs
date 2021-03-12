use std::ops::Deref;

use aragog::DatabaseAccess;

pub struct BoxedPool {
    pub pool: Box<dyn DatabaseAccess>,
}

impl BoxedPool {
    pub fn pool(&self) -> &dyn DatabaseAccess {
        self.pool.deref()
    }
}
