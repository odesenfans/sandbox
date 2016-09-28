/**
 * @file hw_lkm.c
 * @brief Hello world Linux Kernel Module (LKM).
 */

#include <linux/kernel.h>
#include <linux/module.h>

int init_module()
{
    printk("Hello world, I'm your first Linux Kernel Module!\n");

    return 0;
}

void cleanup_module()
{
    printk("See you next time!\n");
}
