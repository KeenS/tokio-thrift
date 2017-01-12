// DO NOT EDIT: autogenerated by tokio_thrift
#![allow(dead_code, unused_imports, non_snake_case, non_camel_case_types)]
use futures::{Future, Async, finished};
use futures::future::BoxFuture;
use tokio_thrift::protocol::{ThriftDeserializer, ThriftSerializer, ThriftMessageType};
use tokio_thrift::protocol::{Error, ThriftType, BinaryProtocol};
use tokio_thrift::protocol::{Serializer, Deserializer};
use tokio_thrift::protocol::{Deserialize, Serialize};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::{ServerProto, ClientProto, Pipeline, ClientService};
use tokio_proto::{TcpServer, TcpClient};
use tokio_service::Service;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

pub trait HelloService: Send {
    // FIXME: generate result type
    fn hello_name(&self, name: String) -> BoxFuture<String, ()>;
    // FIXME: generate result type
    fn hello(&self) -> BoxFuture<String, ()>;
}


#[derive(Debug, Clone)]
pub enum HelloServiceMethodArgs {
    Ahello_name(Hellohello_nameArgs),
    Ahello(HellohelloArgs),
}


impl Serialize for HelloServiceMethodArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodArgs::*;
        match self {
            &Ahello_name(ref b) => {
                s.write_message_begin("hello_name", ThriftMessageType::Call)?;
                b.serialize(s)?;
                s.write_message_end()?;
            }
            &Ahello(ref b) => {
                s.write_message_begin("hello", ThriftMessageType::Call)?;
                b.serialize(s)?;
                s.write_message_end()?;
            }
        };
        Ok(())
    }
}

impl Deserialize for HelloServiceMethodArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer
    {
        let msg = de.read_message_begin()?;
        // assert!(msg.type) == $msg_type
        let ret = match msg.name.as_ref() {
            "hello_name" => {
                HelloServiceMethodArgs::Ahello_name(Hellohello_nameArgs::deserialize(de)?)
            }
            "hello" => HelloServiceMethodArgs::Ahello(HellohelloArgs::deserialize(de)?),
            _ => {
                return Err(Error::from(io::Error::new(io::ErrorKind::InvalidData,
                                                      "failed to parse thrift data")))
            }
        };
        let _ = de.read_message_end()?;
        Ok(ret)
    }
}


#[derive(Debug, Clone)]
pub enum HelloServiceMethodReturn {
    // FIXME: generate exception too
    Rhello_name(Result<String, ()>),
    // FIXME: generate exception too
    Rhello(Result<String, ()>),
}


impl Serialize for HelloServiceMethodReturn {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodReturn::*;
        match self {
            &Rhello_name(ref b) => {
                match b {
                    &Ok(ref b) => {
                        s.write_message_begin("hello_name", ThriftMessageType::Reply)?;
                        b.serialize(s)?;
                        s.write_message_end()?;
                    }
                    &Err(_) => panic!("exception is not supported yet"),
                }
            }
            &Rhello(ref b) => {
                match b {
                    &Ok(ref b) => {
                        s.write_message_begin("hello", ThriftMessageType::Reply)?;
                        b.serialize(s)?;
                        s.write_message_end()?;
                    }
                    &Err(_) => panic!("exception is not supported yet"),
                }
            }
        };
        Ok(())
    }
}

impl Deserialize for HelloServiceMethodReturn {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer
    {
        let msg = de.read_message_begin()?;
        // if msg.type == return
        let ret = match msg.name.as_ref() {
            "hello_name" => HelloServiceMethodReturn::Rhello_name(Ok(String::deserialize(de)?)),
            "hello" => HelloServiceMethodReturn::Rhello(Ok(String::deserialize(de)?)),
            _ => {
                return Err(Error::from(io::Error::new(io::ErrorKind::InvalidData,
                                                      "failed to parse thrift data")))
            }
        };
        // else msg.type == exception
        // FIXME:

        let _ = de.read_message_end()?;
        Ok(ret)
    }
}


#[derive(Debug, Clone)]
pub struct Hellohello_nameArgs {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct HellohelloArgs {
}
impl Serialize for Hellohello_nameArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        s.write_struct_begin("Hello_hello_name_Args")?;
        s.write_field_begin("name", ThriftType::String, 1)?;
        self.name.serialize(s)?;
        s.write_field_end()?;
        s.write_field_stop()?;
        s.write_struct_end()?;
        Ok(())
    }
}
impl Serialize for HellohelloArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        s.write_struct_begin("Hello_hello_Args")?;
        s.write_field_stop()?;
        s.write_struct_end()?;
        Ok(())
    }
}



impl Deserialize for Hellohello_nameArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer
    {
        de.read_struct_begin()?;
        let mut name = None;
        loop {
            let scheme_field = de.read_field_begin()?;
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                1 => {
                    if scheme_field.ty == ThriftType::String {
                        name = Some(de.deserialize_str()?);
                    } else {
                        // skip
                    }
                }
                _ => (),// skip
            }
            de.read_field_end()?;
        }
        de.read_struct_end()?;
        let args = Hellohello_nameArgs { name: name.unwrap() };
        Ok(args)
    }
}

impl Deserialize for HellohelloArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer
    {
        de.read_struct_begin()?;

        loop {
            let scheme_field = de.read_field_begin()?;
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {

                _ => (),// skip
            }
            de.read_field_end()?;
        }
        de.read_struct_end()?;
        let args = HellohelloArgs {};
        Ok(args)
    }
}


pub struct HelloClientCodec;

impl Codec for HelloClientCodec {
    type In = HelloServiceMethodReturn;
    type Out = HelloServiceMethodArgs;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let cur = io::Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = Self::In::deserialize(&mut protocol)?;
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to(size as usize);
        Ok(Some(ret))
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut protocol = BinaryProtocol::from(buf);
        msg.serialize(&mut protocol).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))

    }
}


pub struct HelloClientProto;

impl<T: Io + 'static> ClientProto<T> for HelloClientProto {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Transport = Framed<T, HelloClientCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HelloClientCodec))
    }
}


pub struct HelloClient<T: 'static + Io> {
    client: ClientService<T, HelloClientProto>,
}

impl<T: 'static + Io> HelloClient<T> {
    pub fn new(client: ClientService<T, HelloClientProto>) -> Self {
        HelloClient { client: client }
    }
}

impl<T: 'static + Io> HelloService for HelloClient<T> {
    // FIXME: generate result type
    fn hello_name(&self, name: String) -> BoxFuture<String, ()> {
        use thrift::HelloServiceMethodArgs::*;
        use thrift::HelloServiceMethodReturn::*;
        let args = Hellohello_nameArgs { name: name };
        self.client
            .call(Ahello_name(args))
            .then(|ret| match ret {
                Ok(Rhello_name(Ok(s))) => Ok(s),
                Ok(Rhello_name(Err(_))) |
                Err(_) => panic!("exception is not supported yet"),
                Ok(_) => panic!("tokio-thrift internal error. may be a bug"),
            })
            .boxed()
    }
    // FIXME: generate result type
    fn hello(&self) -> BoxFuture<String, ()> {
        use thrift::HelloServiceMethodArgs::*;
        use thrift::HelloServiceMethodReturn::*;
        let args = HellohelloArgs {};
        self.client
            .call(Ahello(args))
            .then(|ret| match ret {
                Ok(Rhello(Ok(s))) => Ok(s),
                Ok(Rhello(Err(_))) |
                Err(_) => panic!("exception is not supported yet"),
                Ok(_) => panic!("tokio-thrift internal error. may be a bug"),
            })
            .boxed()
    }
}

pub struct HelloServerCodec;

impl Codec for HelloServerCodec {
    type In = HelloServiceMethodArgs;
    type Out = HelloServiceMethodReturn;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let cur = io::Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = Self::In::deserialize(&mut protocol)?;
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to(size as usize);
        Ok(Some(ret))
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut protocol = BinaryProtocol::from(buf);
        msg.serialize(&mut protocol).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}


pub struct HelloServerProto;

impl<T: Io + 'static> ServerProto<T> for HelloServerProto {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Transport = Framed<T, HelloServerCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HelloServerCodec))
    }
}

#[derive(Clone)]
pub struct HelloServer<T> {
    inner: T,
}

impl<T: HelloService> HelloServer<T> {
    pub fn new(inner: T) -> Self {
        HelloServer { inner: inner }
    }
}

impl<T> Service for HelloServer<T>
    where T: HelloService
{
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Future = BoxFuture<HelloServiceMethodReturn, io::Error>;


    fn call(&self, req: Self::Request) -> Self::Future {
        use thrift::HelloServiceMethodArgs::*;
        use thrift::HelloServiceMethodReturn::*;
        match req {
            Ahello_name(_args) => {
                self.inner
                    .hello_name(_args.name)
                    .then(|r| finished(Rhello_name(r)))
                    .boxed()
            }
            Ahello(_args) => {
                self.inner
                    .hello()
                    .then(|r| finished(Rhello(r)))
                    .boxed()
            }
        }
    }
}
