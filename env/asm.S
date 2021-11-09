.global main
main:
  li t0, 2021
  sw t0, -8(sp)
  add sp, sp, -8
  lw a0, 0(sp)
  add sp, sp, 8
  ret
