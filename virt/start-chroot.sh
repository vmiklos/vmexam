#!/bin/bash -x

mount | grep -q $PWD/proc || mount /proc -o bind proc
mount |grep -q $PWD/sys || mount /sys -o bind sys
mount |grep -q $PWD/dev || mount /dev -o bind dev
chroot . /bin/su - vmiklos
umount $PWD/dev
umount $PWD/sys
umount $PWD/proc
