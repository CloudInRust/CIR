use db::MysqlPool;
use actix::prelude::*;

// Database Connection Executor
pub struct DbExecutor(pub MysqlPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}