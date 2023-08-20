Writing a basic OS Kernel in Rus

Using tutorial here as a starting point:
https://os.phil-opp.com/

To build:
git clone https://github.com/Tim-Tscheppe/BlightKernel
cargo build

BlightKernel/target/x86_64-blight_kernel/debug/bootimage-blight_kernel.bin is where the boot image is located.
You can copy it onto a USB drive or boot it on a VM