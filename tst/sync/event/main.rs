#![no_std]
#![feature(concat_idents)]

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

use rafiki::kernel::sys;
use rafiki::sync::event;
use rafiki::debug::harness;

const EVENT_BIT_0: u32 = 0x1;
const EVENT_BIT_1: u32 = 0x2;
const EVENT_BIT_2: u32 = 0x4;

testcase_define!(test_read_write);
fn test_read_write_impl(_: *mut harness::Harness) -> rafiki::Res
{
    let mut mask: u32;
    let (tx, rx) = event::new();

    /* Write two events. */
    mask = EVENT_BIT_1 | EVENT_BIT_0;
    assert!(tx.write(mask) == Ok(4));

    assert!(rx.size() == Ok(1));

    /* Read first event. */
    mask = EVENT_BIT_0;
    assert!(rx.read(mask) == Ok(EVENT_BIT_0));

    /* Read second event. */
    mask = EVENT_BIT_1 | EVENT_BIT_0;
    assert!(rx.read(mask) == Ok(EVENT_BIT_1));

    /* Write second and third events. */
    mask = EVENT_BIT_2 | EVENT_BIT_1;
    assert!(tx.write(mask) == Ok(4));

    /* Write first event. */
    mask = EVENT_BIT_0;
    assert!(tx.write(mask) == Ok(4));

    /* Read first and second events. */
    mask = EVENT_BIT_1 | EVENT_BIT_0;
    assert!(rx.read(mask) == Ok(EVENT_BIT_1 | EVENT_BIT_0));

    /* Read third event. */
    mask = EVENT_BIT_2;
    assert!(rx.read(mask) == Ok(EVENT_BIT_2));

    assert!(rx.size() == Ok(0));

    Ok(0)
}

#[no_mangle]
pub fn main()
{
    let mut harness: harness::Harness = Default::default();
    let mut harness_testcases = [
        testcase!(Some(test_read_write), "test_read_write"),
        testcase!(None, "")
    ];

    sys::start();

    harness.init();
    harness.run(&mut harness_testcases).unwrap();
}
