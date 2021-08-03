## YeffOS

A os. Thats about it.

# Running the OS
To run YeffOS, you need to use the [QEMU](https://www.qemu.org/) emulator. On a Manjaro/Arch-Based system (which is what im using atm), you can use the following command:
```
sudo pacman -S qemu
```

You also need `bootimage`, which you can install with the following commands:
```
rustup component add llvm-tools-preview
cargo bootimage
```

After that, give `build.sh` permissions and run it
```
chmod 755 ./build.sh
./build.sh
```