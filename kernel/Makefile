os_name := rustos
arch ?= x86_64
vendor_os := unknown-none
build_dir := build
kernel := $(build_dir)/kernel-$(arch).bin
iso := $(build_dir)/$(os_name)-$(arch).iso
target := $(arch)-$(vendor_os)
rust_target := $(arch)-$(os_name)
target_folder := target/$(arch)-$(os_name)
libos := $(target_folder)/debug/libos.a

linker_script := src/arch/$(arch)/boot/linker.ld
grub_cfg := src/arch/$(arch)/boot/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/boot/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/boot/%.asm, \
	$(build_dir)/arch/$(arch)/boot/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel debug

all: $(kernel)

clean:
	@rm -r $(build_dir)

run: $(iso)
	@qemu-system-$(arch) -cdrom $(iso) -s

debug: $(iso)
	@qemu-system-$(arch) -cdrom $(iso) -s -S

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(build_dir)/isofiles/boot/grub
	@cp $(kernel) $(build_dir)/isofiles/boot/kernel.bin
	@cp $(grub_cfg) $(build_dir)/isofiles/boot/grub
	@grub-mkrescue -o $(iso) $(build_dir)/isofiles 2> /dev/null
	@rm -r $(build_dir)/isofiles

$(kernel): kernel $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) 			\
		$(assembly_object_files) $(libos)

kernel: $(target_folder)
	@cd $(target_folder) && xargo build --target $(rust_target) --manifest-path ../../Cargo.toml

$(build_dir)/arch/$(arch)/boot/%.o: src/arch/$(arch)/boot/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
