extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_io;
extern crate bytes;

use std::{io, str};
use std::thread;
use std::borrow::Borrow;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::pipeline::ServerProto;
use tokio_proto::TcpServer;
use tokio_service::{Service, NewService};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::{future, Future, Stream, Sink};

use bytes::{BytesMut, BufMut};

// 编码层
pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = src.iter().position(|&b| b == b'\n') {
            let line = src.split_to(i);
            src.split_to(1);

            match str::from_utf8(line.borrow()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend(item.as_bytes());
        dst.put(b'\n');
        Ok(())
    }
}


// 协议层
pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

fn serve<S>(s: S) -> io::Result<()>
where
    S: NewService<Request = String, Response = String, Error = io::Error> + 'static,
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let address = "0.0.0.0:9999".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;

    let connectios = listener.incoming();
    let server = connectios.for_each(move |(socket, _perr_addr)| {
        let (writer, reader) = socket.framed(LineCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses).then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server)
}

// 服务层
pub struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // do somethings here
        Box::new(future::ok(req.chars().rev().collect()))
    }
}

fn main() {
    let old = thread::spawn(move || echo_run());
    let new = thread::spawn(move || serve(|| Ok(Echo)));
    old.join().unwrap();
    new.join().unwrap();
    // if let Err(e) = serve(|| Ok(Echo)) {
    //     println!("Server failed with {}", e);
    // }
}

fn echo_run() {
    println!("hello echo!");

    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);

    server.serve(|| Ok(Echo));
}
