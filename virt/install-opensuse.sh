#wget http://download.opensuse.org/distribution/11.4/iso/openSUSE-11.4-NET-i586.iso
virt-install --name=opensuse --arch=x86_64 --vcpus=1 --ram=8192 --os-type=linux \
	--os-variant=sles15 --connect=qemu:///system --network network=default \
	--cdrom=/home/vmiklos/virt/opensuse/opensuse.iso \
	--disk path=/home/vmiklos/virt/opensuse/opensuse.img,size=16 --accelerate --vnc \
	--noautoconsole
