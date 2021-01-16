// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_DRL_CONNECT: ::grpcio::Method<super::drl::Event, super::drl::Event> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/drl/connect",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DrlClient {
    client: ::grpcio::Client,
}

impl DrlClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DrlClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn connect_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::drl::Event>, ::grpcio::ClientDuplexReceiver<super::drl::Event>)> {
        self.client.duplex_streaming(&METHOD_DRL_CONNECT, opt)
    }

    pub fn connect(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::drl::Event>, ::grpcio::ClientDuplexReceiver<super::drl::Event>)> {
        self.connect_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Drl {
    fn connect(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::drl::Event>, sink: ::grpcio::DuplexSink<super::drl::Event>);
}

pub fn create_drl<S: Drl + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_DRL_CONNECT, move |ctx, req, resp| {
        instance.connect(ctx, req, resp)
    });
    builder.build()
}
