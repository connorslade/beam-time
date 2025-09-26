pub mod hwid;
pub mod time;

pub macro include_asset($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

pub macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub macro key_events(
    $ctx:expr, { $($key:expr => $action:expr),* }
) {
    $($ctx.input.key_pressed($key).then(|| $action);)*
}

// why can't window just be normal...
pub fn enable_console() {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole};
        let _ = FreeConsole();
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
