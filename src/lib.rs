#![no_std]

extern crate io;
extern crate libc;
extern crate null_terminated;
#[macro_use]
extern crate syscall;
#[macro_use]
extern crate unix;

use core::mem;
use unix::{err::OsErr, file::File};

pub mod pty;

mod private { pub trait Sealed {} }

pub trait TtyExt: private::Sealed {
    fn get_tty_size(&self) -> Result<(ushort, ushort), OsErr>;
    fn set_tty_size(&mut self, rows: ushort, cols: ushort) -> Result<(), OsErr>;
    fn get_termios(&self) -> Result<Termios, OsErr>;
    fn set_termios(&mut self, termios: Termios, when: termios::When) -> Result<(), OsErr>;
}

impl private::Sealed for File {}

impl TtyExt for File {
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

    #[inline]
    fn get_termios(&self) -> Result<Termios, OsErr> { unsafe {
        let mut termios = mem::uninitialized();
        esyscall_!(IOCTL, self.fd(), ::libc::TCGETS, &mut termios as *mut _).map(|()| termios)
    } }

    #[inline]
    fn set_termios(&mut self, termios: Termios, when: termios::When) -> Result<(), OsErr> { unsafe {
        esyscall_!(IOCTL, self.fd(), ::libc::TCSETS as usize + when as usize, &termios as *const _)
    } }
}

pub use libc::termios as Termios;
use libc::c_ushort as ushort;

pub mod termios {
    #[derive(Debug, Clone, Copy)]
    pub enum When {
        Now = 0,
        Drain = 1,
        Flush = 2,
    }
}
