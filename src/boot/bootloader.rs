use core::arch::global_asm;

global_asm!(include_str!("boot.asm"));
global_asm!(include_str!("trap.asm"));
