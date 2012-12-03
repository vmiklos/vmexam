wget http://cdimage.debian.org/debian-cd/6.0.6/powerpc/iso-cd/debian-6.0.6-powerpc-businesscard.iso
qemu-img create -f qcow2 linuxppc.qcow2 8G
ln -s debian-6.0.6-powerpc-businesscard.iso debian-ppc.iso
qemu-system-ppc -hda linuxppc.qcow2 -cdrom debian-ppc.iso -boot d -m 512
