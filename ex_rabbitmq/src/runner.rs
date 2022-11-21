use std::{future::Future, pin::Pin, thread::{ self, }, time::Duration};

use super::context::MQContext;
use anyhow::{bail};
use ex_common::log;
use ex_util::{stop_handle::{StopHandle, StopToken}, thread_job_queue::ThreadJobQueueSpin, shared_raw_ptr::TSharedMutPtr};
use tokio::task::JoinHandle;

type ContextBoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<MQContext>> + Send>>;
type FnRecover = fn() -> ContextBoxFuture;
type MessageQueue = ThreadJobQueueSpin<String>;

pub struct Publisher {
    stop_handle_: StopHandle,
    fn_recover_: FnRecover,
    join_handle_: Option<JoinHandle<()>>,
    message_queue_: MessageQueue, // todo! message
}

pub struct Inner {
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

    pub fn publish(&mut self, body: String) {
       self.message_queue_.push(body);
    }

    pub fn stop() { todo!() }
    pub fn close(&self) {todo!()}

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let inner = Inner{
            stop_token_: self.stop_handle_.get_token(),
            fn_recover_: self.fn_recover_.clone(),
            message_queue_: TSharedMutPtr { value_: &mut self.message_queue_ },
        };
        
        let thread_process = async move{
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
                            },
                            Err(_) => {
                                log!("failed recover(wait for {} seconds...)", 10);
                                thread::sleep(Duration::from_secs(10));
                            },
                        }

                        continue;
                    }

                    // process
                    message_queue.swap_conditional();
                    let read_queue = message_queue.get_read_queue();
                    while read_queue.is_empty() == false {
                        let body = read_queue.front().unwrap();

                        if let Some(context) = mq_context.as_ref() {
                            let publish_result = context.publish(1, "game_server.direct", "12312312123", body).await;
                            match publish_result {
                                Ok(_)=> {
                                    read_queue.pop_back();
                                }
                                Err(_) => {
                                    mq_context = None;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        };

        self.join_handle_ = Some(tokio::spawn(thread_process));
        
        Ok(())
    }

    pub async fn _recover(&mut self, mut retry_count: u32) -> anyhow::Result<MQContext> {
        while retry_count > 0 {
            let context = (self.fn_recover_)().await;
            if context.is_ok() {
                return Ok(context.unwrap());
            }
            
            retry_count -=1;
            log!("failed recover(remain_count: {})", retry_count);
        }
        bail!("failed recover");
    }
}

// pub trait MQRunnerBase {
//     fn new(fn_recover: FnRecover) -> Self;
// }

// impl MQRunnerBase for Publisher {
//     fn new(fn_recover: FnRecover) -> Self {
//         Self {
//             context_: RunnerContext {
//                 context_: None,
//                 fn_recover_: fn_recover,
//             },
//         }
//     }
// }
