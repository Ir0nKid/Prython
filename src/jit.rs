extern crate libc;

use std::mem;
use std::ops::{Index, IndexMut};

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

const PAGE_SIZE: usize = 4096;

pub struct JitMemory {
    pub contents: *mut u8,
}

impl JitMemory {
    pub fn new(num_pages: usize) -> JitMemory {
        let contents: *mut u8;
        unsafe {
            let size = num_pages * PAGE_SIZE;
            let mut _contents: *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _contents, PAGE_SIZE, size);
            libc::mprotect(
                _contents,
                size,
                libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
            );

            memset(_contents, 0xc3, size); // for now, prepopulate with 'RET'

            contents = mem::transmute(_contents);
        }

        JitMemory { contents: contents }
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(_index as isize) }
    }
}
