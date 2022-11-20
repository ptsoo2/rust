use std::{future::Future, pin::Pin, thread::{ self, }, time::Duration};

use super::context::MQContext;
use anyhow::{bail};
use ex_common::log;
use ex_util::{stop_handle::{StopHandle}, thread_job_queue::ThreadJobQueueSpin, shared_raw_ptr::TSharedMutPtr};
use tokio::task::JoinHandle;

type ContextBoxFuture = Pin<Box<dyn Future<Output = anyhow::Result<MQContext>> + Send>>;
type FnRecover = fn() -> ContextBoxFuture;

pub struct Publisher {
    stop_handle_: StopHandle,
    fn_recover_: FnRecover,
    join_handle_: Option<JoinHandle<()>>,
    message_queue_: ThreadJobQueueSpin<String>, // todo! message
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
        let fn_recover = self.fn_recover_.clone();
        let stop_token = self.stop_handle_.get_token();
        let message_queue = TSharedMutPtr{value_: &mut self.message_queue_};
        log!("{:?}", thread::current().id());
        let thread_process = async move{
            unsafe {
                log!("{:?}", thread::current().id());
                let message_queue_wrapper = message_queue;
                let message_queue = message_queue_wrapper.value_.as_mut().unwrap();

                let mut local_context: Option<MQContext> = fn_recover().await.ok();
                while stop_token.is_stop() == false {
                    // recover
                    if local_context.is_none() == true {
                        log!("{:?}", thread::current().id());
                        match fn_recover().await {
                            Ok(context) => {
                                log!("success recover");
                                local_context = Some(context);
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

                        if let Some(context) = local_context.as_ref() {
                            let publish_result = context.publish(1, "game_server.direct", "12312312123", body).await;
                            match publish_result {
                                Ok(_)=> {
                                    read_queue.pop_back();
                                }
                                Err(_) => {
                                    local_context = None;
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
