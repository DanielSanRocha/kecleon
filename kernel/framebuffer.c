void* framebuffer_initialize() {
    *(volatile unsigned int *)(0x1000001C) = 0x2CFC; /* timing magic for SVGA 800x600 */
    *(volatile unsigned int *)(0x10120000) = 0x1313a4fe;
    *(volatile unsigned int *)(0x10120004) = 0x0505f67f;
    *(volatile unsigned int *)(0x10120008) = 0x071F1800;
    *(volatile unsigned int *)(0x10120010) = (250 * 1024 * 1024); /* base addr of frame buffer */
    *(volatile unsigned int *)(0x10120018) = 0x82b; /* control bits */

    return (void*) (250 * 1024 * 1024);
}