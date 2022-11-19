use std::{future::Future, pin::Pin};

use super::context::MQContext;
use anyhow::Ok;

type ContextBoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<MQContext>> + Send>>;
type FnRecover = fn() -> ContextBoxFuture;

pub struct RunnerContext {
    #[allow(unused)]
    context_: Option<MQContext>,
    #[allow(unused)]
    fn_recover_: FnRecover,
    // todo! join_handle
}

pub trait MQRunnerBase {
    fn new(fn_recover: FnRecover) -> Self;
}

pub struct Publisher {
    #[allow(unused)]
    context_ :RunnerContext
}

impl MQRunnerBase for Publisher{
    fn new(fn_recover: FnRecover) ->Self {
        Self {
            context_: RunnerContext { context_: None, fn_recover_: fn_recover }
        }
    }
}

impl RunnerContext {
    #[allow(unused)]
    pub async fn start() {}

    async fn _recover(&mut self) -> anyhow::Result<()> {
        self.context_ = None; // todo! is required??
        let new_context = (self.fn_recover_)().await?;
        self.context_ = Some(new_context);
        Ok(())
    }
}
