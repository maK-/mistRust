use mmap::*;
use std::{ptr, mem};

//Run shellcode in linux
pub fn run(shellcode: Vec<u8>){

    let options = [ MapOption::MapReadable,
                    MapOption::MapWritable,
                    MapOption::MapExecutable
    ];
    
    let mappings = MemoryMap::new(shellcode.len(), &options).unwrap();
    
    unsafe{
        ptr::copy(shellcode.as_ptr(), mappings.data(), shellcode.len());
        mem::transmute::<_, fn()>(mappings.data())();
    }
}
