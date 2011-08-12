qemu-img create -f qcow2 frugalware-current.img 40G
qemu-system-ppc -m 1G -kernel vmlinux -initrd initrd-ppc.img -append 'initrd=initrd-ppc.img.gz load_ramdisk=1 prompt_ramdisk=0 raisk_size=47076 rw root=/dev/ram' -hda frugalware-current.img
