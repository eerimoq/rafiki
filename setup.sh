#
# Setup script for the Pumbaa development environment. Source this file
# in your shell to setup the environment.
#

export RAFIKI_ROOT=$(readlink -f .)
export SIMBA_ROOT=$(readlink -f simba)
export PATH=${PATH}:$(readlink -f simba/bin)

source ~/.cargo/env
