#!/bin/bash -x

sudo systemd-nspawn --bind /usr/bin/qemu-ppc64 --bind $HOME/git:$HOME/git -D .
