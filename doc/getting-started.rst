Getting Started
===============

Installation
------------

#. Install Rust. Change the ``default toolchain`` to ``nightly``. Copy
   ``default host triple`` and ``modify PATH variable`` from the
   printed ``Current installtion options``.

   .. code-block:: text

      curl https://sh.rustup.rs -sSf | sh

#. Source the environment setup file.

   .. code-block:: text

      source $HOME/.cargo/env

#. Install `bindgen` and download the Rust sources.

   .. code-block:: text

      cargo install bindgen
      rustup component add rust-src

#. Recursively clone the Rafiki repository.

   .. code-block:: text

      git clone --recursive https://github.com/eerimoq/rafiki

#. Build and run the hello world example.

   .. code-block:: text

      cd rafiki
      source setup.sh
      cd examples/hello_world
      make -s -j4 BOARD=arduino_due run
