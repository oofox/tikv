#![feature(no_more_cas)]
use grpcio::*;
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use std::sync::RwLock;

extern crate base64;

use std::sync::Arc;
use std::{error, fmt, result};

mod drl;

pub use crate::drl::*;

mod drl_grpc;

pub use crate::drl_grpc::*;

use grpcio::{ChannelBuilder, EnvBuilder};

use crate::EventType::{Pull, Quota};
use futures::{Future, Sink, Stream};
use std::collections::HashMap;

pub struct DRLClient {
    addr: String,
    keys: RwLock<HashMap<Vec<u8>, AtomicU64>>,
}

impl DRLClient {
    pub fn new(addr: String) -> Self {
        DRLClient {
            addr: addr.clone(),
            keys: RwLock::new(HashMap::new()),
        }
    }

    pub fn ack(&mut self, key: &[u8]) {
        let keys = self.keys.read().unwrap();
        for k in keys.keys() {
            let c = &k[..];
            if key.starts_with(c) {
                let v = keys.get(k).unwrap();
                v.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    pub fn run(&self) {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(&self.addr);
        let cli = drl_grpc::DrlClient::new(ch);
        let (mut sink, mut receiver) = cli.connect().unwrap();
        loop {
            match receiver.into_future().wait() {
                Ok((Some(event), r)) => {
                    let event_type = event.get_field_type();
                    if event_type == Pull {
                        // info!("quota pull!");
                        let keys = self.keys.write().unwrap();
                        let k = base64::decode(event.get_key()).unwrap();
                        let c: &[u8] = &k;
                        let o = keys
                            .get(c)
                            .unwrap()
                            .fetch_update(|x| Some(0), Ordering::SeqCst, Ordering::SeqCst)
                            .unwrap();
                        let mut e = Event::new();
                        e.set_key(base64::encode(c));
                        e.set_field_type(Quota);
                        e.set_quota(o as f32);
                        sink = sink.send((e, WriteFlags::default())).wait().unwrap()
                    }
                    if event_type == Quota {
                        // info!("quota settings!");
                        let k = event.get_key();
                        let kk = base64::decode(k).unwrap();
                        self.keys.write().unwrap().insert(kk, AtomicU64::new(0));
                    }
                    receiver = r;
                }
                Ok((None, _)) => break,
                Err((e, _)) => panic!("RouteChat RPC failed: {:?}", e),
            }
        }
    }
}
