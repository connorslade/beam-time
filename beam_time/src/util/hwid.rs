#![allow(unused)]

use md5::Digest;

#[cfg(windows)]
pub fn get() -> u64 {
    use uuid::Uuid;
    use windows::Win32::{
        Foundation::HANDLE,
        Security::TOKEN_QUERY,
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    };
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let mut hash = md5::Context::new();

    // The key HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography holds a GUID
    // specific to the particular machine you are on
    let crypto = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"SOFTWARE\Microsoft\Cryptography")
        .unwrap();
    let guid = crypto.get_value::<String, _>("MachineGuid").unwrap();
    let guid = Uuid::parse_str(&guid).unwrap();
    hash.consume(guid);

    // A SID is an identifier for users / groups. Every account on a network is
    // issued a unique SID. By hashing this with the machine GUID, we should get
    // a new id for each user on each computer.
    unsafe {
        let mut handle = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle).unwrap();

        // see https://github.com/Whitecat18/Rust-for-Malware-Development/blob/bcf8bafbffb5f6edbfdbc6aa0e524f3d3a4e8ff4/Enumeration/get_token_info.rs#L22
        // let mut out = TOKEN_USER::default();
        // GetTokenInformation(
        //     tokenhandle,
        //     TokenUser,
        //     Some(&mut out as *mut _),
        //     tokeninformationlength,
        //     returnlength,
        // );
    }

    digest_as_u64(hash.compute())
}

// See http://0pointer.de/blog/projects/ids.html for info on linux hardware and
// software IDs. I am using both a hardware id and a user ID to each user on
// each machine should have different IDs.
#[cfg(target_os = "linux")]
pub fn get() -> u64 {
    use std::{ffi::c_long, fs};

    use uuid::Uuid;

    fn get_machine_id() -> Option<Uuid> {
        ["/var/lib/dbus/machine-id", "/etc/dbus/machine-id"]
            .iter()
            .filter_map(|path| {
                fs::read_to_string(path)
                    .map(|x| x.parse::<Uuid>().ok())
                    .ok()
            })
            .next()
            .flatten()
    }

    extern "C" {
        /// Returns the real user ID of the calling process.
        fn getuid() -> u32;
    }

    let mut hash = md5::Context::new();

    hash.consume(unsafe { getuid() }.to_be_bytes());
    if let Some(id) = get_machine_id() {
        hash.consume(id)
    };

    digest_as_u64(hash.compute())
}

fn digest_as_u64(digest: Digest) -> u64 {
    // Assuming the entropy is evenly distributed throughout the hash digest,
    // just picking the first 8 bits should be fiiine.
    u64::from_be_bytes([
        digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    ])
}
