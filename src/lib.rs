#![no_std]

extern crate io;
extern crate libc;
extern crate null_terminated;
#[macro_use]
extern crate syscall;
#[macro_use]
extern crate unix;

use unix::err::OsErr;

pub mod pty;
pub mod termios;

pub trait TtyExt {
    fn get_tty_size(&self) -> Result<(ushort, ushort), OsErr>;
    fn set_tty_size(&mut self, rows: ushort, cols: ushort) -> Result<(), OsErr>;
}

impl TtyExt for ::unix::file::File {
    #[inline]
    fn get_tty_size(&self) -> Result<(ushort, ushort), OsErr> { unsafe {
        let mut wsz: ::libc::winsize = ::core::mem::uninitialized();
        esyscall_!(IOCTL, self.fd(), ::libc::TIOCGWINSZ, &mut wsz as *mut _)
            .map(|()| (wsz.ws_col, wsz.ws_row))
    } }

    #[inline]
    fn set_tty_size(&mut self, cols: ushort, rows: ushort) -> Result<(), OsErr> { unsafe {
        let mut wsz: ::libc::winsize = ::core::mem::uninitialized();
        wsz.ws_row = rows;
        wsz.ws_col = cols;
        esyscall_!(IOCTL, self.fd(), ::libc::TIOCSWINSZ, &wsz as *const _)
    } }
}

use ::libc::c_ushort as ushort;
