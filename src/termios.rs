use core::mem;
use unix::err::OsErr;
use unix::file::File;

#[derive(Clone, Copy)]
pub struct Termios(::libc::termios);

impl Termios {
    #[inline]
    pub fn get(f: &File) -> Result<Self, OsErr> { unsafe {
        let mut new = mem::uninitialized();
        match OsErr::from_sysret(syscall!(IOCTL, f.fd(), ::libc::TCGETS,
                                          &mut new as *mut _) as _) {
            Ok(_) => Ok(new),
            Err(e) => Err(e),
        }
    } }

    #[inline]
    pub fn set(self, f: &mut File, when: When) -> Result<(), OsErr> { unsafe {
        OsErr::from_sysret(syscall!(IOCTL, f.fd(), ::libc::TCSETS as usize + when as usize,
                                    &self as *const _) as _).map(|_| ())
    } }
}

#[derive(Debug, Clone, Copy)]
pub enum When {
    Now = 0,
    Drain = 1,
    Flush = 2,
}
