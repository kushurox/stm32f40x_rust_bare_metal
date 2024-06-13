ENTRY(_reset)
EXTERN(RESET_HANDLER) /* forces the symbol to not be skipped */


MEMORY {
  FLASH(rx) : ORIGIN = 0x08000000 , LENGTH = 256K
  SRAM(rwx) : ORIGIN = 0x20000000 , LENGTH = 64K
}





SECTIONS {
    .vectors : {
        LONG(ORIGIN(SRAM) + LENGTH(SRAM));  /* setting stack pointer to end of ram as it decrements on push as per data sheet */
        KEEP(*(.vectors.reset_handler));
        KEEP(*(.vectors))   /* for future when declaring more from rust */
    } > FLASH

    .text : {
      *(.text .text.*)
    } > FLASH

    .rodata : {
      KEEP(*(.rodata .rodata.*));
    } > FLASH

    .bss : {
      _sbss = .;
      KEEP(*(.bss .bss.*));
      _ebss = .;
    } > SRAM

    .data :{
      _sdata = .;
      KEEP(*(.data .data.*));
      _edata = .;
    } > SRAM AT>FLASH

     _sidata = LOADADDR(.data);

    /DISCARD/ :{
      *(.ARM.exidx .ARM.exidx.*);
      *(.comment)
    }

}