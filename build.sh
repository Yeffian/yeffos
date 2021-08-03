cd target
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=x86_64-yeffos/debug/bootimage-yeffos.bin     