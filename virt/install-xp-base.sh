virt-install --name=xp-base --arch=x86_64 --vcpus=1 --ram=512 \
	--os-type=windows --os-variant=winxp --connect=qemu:///system \
	--network network=default \
	--cdrom=/home/vmiklos/virt/xp-base/WindowsXPProfessional_HOME.iso \
	--disk path=/home/vmiklos/virt/xp-base/xp-base.img,size=20,bus=virtio,format=qcow2 \
	--disk path=/home/vmiklos/virt/xp-base/virtio-win-1.1.16.vfd,device=floppy \
	--accelerate --vnc --noautoconsole --keymap=hu
