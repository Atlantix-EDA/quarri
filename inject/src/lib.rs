//! LD_PRELOAD library to inject a Qt stylesheet into Intel Quartus on Linux.
//!
//! Quartus's argument parser intercepts `-stylesheet` before Qt can process it.
//! This library hooks `QApplication::exec()` and calls `setStyleSheet()` directly
//! on the live QApplication instance, right before the event loop starts.
//!
//! Set `QUARTUS_QSS=/path/to/darkstyle.qss` before launching Quartus.

use std::ffi::CStr;
use std::fs;
use std::sync::Once;

static INIT: Once = Once::new();

// Qt 6.5 QByteArrayView: size FIRST, data SECOND
#[repr(C)]
struct QByteArrayView {
    size: i64,
    data: *const u8,
}

// Qt 6.5 QString: QArrayDataPointer<char16_t> = 3 pointers (24 bytes)
#[repr(C)]
struct QString {
    _d: [usize; 3],
}

type FnFromUtf8 = unsafe extern "C" fn(QByteArrayView) -> QString;
type FnSetStyleSheet = unsafe extern "C" fn(*mut std::ffi::c_void, *const QString);
type FnExec = unsafe extern "C" fn() -> i32;

unsafe fn dlsym_raw(name: &CStr) -> *mut std::ffi::c_void {
    unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) }
}

unsafe fn dlsym_next(name: &CStr) -> *mut std::ffi::c_void {
    unsafe { libc::dlsym(libc::RTLD_NEXT, name.as_ptr()) }
}

fn apply_stylesheet() {
    let qss_path = match std::env::var("QUARTUS_QSS") {
        Ok(p) if !p.is_empty() => p,
        _ => return,
    };

    let content = match fs::read(&qss_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[qss_inject] can't read {qss_path}: {e}");
            return;
        }
    };

    unsafe {
        let from_utf8: Option<FnFromUtf8> = std::mem::transmute(dlsym_raw(
            c"_ZN7QString8fromUtf8E14QByteArrayView",
        ));
        let set_stylesheet: Option<FnSetStyleSheet> = std::mem::transmute(dlsym_raw(
            c"_ZN12QApplication13setStyleSheetERK7QString",
        ));
        let self_ptr = dlsym_raw(c"_ZN16QCoreApplication4selfE") as *const *mut std::ffi::c_void;

        match (from_utf8, set_stylesheet, self_ptr.as_ref()) {
            (Some(from_utf8), Some(set_stylesheet), Some(&app)) if !app.is_null() => {
                let len = content.len() as i64;
                let bav = QByteArrayView {
                    size: len,
                    data: content.as_ptr(),
                };
                let qs = from_utf8(bav);
                set_stylesheet(app, &qs);
                // Leak content so the stylesheet data stays valid
                std::mem::forget(content);
                eprintln!("[qss_inject] applied stylesheet: {qss_path} ({len} bytes)");
            }
            _ => {
                eprintln!("[qss_inject] failed to resolve Qt symbols");
            }
        }
    }
}

/// Hook for QApplication::exec() — called to start the Qt event loop.
/// At this point QApplication is fully constructed, so we can call setStyleSheet.
#[unsafe(no_mangle)]
pub extern "C" fn _ZN12QApplication4execEv() -> i32 {
    INIT.call_once(apply_stylesheet);

    unsafe {
        let real_exec: Option<FnExec> =
            std::mem::transmute(dlsym_next(c"_ZN12QApplication4execEv"));
        match real_exec {
            Some(f) => f(),
            None => {
                eprintln!("[qss_inject] FATAL: can't find real QApplication::exec()");
                1
            }
        }
    }
}
