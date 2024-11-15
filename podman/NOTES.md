# podman notes

## docker

### hello world

https://www.collaboraoffice.com/code/quick-tryout-nextcloud-docker/ or in general: https://www.collaboraoffice.com/code/

- my ip: 192.168.0.4

- docker run -d -p 80:80 nextcloud

- http://192.168.0.4/ nextcloud setup

- install Online nextcloud app

- `docker run -t -d -p 9980:9980 -e "extra_params=--o:ssl.enable=false" collabora/code`

... wait till 'docker container logs -f <hash>' indicates it started ...

- in nextcloud settings, set the Online server to http://192.168.0.4:9980

### stop instances

docker container ls
docker container stop <hash> (for each)
docker container rm <hash> (for each)

docker image ls
docker image rm <hash> (for each)

### clean up the rest

systemctl stop docker.service
rm -rf /var/lib/docker

## create containers

```
podman run --name ubuntu1804 --hostname ubuntu1804 -v $HOME:$HOME -ti ubuntu:18.04
podman run --name ubuntu2204 --hostname ubuntu2204 -v $HOME:$HOME -ti ubuntu:22.04
podman run --name centos7 --hostname centos7 -v $HOME:$HOME -ti centos:centos7
podman run --name almalinux8 --hostname almalinux8 -v $HOME:$HOME -ti almalinux:8
podman run --name fedora39 --hostname fedora39 -v $HOME:$HOME -ti --publish 8000:8000 fedora:39
podman run --name fedora40 --hostname fedora40 -v $HOME:$HOME -ti fedora:40
podman run --name fedora41 --hostname fedora41 -v $HOME:$HOME -ti --publish 8000:8000 fedora:41
podman run --cap-add=SYS_ADMIN --name fedora39root --hostname fedora39root -v $HOME:$HOME --publish 12345:12345 -ti fedora:39
podman run --name opensuse156 --hostname opensuse156 -v $HOME:$HOME -ti opensuse/leap:15.6
podman run --name sphinx --hostname sphinx -v $HOME:$HOME -ti python:3.9.12-slim-bullseye bash
```

then

```
podman start -ai ubuntu1804
podman start -ai ubuntu2204
podman start -ai centos7
podman start -ai almalinux8
podman start -ai fedora39
podman start -ai fedora40
podman start -ai fedora41
podman start -ai fedora39root
podman start -ai opensuse156
podman start -ai sphinx
```

is a stateful container.

## list all containers

```
podman container ls --all
```
