; 
; file: long_mode_start.asm
;
; This file is the secondary entrypoint for the kernel. By this point 64-bit instructions should be enabled,
; the initial stack and page tables should be set up, and initial cpu checks should have been done.
;
; notes:
;		original implementation taken from https://github.com/phil-opp/blog_os/src/arch/x86_64/long_mode_start.asm
; 
; owner: gavaga
; date: 2017-08-03
;
; changelog:
;		2017-08-03: file created
;
global long_mode_start

section .text
bits 64
long_mode_start:
	; clear all data segment registers in case it matters
	mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; print `OKAY` to screen
	mov rax, 0x2f592f412f4b2f4f
	mov qword [0xb8000], rax
	hlt
