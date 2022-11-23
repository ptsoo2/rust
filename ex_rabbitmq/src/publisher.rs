use std::{
    future::Future,
    pin::Pin,
    thread::{self},
    time::Duration,
};

use crate::message::Message;

use super::context::MQContext;
use ex_common::log;
use ex_util::{
    shared_raw_ptr::TSharedMutPtr,
    stop_handle::{StopHandle, StopToken},
    thread_job_queue::ThreadJobQueueSpin,
};
use tokio::task::JoinHandle;

type ContextBoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<MQContext>> + Send>>;
type FnRecover = fn() -> ContextBoxFuture;
type MessageQueue = ThreadJobQueueSpin<Message>;


pub struct Publisher {
    stop_handle_: StopHandle,
    fn_recover_: FnRecover,
    join_handle_: Option<JoinHandle<()>>,
    message_queue_: MessageQueue, // todo! message
}

struct Inner {
    stop_token_ :StopToken,
    fn_recover_: FnRecover, 
    message_queue_: TSharedMutPtr<MessageQueue>,
}

impl Publisher {
    pub fn new(fn_recover: FnRecover) -> Self {
        Self {
            stop_handle_: StopHandle::new(),
            fn_recover_: fn_recover,
            join_handle_: None,
            message_queue_: ThreadJobQueueSpin::default(),
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let join_handle = tokio::spawn(Self::_run(Inner{
            stop_token_: self.stop_handle_.get_token(),
            fn_recover_: self.fn_recover_.clone(),
            message_queue_: TSharedMutPtr {
                value_: &mut self.message_queue_,
            },
        }));

        self.join_handle_ = Some(join_handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.stop_handle_.stop();
    }

    pub fn publish(&mut self, message: Message) {
        self.message_queue_.push(message);
    }

    async fn _run(inner: Inner) {
        unsafe {
            let inner = inner;
            let message_queue = inner.message_queue_.value_.as_mut().expect("!!!");

            let mut mq_context: Option<MQContext> = None;
            while inner.stop_token_.is_stop() == false {
                // recover
                if mq_context.is_none() == true {
                    match (inner.fn_recover_)().await {
                        Ok(context) => {
                            log!("success recover");
                            mq_context = Some(context);
                        }
                        Err(_) => {
                            log!("failed recover(wait for {} seconds...)", 10);
                            thread::sleep(Duration::from_secs(10));
                        }
                    }
        
                    continue;
                }
        
                // process
                message_queue.swap_conditional();
                let read_queue = message_queue.get_read_queue();
                while read_queue.is_empty() == false {
                    let message = read_queue.front().unwrap().clone();
        
                    match mq_context.as_ref().unwrap().publish(message).await {
                        Ok(_) => {
                            read_queue.pop_back();
                        }
                        Err(_) => {
                            mq_context = None;
                            break;
                        }
                    }
                }
            }
        
            // close context
            if let Some(mut mq_context) = mq_context {
                mq_context.close().await.expect("failed close");
            }
        }
    }
}
