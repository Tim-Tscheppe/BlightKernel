Writing a basic OS Kernel in Rust

Using tutorial here as a starting point:
https://os.phil-opp.com/

To build:
git clone https://github.com/Tim-Tscheppe/BlightKernel
cargo run

BlightKernel/target/x86_64-blight_kernel/debug/bootimage-blight_kernel.bin is where the boot image is located.
The comfig files are located at BlightKernel/x86_64-blight_kernel.json and 
You can copy it onto a USB drive or boot it on a VM