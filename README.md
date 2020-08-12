# mistRust
Making a start at learning rust through implementing a stupid/simple shellcode dropper. Eventually I'll add some functionality like XOR or encryption and different methods or places to obfuscate/deobfuscate the shellcode. Ideally the final project will build both elf and PE executables.

Make sure toolchains installed (with rustup target add [chain])

 - stable-x86_64-pc-windows-gnu 
 - nightly

## Building Binary

For linux

`cargo build`

For Windows

`cargo build --target x86_64-pc-windows-gnu` 

## Reduce the size of the executable binary
To make smaller binaries you can `export RUSTFLAGS='-C prefer-dynamic'` before building. You can also use the linux native `strip droppin_a_load` to remove all symbols from object files etc.
