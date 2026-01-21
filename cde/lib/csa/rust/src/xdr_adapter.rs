use crate::bindings::XDR;
use std::io::{self, Read, Write};
use std::os::raw::c_char;

pub struct XdrStream {
    xdr: *mut XDR,
}

impl XdrStream {
    pub unsafe fn new(xdr: *mut XDR) -> Self {
        Self { xdr }
    }
}

impl Write for XdrStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len() as u32; // Assuming u_int is u32 compat
        unsafe {
            let ops = (*self.xdr).x_ops;
            if let Some(putbytes) = (*ops).x_putbytes {
                // putbytes takes (xdrs, buf, len)
                let success = putbytes(self.xdr, buf.as_ptr() as *const c_char, len);
                if success != 0 {
                    return Ok(buf.len());
                }
            }
            Err(io::Error::new(io::ErrorKind::Other, "XDR write failed"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for XdrStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = buf.len() as u32;
        unsafe {
            let ops = (*self.xdr).x_ops;
            if let Some(getbytes) = (*ops).x_getbytes {
                let success = getbytes(self.xdr, buf.as_mut_ptr() as *mut c_char, len);
                if success != 0 {
                    return Ok(buf.len());
                }
            }
            Err(io::Error::new(io::ErrorKind::Other, "XDR read failed"))
        }
    }
}
