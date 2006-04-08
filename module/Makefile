# if KERNELRELEASE is defined, we've been invoked from
# the kernel build system and can use its language
ifneq ($(KERNELRELEASE),)
	obj-m := hello.o

# otherwise we were called directly from the command
# line, so we invoke the kernel build system
else
	KERNELDIR ?= /lib/modules/$(shell uname -r)/build
	PWD := $(shell pwd)
default:
	$(MAKE) -C $(KERNELDIR) M=$(PWD) modules
endif
