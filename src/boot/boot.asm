.section .text.boot
.global _start
_start:
  la sp, _stack_end
  jal main
  j .