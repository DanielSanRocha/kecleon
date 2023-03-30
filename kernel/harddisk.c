#include "memory.h"

#define ATA_IO 0x1F0

unsigned char ATA_PRIMARY_DETECTED;
unsigned char ATA_DRIVE;

void ata_wait_drq() {
    while(c_inb(ATA_IO + 7) & 0x40 == 0) {}
}

void ata_wait_bsy() {
    while(c_inb(ATA_IO + 7) & 0x80 != 0) {}
}

void ata_read_sectors(unsigned int lba, unsigned char sector_count) {
	ata_wait_bsy();
    ata_wait_drq();

	c_outb(ATA_IO + 6,0xE0 | ((lba >>24) & 0xF));
	c_outb(ATA_IO + 2,sector_count);
	c_outb(ATA_IO + 3, (unsigned char) lba);
	c_outb(ATA_IO + 4, (unsigned char)(lba >> 8));
	c_outb(ATA_IO + 5, (unsigned char)(lba >> 16));
	c_outb(ATA_IO + 7,0x20);

	// unsigned int* target = (unsigned int*) ptr;
    // unsigned int* data = (unsigned int*) ATA_IO;

	// for (int j = 0;j < sector_count; j++)
	// {
	// 	ata_wait_bsy();
	// 	ata_wait_drq();

	// 	for(int i=0;i<256;i++) target[i] = *data;
	// 	target+=256;
	// }
}