#[allow(unused_imports)]
use std::io::ErrorKind::WouldBlock;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
use std::time::Duration;

use ex_common::bench::bench_multiple;
use ex_common::log;

use ex_database::redis_entry;
use ex_database::redis_value::RedisValue;

use ex_rabbitmq::context::MQContext;

use ex_util::stop_handle::StopHandle;
use ex_util::thread_job_queue::ThreadJobQueueNull;

use lapin::ExchangeKind;

use redis::{Cmd, ConnectionLike, Pipeline, Value};
use std::thread::{self};

use crate::app;

pub fn _test_closure_and_lambda() {
    let vec: Vec<i32> = vec![1, 1];

    log!("{:?}", vec.as_ptr());

    fn lambda_as_ref(vec: &Vec<i32>) {
        log!("{:?}", vec.as_ptr());
    }

    fn lambda_as_move(vec: Vec<i32>) {
        log!("{:?}", vec.as_ptr());
    }

    lambda_as_ref(&vec);
    // lambda_as_move(vec);

    let closure_as_ref = |vec: &Vec<i32>| {
        log!("{:?}", vec.as_ptr());
    };

    let _closure_as_move = |vec: Vec<i32>| {
        log!("{:?}", vec.as_ptr());
    };

    closure_as_ref(&vec);
    // closure_as_move(vec);

    let vec2: Vec<i32> = Vec::new();

    let closure_as_all_ref_capture = || {
        log!("{:?}", vec.as_ptr());
        log!("{:?}", vec2.as_ptr());
    };
    closure_as_all_ref_capture();

    let closure_as_all_move_capture = move || {
        log!("{:?}", vec.as_ptr());
        log!("{:?}", vec2.as_ptr());
    };
    closure_as_all_move_capture();
}

// attempt to add with overflow
pub fn _test_lambda_performance() {
    fn lambda() {
        let mut sum: u32 = 0;
        for idx in 0..10000000 {
            if u32::MAX - sum >= idx {
                sum = 0
            }
            sum += idx;
        }
    }

    lambda();
}

pub fn _test_closure_performance() {
    let closure = || {
        let mut sum: u32 = 0;
        for idx in 0..10000000 {
            if u32::MAX - sum >= idx {
                sum = 0
            }
            sum += idx;
        }
    };

    closure();
}

fn _handle_connection(stream: TcpStream) {
    log!("{:?}", stream);
}

pub fn _test_acceptor() {
    let stop_handle = Arc::new(AtomicBool::new(false));
    let stop_handle_clone = stop_handle.clone();

    ctrlc::set_handler(move || {
        log!("Signal detected!!!!!(request stop)");
        stop_handle_clone.store(true, Release);
    })
    .expect("Error setting Ctrl-C handler");

    let listener = TcpListener::bind("localhost:7878").unwrap();
    while listener.set_nonblocking(true).is_err() {}

    log!("Waiting for Ctrl-C...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => _handle_connection(stream),
            Err(err) => {
                if err.kind() != WouldBlock {
                    log!("leaving loop. error: {}", err);
                    break;
                }
            }
        }

        if stop_handle.load(Acquire) {
            log!("stop!!");
            break;
        }
    }

    log!("Exit!!!");
}

// #[allow(unused)]
// pub fn test_mq_long_body() -> anyhow::Result<()> {
//     let conn_tune = ConnectionTuning::default();

//     let mut conn = Connection::insecure_open_tuned("amqp://admin:admin@localhost:5672", conn_tune)?;
//     let channel = conn.open_channel(Some(1))?;

//     let exchange = channel.exchange_declare(
//         amiquip::ExchangeType::Direct,
//         "rust.direct",
//         amiquip::ExchangeDeclareOptions {
//             durable: false,
//             auto_delete: false,
//             internal: false,
//             arguments: amiquip::FieldTable::new(),
//         },
//     )?;

//     bench_multiple("test", 100000, || {
//         exchange.publish(amiquip::Publish::new(b"hel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel1239072309712309712hel12390hel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo there7230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo there39071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel1239072309712309712hel12390hel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo there7230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo there39071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo therehel123907230971230971239071290319027390127097lo there", "hello"));
//     });
//     Ok(())
// }

// #[allow(unused)]
// pub(crate) fn test_mq_no_context() -> anyhow::Result<()> {
//     let conn_tune = ConnectionTuning::default();

//     let mut conn = Connection::insecure_open_tuned("amqp://admin:admin@localhost:5672", conn_tune)?;
//     let channel = conn.open_channel(Some(1))?;

//     let exchange = channel.exchange_declare(
//         amiquip::ExchangeType::Direct,
//         "rust.direct",
//         amiquip::ExchangeDeclareOptions {
//             durable: false,
//             auto_delete: false,
//             internal: false,
//             arguments: amiquip::FieldTable::new(),
//         },
//     )?;

//     bench_multiple("test", 10000, || {
//         let _ = exchange.publish(amiquip::Publish::new(
//             b"hel123907230971230971239071290319027390127097lo ",
//             "hello",
//         ));
//     });
//     Ok(())
// }

#[allow(unused)]
pub async fn test_mq_publish() -> anyhow::Result<()> {
    let conf = app::get_instance().get_config();

    let mut context = MQContext::new(&conf.mq_conf).await?;
    context
        .channel()
        .await?
        .declare_exchange(1, "game_server.direct", ExchangeKind::Direct)
        .await?;

    bench_multiple("name", 10000, || {
        let _ = context.publish(1, "game_server.direct", "12312312", "12312312132");
    });

    context.close().await?;

    Ok(())
}

#[allow(unused)]
fn test_redis() -> anyhow::Result<()> {
    let connection_info = redis_entry::make_connection_info("localhost", 6379, 1, None, None);

    let pool_config = redis_entry::StubConfig::default();
    let pool = redis_entry::make_pool_default(connection_info, pool_config, None)?;

    let mut conn = pool.get()?;
    {
        let rpy = conn.req_command(Cmd::new().arg("PING"))?;
        if let Value::Status(stat) = rpy {
            println!("{}", stat);
        }
    }
    {
        let a = 1;
    }
    {
        // 와.. 너무 쓰레기같이 써야하네..
        let result: Vec<Value> = Pipeline::with_capacity(3)
            .set("132123", "!@3123")
            .get("132123")
            .zrevrange("test_ranking", 0, -1)
            .query(&mut conn)?;

        let result = result.get(3).unwrap();
        if let Value::Bulk(result) = result {
            println!("{:?}", result);
        }
    }
    Ok(())
}

#[allow(unused)]
fn singleton_test() -> anyhow::Result<()> {
    app::get_instance().init()?;

    // singleton test
    let join_handle = thread::spawn(|| -> anyhow::Result<()> {
        let mut conn = app::get_instance().get_redis_pool(0).unwrap().get()?;

        let rpy = conn.req_command(redis::Cmd::new().arg("GET").arg("1231231231"))?;
        let rpy = RedisValue::new(rpy);

        println!("{}", rpy.is_string());
        println!("{}", rpy.get_string());

        Ok(())
    });
    join_handle.join().unwrap()
}

#[allow(unused)]
fn pointer_test() {
    let mut a = 0;
    let mut b = 0;

    unsafe {
        let mut pa: *mut i32 = &mut a;
        let mut pb: *mut i32 = &mut b;

        println!("{}({:?})", *pa, pa);
        println!("{}({:?})", *pb, pb);

        *pa = 123;
        *pb = 456;

        println!("{}({:?})", *pa, pa);
        println!("{}({:?})", *pb, pb);

        std::mem::swap(&mut pa, &mut pb);

        println!("{}({:?})", *pa, pa);
        println!("{}({:?})", *pb, pb);
    }
}

#[allow(unused)]
pub(crate) fn test_stop_handle(thread_count: usize, with_sec: u64) {
    let mut stop_handle = StopHandle::new();
    let mut vec_handle = Vec::with_capacity(thread_count);
    for idx in 0..thread_count {
        let stop_token = stop_handle.get_token();
        let handle = thread::spawn(move || {
            println!("[{}] thread spawn...", idx);
            while !stop_token.is_stop() {
                std::thread::sleep(Duration::from_millis(1));
            }
            println!("[{}] thread exit...", idx);
        });
        vec_handle.push(handle);
    }

    std::thread::sleep(Duration::from_secs(with_sec));
    stop_handle.stop();
    for handle in vec_handle.into_iter() {
        handle.join().unwrap();
    }
    println!("all thread exit...");
}

#[allow(unused)]
fn test_thread_job_queue_st() {
    let mut thread_job_queue = ThreadJobQueueNull::<i32>::default();

    // publish
    {
        let mut a = 0;
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
    }

    // consume
    {
        thread_job_queue.consume_all(|element| {
            println!("{}", element);
        });
    }
}

// #[allow(unused)]
// pub(crate) fn test_thread_job_queue_mt(publish_thread_count: usize) {
//     let mut thread_job_queue: ThreadJobQueueSpin<String> = ThreadJobQueueBase::default();

//     let mut vec_handle = Vec::with_capacity(publish_thread_count);

//     // publisher
//     for idx in 0..publish_thread_count {
//         let wrapper = TSharedMutPtr::new(&mut thread_job_queue);

//         unsafe {
//             let thread_process = move || {
//                 println!("[{}]spawn publisher", idx);
//                 let wrapper = wrapper;
//                 let queue = wrapper.value_.as_mut().unwrap();

//                 {
//                     let a = SystemTime::now()
//                         .duration_since(UNIX_EPOCH)
//                         .expect("!!")
//                         .as_millis();
//                     srand(a as c_uint);
//                 }

//                 loop {
//                     // random exit
//                     let wait_seconds = (rand() % 5) as u64;
//                     if wait_seconds == 4 {
//                         println!("[{}]exit publisher", idx);
//                         queue.push("-1".to_owned());
//                         break;
//                     }

//                     let system_time: DateTime<chrono::Utc> = SystemTime::now().into();
//                     let value = system_time.format("%Y/%m/%dT%T").to_string();
//                     queue.push(value.clone());
//                     println!("[{}]publish({})", idx, value);
//                     thread::sleep(Duration::from_secs(wait_seconds));
//                 }
//             };

//             vec_handle.push(thread::spawn(thread_process));
//         }
//     }

//     // consumer
//     let wrapper = TSharedMutPtr::new(&mut thread_job_queue);

//     unsafe {
//         let thread_process = move || {
//             let wrapper = wrapper;
//             let queue = wrapper.value_.as_mut().unwrap();

//             let mut exit_count = 0;
//             let mut is_stop = false;
//             while !is_stop {
//                 queue.consume_all(|elem| {
//                     if elem.eq(&"-1".to_owned()) {
//                         exit_count += 1;
//                         if exit_count == publish_thread_count {
//                             is_stop = true;
//                         }
//                     }
//                     println!("consume({})", elem);
//                 });
//             }
//         };
//         vec_handle.push(thread::spawn(thread_process));
//     }

//     for handle in vec_handle.into_iter() {
//         handle.join().unwrap();
//     }
//     println!("all thread exit...");
// }

// #[allow(unused)]
// pub(crate) fn test_thread_job_queue_custom_lock<TLock>(
//     publish_thread_count: usize,
//     mut publish_count: usize,
// ) where
//     TLock: ILockable + 'static,
// {
//     let mut thread_job_queue: ThreadJobQueueBase<String, TLock> = ThreadJobQueueBase::default();

//     let mut vec_handle = Vec::with_capacity(publish_thread_count);

//     // publisher
//     for idx in 0..publish_thread_count {
//         let wrapper = TSharedMutPtr::new(&mut thread_job_queue);

//         unsafe {
//             let thread_process = move || {
//                 let wrapper = wrapper;
//                 let queue = wrapper.value_.as_mut().unwrap();

//                 while publish_count > 0 {
//                     queue.push("12312312312312123".to_owned());
//                     publish_count -= 1;
//                 }
//             };

//             vec_handle.push(thread::spawn(thread_process));
//         }
//     }

//     // consumer
//     let wrapper = TSharedMutPtr::new(&mut thread_job_queue);

//     unsafe {
//         let thread_process = move || {
//             let wrapper = wrapper;
//             let queue = wrapper.value_.as_mut().unwrap();

//             let mut remain_consume_count = publish_thread_count * publish_count;
//             let mut is_stop = false;
//             while remain_consume_count > 0 {
//                 queue.consume_all(|elem| {
//                     remain_consume_count -= 1;
//                 });
//             }
//         };
//         vec_handle.push(thread::spawn(thread_process));
//     }

//     for handle in vec_handle.into_iter() {
//         handle.join().unwrap();
//     }
// }

// #[allow(unused)]
// pub(crate) fn test_thread_job_queue_performance(
//     publish_thread_count: usize,
//     publish_count: usize,
//     loop_count: u32,
// ) {
//     for _ in 0..10 {
//         bench_multiple("spin_mutex", loop_count, || {
//             test_thread_job_queue_custom_lock::<SpinMutexDefault>(
//                 publish_thread_count,
//                 publish_count,
//             );
//         });
//     }
//     for _ in 0..10 {
//         bench_multiple("mutex", loop_count, || {
//             test_thread_job_queue_custom_lock::<MutexDefault>(publish_thread_count, publish_count);
//         });
//     }
// }
