MEMORY {
    FLASH : ORIGIN = 0x08000000, LENGTH = 512k
    RAM : ORIGIN = 0x20000000, LENGTH = 128k

}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_stext = ORIGIN(FLASH) + 0x400;