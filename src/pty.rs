use core::fmt::Write as FmtWrite;
use core::mem;
use io::Pos;
use null_terminated::Nul;
use unix::err::*;
use unix::file::*;

#[inline]
fn name(f: &File) -> Result<[u8; MAX_PATH_LENGTH], OsErr> {
    let mut path = [0u8; MAX_PATH_LENGTH];
    let _ = write!(Pos::from(&mut path[..]), "/dev/pts/{}", f.pts_n()?);
    Ok(path)
}

const MAX_PATH_LENGTH: usize = ((mem::size_of::<usize>() >> 1) + 2) * 5;

pub trait PtyExt {
    fn pts_n(&self) -> Result<usize, OsErr>;
    fn pts(&self) -> Result<File, OsErr>;
}

impl PtyExt for File {
    #[inline]
    fn pts_n(&self) -> Result<usize, OsErr> { unsafe {
        let mut n: ::libc::c_int = mem::uninitialized();
        esyscall_!(IOCTL, self.fd(), 0x80045430, &mut n as *mut _).map(|()| n as _)
    } }

    #[inline]
    fn pts(&self) -> Result<File, OsErr> {
        let n: ::libc::c_int = 0;
        unsafe { esyscall_!(IOCTL, self.fd(), 0x40045431, &n as *const _) }?;
        open_at(None, unsafe { Nul::new_unchecked(name(self)?.as_ptr()) }, OpenMode::RdWr | O_CLOEXEC, None)
    }
}
