use std::{ffi::CString, thread, time::Duration};

fn main() {
    unsafe {
        let inst = nt::bindings::NT_GetDefaultInstance();

        let address = CString::new("/home/lvuser/networktables.json").unwrap();
        let local_path = CString::new("").unwrap();

        let address_ptr = address.as_ptr();
        let local_path_ptr = local_path.as_ptr();

        std::mem::forget(address);
        std::mem::forget(local_path);

        nt::bindings::NT_StartServer(inst, local_path_ptr, address_ptr, 1735, 5810);

        let mut amount_skipped: u8 = 0;

        while nt::bindings::NT_GetNetworkMode(inst)
            == nt::bindings::NT_NetworkMode_NT_NET_MODE_STARTING
        {
            thread::sleep(Duration::from_millis(10));

            amount_skipped += 1;

            if amount_skipped > 100 {
                panic!("Time out");
            }
        }
    }
}
