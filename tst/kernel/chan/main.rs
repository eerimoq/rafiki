#![no_std]
#![feature(concat_idents)]
#![feature(collections)]

extern crate collections;

use collections::vec::Vec;

/**
 * @section License
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2017, Erik Moqvist
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * This file is part of the Rafiki project.
 */

#[macro_use] extern crate rafiki;

use rafiki::kernel::{errno, sys, time};
use rafiki::kernel::chan::Channel;
use rafiki::sync::{chan, event, queue};
use rafiki::debug::harness::Harness;

testcase_define!(test_poll);
fn test_poll_impl(_: *mut Harness) -> rafiki::Res
{
    let timeout = sys::Time { seconds: 0, nanoseconds: 100 };
    let (queue_tx, queue_rx) = queue::new(Some(32));
    let (event_tx, event_rx) = event::new();
    let list: chan::List();

    /* Add both channels to the channel list. */
    list.add(queue_rx.clone());
    list.add(event_rx.clone());

    println!("1. Writing to the queue channel.");
    assert!(queue_tx.write(&[2, 1, 0]) == Ok(3));

    loop {
        println!("Polling...");

        match list.poll(&Some(timeout)) {

            Ok(0) => {
                println!("2. Reading from the queue channel.");
                let mut buf: [u8; 3] = [0; 3];
                assert!(queue_rx.read(&mut buf) == Ok(3));
                assert!(buf == [2, 1, 0]);
            },

            Ok(1) => {
                println!("4. Reading from the event channel.");
                assert!(event_rx.read(0x1) == Ok(0x1));
            },

            Err(errno::ETIMEDOUT) => {
                println!("3. Timeout. Writing to the event channel.");
                assert!(event_tx.write(0x1) == Ok(4));
            },

            _ => {
                unreachable!();
            }
        }
    }
}

#[no_mangle]
pub fn main()
{
    let mut harness: Harness = Default::default();
    let mut harness_testcases = [
        testcase!(Some(test_poll), "test_poll"),
        testcase!(None, "")
    ];

    sys::start();
    uart::init();

    harness.init();
    harness.run(&mut harness_testcases);
}
