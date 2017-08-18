; 
; file: boot.asm
;
; This file is the intial entrypoint of the operating system. It does a couple checks to make
; sure that the CPU supports all the features we need, and then sets up the initial page tables,
; enabled paging, enabled 64-bit instructions, and jumps to the 64-bit entrypoint.
;
; notes:
;		original implementation taken from https://github.com/phil-opp/blog_os/src/arch/x86_64/boot.asm
; 
; owner: gavaga
; date: 2017-08-03
;
; changelog:
;		2017-08-03: file created
;
global start
extern long_mode_start

section .text
bits 32
start:
	; setup the stack
	mov esp, stack_top
	mov edi, ebx			; move multiboot info pointer to edi

	; cpu feature checks
	call check_multiboot
	call check_cpuid
	call check_long_mode

	call set_up_page_tables
	call enable_paging

	lgdt [gdt64.pointer]

	jmp gdt64.code:long_mode_start

	; print 'OK' to the screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

check_multiboot:
	; check whether multiboot is enabled on this processor
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

check_cpuid:
	; check whether cpuid is supported on this processor by attempting to
	; flip the ID bit (bit 21) in the FLAGS register. if we can flip it, 
	; CPUID is available.

	; Copy FLAGS in to EAX via stack
	pushfd
	pop eax

	; copy to ecx as well for comparing later on
	mov ecx, eax

	; flip the ID bit
	xor eax, 1 << 21

	; copy eax to fLAGS via the stack
	push eax
	popfd

	;copy flags back to eax (with the flipped bit if cpuid is supported)
	pushfd
	pop eax

	; restore flags from the old version stored in ecx (i.e. flipping the
	; id bit back if it was ever flipped).
	push ecx
	popfd

	; Compare eax and ecx. if they are equal then that means the bit
	; wasn't flipped, and cpuid isn't supported.
	cmp eax, ecx
	je .no_cpuid
	ret
.no_cpuid:
	mov al, "1"
	jmp error

check_long_mode:
	; test if extended processor info is available
	mov eax, 0x80000000		; implicit argument for cpuid
	cpuid					; get highest supported argument
	cmp eax, 0x80000001		; it needs to be at least 0x80000001
	jb .no_long_mode		; if it's less, the cpu is too old for long mode

	; use extended info to test if long mode is available
	mov eax, 0x80000001		; argument for extended processor info
	cpuid					; returns various feature bits in ecx and edx
	test edx, 1 << 29		; test if the LM-bit is set in the D-register
	jz .no_long_mode		; if it's not set, there is no long mode
	ret
.no_long_mode:
	mov al, "2"
	jmp error

set_up_page_tables:
	; setup recursive mapping
	mov eax, p4_table
	or eax, 0b11 ; present + writable
	mov [p4_table + 511*8], eax

	; map first p4 entry to p3 table
	mov eax, p3_table
	or eax, 0b11			; present + writable
	mov [p4_table], eax

	; map first p3 entry to p2 table
	mov eax, p2_table
	or eax, 0b11			; present + writable
	mov [p3_table], eax

	; map each P2 entry to a huge 2MiB page
	mov ecx, 0				; counter var

.map_p2_table:
	; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
	mov eax, 0x200000		; 2MiB
	mul ecx					; start address of ecx-th page
	or eax, 0b10000011		; present + writable + huge
	mov [p2_table + ecx * 8], eax ; map exc-th entry

	inc ecx					; increase counter
	cmp ecx, 512			; if counter == 512, the whole P2 table is mapped
	jne .map_p2_table		; else map the next entry

	ret

enable_paging:
	; load P4 to cr3 register
	mov eax, p4_table
	mov cr3, eax

	; enable PAE-flag in cr4 (physical address extension)
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; set the long mode bit in the EFER MSR (model specific register)
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	; enable paging in the cr0 register
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret

; print 'ERR: X' where X is an error code
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb000a], al
	hlt

section .rodata
gdt64:
	dq 0										; zero entry
.code: equ $ - gdt64
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53)	; code segment
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

section .bss
align 4096
p4_table:
	resb 4096
p3_table:
	resb 4096
p2_table:
	resb 4096
stack_bottom:
	resb 4096 * 4
stack_top:
