use crate::traits::{self, CorePort};

pub struct API<DBPort: traits::DBPort, CorePort: traits::CorePort> {
    pub db: DBPort,
    pub core: CorePort,
}
