#![allow(unused)]

use md5::Digest;

#[cfg(windows)]
pub fn get() -> u64 {
    use std::{
        ffi::{CString, c_void},
        mem,
        ptr::null_mut,
        time::Instant,
    };

    use uuid::Uuid;
    use windows::Win32::{
        Foundation::{HANDLE, LocalFree},
        Security::{
            Authorization::ConvertSidToStringSidA, GetLengthSid, GetSidSubAuthority,
            GetSidSubAuthorityCount, GetTokenInformation, SID, TOKEN_QUERY, TOKEN_USER, TokenUser,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    };
    use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

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

        let mut token_size = 0;
        GetTokenInformation(handle, TokenUser, None, 0, &mut token_size);

        let mut token_user = vec![0u8; token_size as usize];
        let mut size = 0;
        GetTokenInformation(
            handle,
            TokenUser,
            Some(token_user.as_mut_ptr() as *mut _),
            token_size,
            &mut size,
        )
        .unwrap();

        let user = token_user.as_ptr() as *const TOKEN_USER;
        let psid = (*user).User.Sid;
        let sid = *(psid.0 as *const SID);

        hash.consume(sid.IdentifierAuthority.Value);
        for i in 0..sid.SubAuthorityCount as u32 {
            let sub_auth = *GetSidSubAuthority(psid, i);
            hash.consume(sub_auth.to_be_bytes());
        }
    }

    digest_as_u64(hash.compute())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
unsafe extern "C" {
    /// Returns the real user ID of the calling process.
    fn getuid() -> u32;
}

// See http://0pointer.de/blog/projects/ids.html for info on linux hardware and
// software IDs. I am using both a hardware id and a user ID, so each user on
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

    let mut hash = md5::Context::new();

    hash.consume(unsafe { getuid() }.to_be_bytes());
    if let Some(id) = get_machine_id() {
        hash.consume(id)
    };

    digest_as_u64(hash.compute())
}

#[cfg(target_os = "macos")]
pub fn get() -> u64 {
    use std::{ffi::CString, hash::Hash};

    use objc2_core_foundation::{CFString, kCFAllocatorDefault};
    use objc2_io_kit::{
        IORegistryEntryCreateCFProperty, IOServiceGetMatchingService, IOServiceMatching,
        kIOMasterPortDefault,
    };

    let serial_number = unsafe {
        let service = IOServiceMatching(c"IOPlatformExpertDevice".as_ptr()).unwrap();
        let result =
            IOServiceGetMatchingService(kIOMasterPortDefault, Some(service.as_opaque().into()));

        let property = IORegistryEntryCreateCFProperty(
            result,
            Some(&CFString::from_str("IOPlatformSerialNumber")),
            kCFAllocatorDefault,
            0,
        )
        .unwrap();

        property.downcast::<CFString>().unwrap().to_string()
    };

    let mut hash = md5::Context::new();
    hash.consume(unsafe { getuid() }.to_be_bytes());
    hash.consume(serial_number);

    digest_as_u64(hash.compute())
}

fn digest_as_u64(digest: Digest) -> u64 {
    // Assuming the entropy is evenly distributed throughout the hash digest,
    // just picking the first 8 bits should be fiiine.
    u64::from_be_bytes([
        digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    ])
}
