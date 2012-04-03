virt-install --name=w7-base --arch=x86_64 --vcpus=1 --ram=1024 \
	--os-type=windows --os-variant=win7 --connect=qemu:///system \
	--network network=default \
	--disk path=/home/vmiklos/virt/w7-base/w7-base.img,size=30,bus=virtio,format=qcow2 \
	--disk path=/home/vmiklos/virt/w7-base/virtio-win-0.1-22.iso,device=cdrom \
	--cdrom=/home/vmiklos/virt/w7-base/en_windows_7_professional_with_sp1_x86_dvd_u_677056.iso \
	--accelerate --vnc --noautoconsole --keymap=hu
