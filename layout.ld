ENTRY(_reset);

MEMORY
{
  CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 40K
}

SECTIONS
{
  .text : {
    _stack_top = .;
    /* Vector table */
    LONG(ORIGIN(CCRAM) + LENGTH(CCRAM))
    _reset_vector = .;
    LONG(_reset + 1);
    KEEP(*(.rodata._VECTORS));

    /* Reset handler */
    _reset = .;
    *(.text._reset)

    *(.text*)
  } > FLASH

  .bss(NOLOAD) : {
      *(.bss*)
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.ARM.attributes*)
    *(.note.gnu.build-id*)
  }
}
