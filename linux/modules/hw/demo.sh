#! /bin/bash

make distclean && make

sudo insmod hw_lkm.ko
sudo rmmod hw_lkm
tail /var/log/kern.log
