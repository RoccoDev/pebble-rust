/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

mod internal;

pub mod app;
pub mod window;
pub mod types;
pub mod layer;
pub mod std;
pub mod system;
pub mod app_message;
pub mod event;
pub mod clock;

pub use internal::alloc;

pub use internal::types::Window as RawWindow;
pub type WindowPtr = *mut RawWindow;

pub type Result<T> = core::result::Result<T, &'static str>;

pub use internal::functions::declarations::app_log as println;
pub use internal::functions::declarations::snprintf;

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! pbl_print {
    ($lvl: expr, $name: expr, $fmt: expr $(, $arg:expr)*) => {
        unsafe {
            pebble::println($lvl, $name.as_ptr(), 0, nt!($fmt).as_ptr() $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! pbl_log {
    ($fmt: expr $(, $arg: expr)*) => {
        pbl_print!(100, "pebble-rust (Info)\0", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! pbl_warn {
    ($fmt: expr $(, $arg: expr)*) => {
        pbl_print!(50, "pebble-rust (Warning)\0", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! pbl_err {
    ($fmt: expr $(, $arg: expr)*) => {
        pbl_print!(1, "pebble-rust (Error)\0", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! null_term {
    ($content: tt) => {
        concat!($content, "\0");
    };
}

#[macro_export]
macro_rules! nt {
    ($content: tt) => {
        null_term!($content);
    };
}