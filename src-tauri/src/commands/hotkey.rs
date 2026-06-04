use tauri::Emitter;
use std::sync::atomic::{AtomicBool, Ordering};

static FN_MONITOR_ACTIVE: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub async fn start_fn_key_monitor(app: tauri::AppHandle) -> Result<(), String> {
    if FN_MONITOR_ACTIVE.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    std::thread::spawn(move || {
        unsafe {
            extern "C" {
                fn CGEventTapCreate(
                    tap: u32,
                    place: u32,
                    options: u32,
                    events_of_interest: u64,
                    callback: extern "C" fn(
                        proxy: *mut std::ffi::c_void,
                        event_type: u32,
                        event: *mut std::ffi::c_void,
                        user_info: *mut std::ffi::c_void,
                    ) -> *mut std::ffi::c_void,
                    user_info: *mut std::ffi::c_void,
                ) -> *mut std::ffi::c_void;

                fn CFMachPortCreateRunLoopSource(
                    allocator: *const std::ffi::c_void,
                    port: *mut std::ffi::c_void,
                    order: i64,
                ) -> *mut std::ffi::c_void;

                fn CFRunLoopAddSource(
                    rl: *mut std::ffi::c_void,
                    source: *mut std::ffi::c_void,
                    mode: *const std::ffi::c_void,
                );

                fn CFRunLoopGetCurrent() -> *mut std::ffi::c_void;
                fn CFRunLoopRun();
                fn CGEventGetFlags(event: *mut std::ffi::c_void) -> u64;

                static kCFRunLoopDefaultMode: *const std::ffi::c_void;
            }

            static mut APP_HANDLE: Option<tauri::AppHandle> = None;
            static mut FN_WAS_DOWN: bool = false;
            APP_HANDLE = Some(app);

            extern "C" fn event_callback(
                _proxy: *mut std::ffi::c_void,
                _event_type: u32,
                event: *mut std::ffi::c_void,
                _user_info: *mut std::ffi::c_void,
            ) -> *mut std::ffi::c_void {
                unsafe {
                    let flags = CGEventGetFlags(event);
                    let fn_down = (flags & 0x00800000) != 0;

                    if fn_down && !FN_WAS_DOWN {
                        FN_WAS_DOWN = true;
                        if let Some(ref app) = APP_HANDLE {
                            let _ = app.emit("fn-key-down", ());
                        }
                    } else if !fn_down && FN_WAS_DOWN {
                        FN_WAS_DOWN = false;
                        if let Some(ref app) = APP_HANDLE {
                            let _ = app.emit("fn-key-up", ());
                        }
                    }
                }
                event
            }

            let tap = CGEventTapCreate(
                0,       // kCGHIDEventTap
                0,       // kCGHeadInsertEventTap
                1,       // kCGEventTapOptionListenOnly
                1 << 12, // flagsChanged events
                event_callback,
                std::ptr::null_mut(),
            );

            if tap.is_null() {
                log::error!("Failed to create event tap — Accessibility permission required");
                FN_MONITOR_ACTIVE.store(false, Ordering::SeqCst);
                return;
            }

            let source = CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0);
            let run_loop = CFRunLoopGetCurrent();
            CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);

            log::info!("Fn key monitor started");
            CFRunLoopRun();
        }
    });

    Ok(())
}
