wget http://download.opensuse.org/distribution/11.4/iso/openSUSE-11.4-NET-i586.iso
virt-install --name=opensuse --arch=x86_64 --vcpus=1 --ram=768 --os-type=linux \
	--os-variant=virtio26 --connect=qemu:///system --network network=default \
	--cdrom=/home/vmiklos/virt/opensuse/openSUSE-11.4-NET-i586.iso \
	--disk path=/home/vmiklos/virt/opensuse/opensuse.img,size=8 --accelerate --vnc \
	--noautoconsole
