//! Sends email through Outlook via COM automation — the Rust equivalent of
//! the Python `win32com.client.Dispatch("Outlook.Application")`.

use std::{mem, ptr};

use windows::core::{GUID, IUnknown, Interface, PCWSTR, VARIANT};
use windows::Win32::System::Com::{
    CLSIDFromProgID, CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_LOCAL_SERVER,
    COINIT_APARTMENTTHREADED, DISPATCH_METHOD, DISPATCH_PROPERTYPUT, DISPPARAMS, IDispatch,
};
use windows::Win32::System::Ole::DISPID_PROPERTYPUT;

const LOCALE_USER_DEFAULT: u32 = 0x0400;
const VT_DISPATCH: u16 = 9;

/// Raw view of the platform VARIANT ABI, used to extract the IDispatch
/// pointer returned by IDispatch::Invoke. Layout matches tagVARIANT:
/// vt + 3 reserved words, then the 8-byte value, padded to 24 bytes total.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct RawVariant {
    vt: u16,
    _reserved: [u16; 3],
    payload: [u64; 2],
}

/// Mirrors the Python `email_func()` — sends `body` to `to_email` with
/// `subject` as the subject line, via Outlook.
pub fn send_email(to: &str, subject: &str, body: &str) -> Result<(), String> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(format!("Could not initialize COM: {}", hr.message()));
        }
        let result = send_via_outlook(to, subject, body);
        CoUninitialize();
        result
    }
}

unsafe fn send_via_outlook(to: &str, subject: &str, body: &str) -> Result<(), String> {
    // Locate the installed Outlook.Application class.
    let prog_id: Vec<u16> = "Outlook.Application"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // Step 1: resolve the ProgID.
    let clsid = CLSIDFromProgID(PCWSTR(prog_id.as_ptr()))
        .map_err(|e| format!("Step 1 (ProgID lookup) failed: {}", e))?;

    // Step 2: create the object as IUnknown, the way PowerShell does.
    let unknown: IUnknown = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
        .map_err(|e| format!("Step 2 (create object as IUnknown) failed: {}", e))?;

    // Step 3: ask the object for its IDispatch interface.
    let outlook: IDispatch = unknown
        .cast::<IDispatch>()
        .map_err(|e| format!("Step 3 (QueryInterface for IDispatch) failed: {}", e))?;

    // Step 4+: exercise the automation, exactly as the Python version did.
    let mail = invoke_get_object(&outlook, "CreateItem", &[VARIANT::from(0_i32)])
        .map_err(|e| format!("Step 4 (CreateItem) failed: {}", e))?;

    put_string(&mail, "To", to).map_err(|e| format!("Step 5 (set To) failed: {}", e))?;
    put_string(&mail, "Subject", subject)
        .map_err(|e| format!("Step 6 (set Subject) failed: {}", e))?;
    put_string(&mail, "Body", body).map_err(|e| format!("Step 7 (set Body) failed: {}", e))?;

    invoke_simple(&mail, "Send").map_err(|e| format!("Step 8 (Send) failed: {}", e))?;

    Ok(())
}

/// Look up the dispatch ID for a named method/property (late binding).
unsafe fn get_dispid(target: &IDispatch, name: &str) -> Result<i32, String> {
    let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let names = [PCWSTR(wide.as_ptr())];
    let mut dispid: [i32; 1] = [0];
    let null_guid = GUID::zeroed();
    target
        .GetIDsOfNames(
            &null_guid,
            names.as_ptr(),
            1,
            LOCALE_USER_DEFAULT,
            dispid.as_mut_ptr(),
        )
        .map_err(|_| format!("Outlook does not support '{}'", name))?;
    Ok(dispid[0])
}

/// Set a string property on an object, e.g. mailitem.To = "..."
unsafe fn put_string(target: &IDispatch, name: &str, value: &str) -> Result<(), String> {
    let dispid = get_dispid(target, name)?;
    let mut variant = VARIANT::from(value);
    let mut named: [i32; 1] = [DISPID_PROPERTYPUT];
    let params = DISPPARAMS {
        rgvarg: &mut variant,
        rgdispidNamedArgs: named.as_mut_ptr(),
        cArgs: 1,
        cNamedArgs: 1,
    };
    let null_guid = GUID::zeroed();
    target
        .Invoke(
            dispid,
            &null_guid,
            LOCALE_USER_DEFAULT,
            DISPATCH_PROPERTYPUT,
            &params,
            None,
            None,
            None,
        )
        .map_err(|e| format!("Setting '{}' failed: {}", name, e))
}

/// Call a method that takes no arguments and returns nothing, e.g. Send().
unsafe fn invoke_simple(target: &IDispatch, name: &str) -> Result<(), String> {
    let dispid = get_dispid(target, name)?;
    let params = DISPPARAMS {
        rgvarg: ptr::null_mut(),
        rgdispidNamedArgs: ptr::null_mut(),
        cArgs: 0,
        cNamedArgs: 0,
    };
    let null_guid = GUID::zeroed();
    target
        .Invoke(
            dispid,
            &null_guid,
            LOCALE_USER_DEFAULT,
            DISPATCH_METHOD,
            &params,
            None,
            None,
            None,
        )
        .map_err(|e| format!("'{}' failed: {}", name, e))
}

/// Call a method that returns an object (an IDispatch interface).
unsafe fn invoke_get_object(
    target: &IDispatch,
    name: &str,
    args: &[VARIANT],
) -> Result<IDispatch, String> {
    let dispid = get_dispid(target, name)?;

    // Invoke expects arguments in reverse order.
    let mut rev: Vec<VARIANT> = args.to_vec();
    rev.reverse();
    let params = DISPPARAMS {
        rgvarg: if rev.is_empty() {
            ptr::null_mut()
        } else {
            rev.as_mut_ptr()
        },
        rgdispidNamedArgs: ptr::null_mut(),
        cArgs: rev.len() as u32,
        cNamedArgs: 0,
    };

    let mut result = VARIANT::default();
    let null_guid = GUID::zeroed();
    target
        .Invoke(
            dispid,
            &null_guid,
            LOCALE_USER_DEFAULT,
            DISPATCH_METHOD,
            &params,
            Some(&mut result),
            None,
            None,
        )
        .map_err(|e| format!("'{}' failed: {}", name, e))?;

    // Pull the returned IDispatch out of the result VARIANT.
    let raw = ptr::read(&result as *const VARIANT as *const RawVariant);
    // Neutralize the original so its Drop does not release the interface.
    ptr::write(
        &mut result as *mut VARIANT as *mut RawVariant,
        RawVariant { vt: 0, _reserved: [0; 3], payload: [0, 2] },
    );
    mem::forget(result);

    if raw.vt != VT_DISPATCH {
        return Err(format!("Unexpected value type returned from Outlook (vt={})", raw.vt));
    }
    let raw_ptr = raw.payload[0] as *mut core::ffi::c_void;
    if raw_ptr.is_null() {
        return Err("Outlook returned no object".to_string());
    }
    Ok(mem::transmute(raw_ptr))
}