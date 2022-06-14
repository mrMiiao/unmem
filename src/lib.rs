//! unmem crate contains some interesting functions. I tried to make them as safe as possible. However, remember that even not marked as unsafe they still can be dangerous.
//! All of these functions may cause UB!
//! Don't deploy this shit to production you madman!

#![no_std]
#![allow(unused_mut)]
#![allow(unused_unsafe)]

/// Changes value of immutable variable.
/// 
/// Example:
/// ```no_run
/// let foo: u8 = 19;
/// change(&foo, 86); // foo = 86
/// ```
pub fn change<T>(src: &T, to: T) {
    unsafe {
        let mut ptr: usize = (src as *const T) as usize;
        *(ptr as *mut T) = to;
    }
}

/// Gives you a mutable reference from immutable.
pub fn get_mut<T>(src: &T) -> &mut T {
    unsafe {
        let mut ptr: usize = (src as *const T) as usize;
        &mut (*(ptr as *mut T))
    }
}

/// Analogue of memset.
pub fn set<T>(to: &mut T, val: u8, size: usize) -> &mut T {
    unsafe {
        core::ptr::write_bytes(to as *mut T, val, *&size);
        to
    }
}

/// Takes variable of one type and returns it as another type. EXTREMELY UNSAFE. Undefined Behavior or panic guaranteed.
/// 
/// Example:
/// ```no_run
/// let foo: u8 = 48;
/// let bar: char = transmute::<u8, char>(foo); // '0'
/// ```
pub fn transmute<FROM, TO: Copy>(from: FROM) -> TO {
    unsafe {
        *(((&from as *const FROM) as usize) as *const TO)
    }
}

/// Writes value to address.
pub fn write<T>(to: usize, val: T) {
    unsafe {
        let mut ptr: usize = to;
        *(ptr as *mut T) = val;
    }
}

/// Returns value from address.
pub fn get<T: Copy>(from: usize) -> T {
    unsafe {
        *(from as *mut T)
    }
}

/// Frees memory.
pub fn free<T>(_src: T) {
    ()
}

/// Allocates memory.
pub fn alloc<T>(size: usize) -> &'static mut T {
    unsafe {
        let mut ptr: usize = 0;
        for i in 0..size {
            let byte: u8 = 6;
            if i == 0 {
                ptr = (&byte as *const u8) as usize;
            }
        }
        let mut n_ptr: usize = ptr as usize;
        &mut *(n_ptr as *mut T)
    }
}

/// Returns address from reference.
/// 
/// Example:
/// ```no_run
/// let foo: u8 = 16;
/// let bar: usize = get_address(&foo);
/// ```
pub fn get_address<T>(src: &T) -> usize {
    unsafe {
        src as *const T as usize
    }
}

/// Get value in memory related to src.
pub fn orient<T: Copy>(src: &T, val: isize) -> T {
    get((get_address(src) as isize + val) as usize)
}

/// Turns &T to [T; len] where len is constant.
/// 
/// Example:
/// ```no_run
/// let foo: [u8; 16] = ref_as_buf!(alloc::<u8>(16) => u8, 16);
/// ```
#[macro_export]
macro_rules! ref_as_buf {
    ($name:expr => $type:ty, $len:expr) => {
        unsafe {
            *(get_address($name) as *mut $type as *mut [$type; $len])
        }
    };
}
