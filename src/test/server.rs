use std::cell::RefCell;
use std::io::IoResult;
use std::io::net::tcp::{TcpAcceptor,TcpListener};
use std::io::{Acceptor, Listener};

// Contains
local_data_key!(listener: RefCell<TcpListener>)

#[deriving(Clone)]
pub enum Op {
  SendBytes(&'static [u8]),
  ReceiveBytes(&'static [u8]),
  Wait(uint) // ms
}

pub struct OpSequence {
  ops: Vec<Op>
}

impl OpSequence {
  pub fn new(op: Op) -> OpSequence {
    OpSequence { ops: vec!(op) }
  }

  pub fn concat(op: Op, seq: OpSequence) -> OpSequence {
    let mut ops = vec!(op);
    ops.push_all(seq.ops.as_slice());
    OpSequence { ops: ops }
  }
}

pub fn server(ops: OpSequence) -> Receiver<bool> {
  let (tx, rx) = channel();

  spawn(proc() {
    let mut srv = TcpListener::bind("127.0.0.1", 8482).unwrap().listen().unwrap();

    let stream = match srv.accept() {
      Ok(s) => s,
      Err(e) => {
        println!("fail: {}", e);
        return;
      }
    };

    tx.send(true);
  });

  rx
}

fn get_server<'a>() -> &'a TcpListener {
  listener.get().unwrap().deref().borrow_mut().deref()
}
