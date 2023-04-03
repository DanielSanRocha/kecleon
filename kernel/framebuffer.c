#include "mailbox.h"

void* framebuffer_initialize() {
    property_message_tag_t tags[5];

    tags[0].proptag = FB_SET_PHYSICAL_DIMENSIONS;
    tags[0].value_buffer.fb_screen_size.width = 1024;
    tags[0].value_buffer.fb_screen_size.height = 768;
    tags[1].proptag = FB_SET_VIRTUAL_DIMENSIONS;
    tags[1].value_buffer.fb_screen_size.width = 1024;
    tags[1].value_buffer.fb_screen_size.height = 768;
    tags[2].proptag = FB_SET_BITS_PER_PIXEL;
    tags[2].value_buffer.fb_bits_per_pixel = 24;
    tags[3].proptag = NULL_TAG;

     if (send_messages(tags) != 0) {
        return 0;
    }

    // request a framebuffer
    tags[0].proptag = FB_ALLOCATE_BUFFER;
    tags[0].value_buffer.fb_screen_size.width = 0;
    tags[0].value_buffer.fb_screen_size.height = 0;
    tags[0].value_buffer.fb_allocate_align = 16;
    tags[1].proptag = NULL_TAG;


    if (send_messages(tags) != 0) {
        return 0;
    }

    return tags[0].value_buffer.fb_allocate_res.fb_addr;
}