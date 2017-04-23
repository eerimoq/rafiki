Hello World application
=======================

Let's start with the `Rafiki` "Hello World" application. It
examplifies what an application is and how to build and run it.

It consists of two files; ``main.rs`` and ``Makefile``.

main.rs
-------

:github-blob:`main.rs<examples/hello_world/main.rs>` defines the
application entry function ``main()``.

.. code-block:: rust

   #[macro_use] extern crate rafiki;
   
   use rafiki::kernel::sys;
   
   #[no_mangle]
   pub fn main(_: isize, _: *const *const u8) -> isize
   {
       sys::start();
   
       println!("Hello World!");
   
       0
   }

Makefile
--------

:github-blob:`Makefile<examples/hello_world/Makefile>` contains build
configuration of the application.

.. code-block:: makefile

   NAME = hello_world
   BOARD ?= linux

   RAFIKI_ROOT = ../..
   include $(RAFIKI_ROOT)/make/app.mk

Build and run
-------------

Cross-compile, link and then run on an Arduino Due:

.. code-block:: text

   $ cd examples/hello_world
   $ make -s BOARD=arduino_due run
   <build system output>
   Hello world!
   $
