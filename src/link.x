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

    .data : {
      _sdata = .;
      *(.data .data.*);
      _edata = .;
    } > SRAM AT > FLASH  /* setting VMA to RAM and LMA to FLASH */

    .rodata : {
      KEEP(*(.rodata .rodata.*));
    } > FLASH

    .bss : {
      _sbss = .;
      *(.bss .bss.*);
      _ebss = .;
    } > SRAM

    /DISCARD/ :{
      *(.ARM.exidx .ARM.exidx.*);
    }

}