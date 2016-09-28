ifneq ($(KERNELRELEASE),)
# kbuild part of makefile
obj-m  := myrandom.o

else
# normal makefile
    KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

clean:
	@rm -rf *.mod.c *.o .*.o .*.cmd .*ko.cmd *.order *.symvers .tmp_versions

distclean: clean
	@rm -rf *.ko

endif
