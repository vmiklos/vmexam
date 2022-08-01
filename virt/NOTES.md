# virt-manager notes

## Windows 7 vs slow IO

Perhaps
<http://unix.stackexchange.com/questions/47082/how-to-improve-windows-perfomance-when-running-inside-kvm/48584#48584>
is the answer, i.e. set cache to none and io mode to native.

## docker image to chroot with podman

```
sudo podman run -ti --rm --volume=$PWD:/opt/backup centos:centos7 tar -cvf /opt/backup/chroot.tar --exclude=/opt/backup --one-file-system /
```
