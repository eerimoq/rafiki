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

//use alloc::boxed::
use core::mem::size_of;

#[derive(Debug)]
pub struct Writer {
    event_p: *mut ::event_t
}

#[derive(Debug)]
pub struct Reader {
    event_p: *mut ::event_t
}

pub fn new() -> (Writer, Reader) {
    let mut writer: Writer = Writer { event_p: 0 as *mut ::event_t};
    let mut reader: Reader = Reader { event_p: 0 as *mut ::event_t};
    let buf_p;
    let event_p: *mut ::event_t;

    unsafe {
        buf_p = ::kernel::sys::__rust_allocate(size_of::<::event_t>(), 4);
        event_p = buf_p as *mut ::event_t;
        ::event_init(event_p);
    }

    writer.event_p = event_p;
    reader.event_p = event_p;

    (writer, reader)
}

impl Writer {

    pub fn write(&self, mask: u32) -> Result<u32, i32>
    {
        let res;

        unsafe {
            res = ::event_write(self.event_p,
                                &mask as *const _ as *const i32,
                                4);
        }

        match res {
            4 => Ok(4),
            _ => Err(res as i32)
        }
    }
}

impl Reader {

    pub fn read(&self, mut mask: u32) -> Result<u32, i32>
    {
        let res;

        unsafe {
            res = ::event_read(self.event_p,
                               &mut mask as *mut _ as *mut i32,
                               4);
        }

        match res {
            4 => Ok(mask),
            _ => Err(res as i32)
        }
    }

    pub fn size(&self) -> Result<u32, i32>
    {
        let res;

        unsafe {
            res = ::event_size(self.event_p);
        }

        match res {
            size if res >= 0 => Ok(size as u32),
            _ => Err(res as i32)
        }
    }
}

//impl Event {
//
//    pub fn new()
//               -> Box<Event>
//    {
//        let mut event: Box<Event> = Box::new(Event { inner: Default::default() });
//
//        unsafe {
//            ::event_init(&mut event.inner);
//        }
//
//        event
//    }
//
//    pub fn write(&mut self, buf: &[u8]) -> ::Res
//    {
//        unsafe {
//            Ok(::event_write(&mut self.inner as *mut ::Struct_event_t,
//                             buf.as_ptr() as *const i32,
//                             buf.len() as u32))
//        }
//    }
//
//    pub fn read(&mut self, buf: &mut [u8]) -> ::Res
//    {
//        unsafe {
//            Ok(::event_read(&mut self.inner as *mut ::Struct_event_t,
//                            buf.as_ptr() as *mut i32,
//                            buf.len() as u32))
//        }
//    }
//}

//pub trait EventBuffer {
//    fn buf_p(&self) -> *mut ::std::os::raw::c_void;
//    fn len(&self) -> u32;
//}
//
//impl Event {
//
//    pub fn new()
//               -> Event
//    {
//        let mut event: Event = Default::default();
//
//        unsafe {
//            ::event_init(&mut event);
//        }
//
//        event
//    }
//
//    pub fn write(&mut self, value: u32)
//                 -> ::Res
//    {
//        unsafe {
//            ::event_write(self,
//                          &value as *const _ as *const i32,
//                          4);
//        }
//
//        Ok(0)
//    }
//
//    pub fn read(&mut self, value: &mut u32)
//                -> ::Res
//    {
//        unsafe {
//            ::event_read(self,
//                         value as *mut _ as *mut i32,
//                         4);
//        }
//
//        Ok(0)
//    }
//
//    pub fn size(&mut self)
//                -> ::Res
//    {
//        unsafe {
//            Ok(::event_size(self))
//        }
//    }
//}
//
//impl EventBuffer for u32 {
//
//    fn buf_p(&self) -> *mut ::std::os::raw::c_void
//    {
//        self as *const _ as *mut ::std::os::raw::c_void
//    }
//
//    fn len(&self) -> u32
//    {
//        4
//    }
//}
//
//unsafe impl Send for Event {}
//
//impl ::kernel::chan::Channel for Box<Event> {
//
//    fn get_chan_p(&mut self) -> *mut ::std::os::raw::c_void
//    {
//        &mut self.inner.base as *mut _ as *mut ::std::os::raw::c_void
//    }
//
//    fn write(&mut self, buf: &[u8]) -> ::Res
//    {
//        unsafe {
//            self.write(buf)
//        }
//    }
//
//    fn read(&mut self, buf: &mut [u8]) -> ::Res
//    {
//        unsafe {
//            self.read(buf)
//        }
//    }
//}
