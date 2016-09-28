#! /bin/bash

# Create device file
DEVICE_NAME="/dev/myrandom"
if [ ! -c $DEVICE_NAME ]; then
    echo "This demo requires your permission to create file $DEVICE_NAME"
    sudo mknod $DEVICE_NAME c 666 0
    sudo chmod 666 $DEVICE_NAME
fi

# Compile driver
make distclean && make

# Load module
sudo insmod random_lkm.ko
dd if=/dev/myrandom bs=1 count=10
dd if=/dev/myrandom
sudo rmmod random_lkm
# tail /var/log/kern.log
