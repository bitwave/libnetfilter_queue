extern crate libc;
extern crate libnetfilter_queue as nfq;

use libc::*;
use std::ptr::null;
use std::mem;
use nfq::nfq_q_handle;
use nfq::handle::{Handle, ProtocolFamily};
use nfq::queue::{Queue, CopyMode};
use nfq::message::Message;
use nfq::message::verdict::{set_verdict, Verdict};

fn main() {
    let mut void = Void;
    let mut handle = Handle::new().ok().unwrap();

    handle.bind(ProtocolFamily::INET);
    let mut queue = handle.queue::<Void>(0, packet_handler, void).ok().unwrap();
    queue.mode(CopyMode::Packet(4096)).ok().unwrap();
    println!("Set copy mode");

    println!("Listen for packets...");
    handle.start(4096);

    println!("Finished...");
}

fn packet_handler(qh: *mut nfq_q_handle, mut message: Message, data: &mut Void) -> i32 {
    let id = message.header.id();
    println!("Handline packet (ID: {})", id);

    let NULL: *const c_uchar = null();
    match set_verdict(qh, id, Verdict::Accept, 0, NULL) {
        Ok(r) => { println!("Verdict set: {}", r); r },
        Err(_) => -1
    }
}

struct Void;
