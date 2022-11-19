use std::{future::Future, pin::Pin};

use super::context::MQContext;
use anyhow::Ok;

type ContextBoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<MQContext>> + Send>>;
type FnRecover = fn() -> ContextBoxFuture;

pub struct MQRunnerBase {
    #[allow(unused)]
    context_: Option<MQContext>,
    #[allow(unused)]
    fn_recover_: FnRecover,
    // todo! join_handle
}

impl MQRunnerBase {
    pub fn new(fn_recover: FnRecover) -> Self {
        Self {
            context_: None,
            fn_recover_: fn_recover,
        }
    }

    #[allow(unused)]
    pub async fn start() {}

    async fn _recover(&mut self) -> anyhow::Result<()> {
        self.context_ = None; // todo! is required??
        let new_context = (self.fn_recover_)().await?;
        self.context_ = Some(new_context);
        Ok(())
    }
}
