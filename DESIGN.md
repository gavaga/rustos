# Rust Learning OS Design

## Module Structure

* _kernel_ -- The main kernel module representing everything that runs in kernel mode
	* _src_ -- The source tree for the kernel module
		* _arch_ -- All architecture-dependent kernel code, including boot setup code
			* _x86_64_ -- Arch-dependent stuff for x86_64
		* _mem_	-- all memory management related stuff
		* _dev_ -- Generic device interface
			* _tty_	-- Modules related to tty
				* _vga_ -- Modules related to
* _boot_ -- (asm) code related to booting the operating system and general setup
