use crate::format::{DBREC, OR_DBREC};
use crate::raima::{d_close, d_open, d_recfrst, d_recread, d_setpages, S_DBOPEN, S_OKAY};
use libc::{c_char, c_int, c_void};
use std::ffi::CString;

pub struct DtSearchParser {
    connected: bool,
}

impl DtSearchParser {
    pub fn new() -> Self {
        DtSearchParser { connected: false }
    }

    pub fn open(&mut self, path: &str) -> Result<(), String> {
        unsafe {
            let c_path = CString::new(path).map_err(|_| "Invalid path")?;
            let c_mode = CString::new("r").unwrap(); // Read only

            // Set reasonable defaults
            d_setpages(64, 4);

            let status = d_open(c_path.as_ptr(), c_mode.as_ptr());
            if status != S_OKAY {
                return Err(format!("Failed to open database: {}", status));
            }
            self.connected = true;
            Ok(())
        }
    }

    pub fn read_dbrec(&self) -> Result<DBREC, String> {
        if !self.connected {
            return Err("Database not connected".to_string());
        }

        unsafe {
            // Locate the first DBREC
            let status = d_recfrst(OR_DBREC, 0); // 0 = LOCK? FFI signature check
            if status != S_OKAY {
                return Err(format!("Failed to locate DBREC: {}", status));
            }

            // Create uninitialized DBREC
            let mut dbrec: DBREC = std::mem::zeroed();

            // Read it
            let status = d_recread(&mut dbrec as *mut _ as *mut c_void, 0);
            if status != S_OKAY {
                return Err(format!("Failed to read DBREC: {}", status));
            }

            // Swap endianness
            dbrec.swab_to_host();

            Ok(dbrec)
        }
    }

    pub fn read_objrec(&self, lock: bool) -> Result<crate::format::or_objrec, String> {
        use crate::format::{or_objrec, OR_OBJREC};
        use libc::c_void;

        unsafe {
            let mut obj: or_objrec = std::mem::zeroed();
            let status = d_recread(&mut obj as *mut _ as *mut c_void, if lock { 1 } else { 0 });
            if status != S_OKAY {
                return Err(format!("Failed to read OBJREC: {}", status));
            }
            obj.swab_to_host();
            Ok(obj)
        }
    }

    pub fn find_term(&self, term: &str) -> Option<crate::format::or_hwordrec> {
        use crate::format::{or_hwordrec, OR_HWORDKEY};
        use crate::raima::{d_keyfind, S_OKAY};
        use libc::c_void;
        use std::ffi::CString;

        let c_term = CString::new(term).ok()?;
        unsafe {
            if d_keyfind(OR_HWORDKEY as i32, c_term.as_ptr(), 0) == S_OKAY {
                let mut word: or_hwordrec = std::mem::zeroed();
                if d_recread(&mut word as *mut _ as *mut c_void, 0) == S_OKAY {
                    word.swab_to_host();
                    return Some(word);
                }
            }
        }
        None
    }

    pub fn read_occurrences(&self, word: &crate::format::or_hwordrec) -> Vec<i32> {
        use crate::format::usrblk;
        use libc::{c_long, c_void, fread, fseek, SEEK_SET};

        let mut results = Vec::new();
        unsafe {
            let dblk = usrblk.dblk;
            if dblk.is_null() {
                return results;
            }
            let fptr = (*dblk).iifile;
            if fptr.is_null() {
                return results;
            }

            if fseek(fptr, word.or_hwoffset as c_long, SEEK_SET) != 0 {
                return results;
            }

            let count = word.or_hwaddrs as usize;
            if count == 0 {
                return results;
            }

            let mut buf = vec![0i32; count];
            let read_count = fread(buf.as_mut_ptr() as *mut c_void, 4, count, fptr);

            for i in 0..read_count {
                let raw = i32::from_be(buf[i]);
                let recno = (raw >> 8) & 0x00_ff_ff_ff;
                results.push(recno);
            }
        }
        results
    }
}

impl Drop for DtSearchParser {
    fn drop(&mut self) {
        if self.connected {
            unsafe {
                d_close();
            }
        }
    }
}
