//! unmem crate contains some interesting functions. I tried to make them as safe as possible. However, remember that even not marked as unsafe they still can be dangerous.
//! All of these functions may cause UB!

#![no_std]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![feature(box_syntax)]
#![feature(decl_macro)]

extern crate alloc;
use alloc::boxed::Box;

/// Changes value of immutable variable.
/// 
/// Example:
/// ```no_run
/// let foo: u8 = 19;
/// change(&foo, 86); // foo = 86
/// ```
pub fn change<T>(src: &T, to: T) {
    unsafe {
        *((src as *const T) as usize as *mut T) = to;
    }
}

/// Gives you a mutable reference from immutable.
pub fn get_mut<T>(src: &T) -> &mut T {
    unsafe {
        &mut (*((src as *const T) as usize as *mut T))
    }
}

/// Analogue of memset.
pub fn set<T>(to: &mut T, val: u8, size: usize) -> &mut T {
    unsafe {
        core::ptr::write_bytes(to as *mut T, val, *&size);
        to
    }
}

/// Internal of transmute macro.
pub unsafe fn cast<FROM, TO>(src: FROM) -> TO {
    unsafe {
        core::ptr::read(&src as *const FROM as usize as *const TO)
    }
}

/// Takes variable of one type and returns it as another type. EXTREMELY UNSAFE. Undefined Behavior or panic guaranteed.
/// 
/// Example:
/// ```no_run
/// let foo: u8 = 48;
/// let bar: char = transmute!(u8 => char, foo); // '0'
/// ```
pub macro transmute($from:ty => $to:ty, $src:expr) {
    unsafe {
        cast::<$from, $to>($src)
    }
}

/// Writes value to address.
pub unsafe fn write<T>(to: usize, val: T) {
    unsafe {
        let mut ptr: usize = to;
        *(ptr as *mut T) = val;
    }
}

/// Returns value from address.
pub unsafe fn get<T>(from: usize) -> &'static T {
    unsafe {
        &*(from as *mut T)
    }
}

/// Drops given variable.
pub fn drop<T>(_src: T) {
    ()
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
pub unsafe fn orient<T>(src: &T, val: isize) -> &T {
    unsafe {
        &*((get_address(get_mut(src)) as isize + val) as usize as *mut T)
    }
}

/// Mean safe wrapper around raw pointers.
#[repr(transparent)]
#[derive(Clone)]
pub struct Ptr<T: ?Sized> {
    ptr: *mut T
}

impl<T> Ptr<T> {

    /// Not recommended, use Ptr::from_ref instead.
    #[inline]
    pub fn new(src: T) -> Self {
        Self::from_ref(&src)
    }

    /// Get value of T.
    pub fn get(&self) -> T {
        unsafe {
            core::ptr::read(self.as_ptr())
        }
    }

    /// Ptr<T> -> &T;
    #[inline]
    pub fn as_ref(&self) -> &T {
        unsafe {
            &*self.ptr
        }
    }

    /// Ptr<T> -> &mut T;
    #[inline]
    pub fn as_mut_ref(&self) -> &mut T {
        unsafe {
            &mut *self.ptr
        }
    }

    /// Replace inner's value to passed.
    pub fn set(&mut self, to: T) -> &mut Self {
        *self.as_mut_ref() = to;
        self
    }

    /// Ptr<T> -> Box<T>.
    #[inline]
    pub fn as_box(&self) -> Box<T> {
        box self.get()
    }

    /// Ptr<T> -> *const T;
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.as_ref() as *const T
    }

    /// Ptr<T> -> *mut T;
    #[inline]
    pub const fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Ptr<T> -> usize;
    #[inline]
    pub fn as_adr(&self) -> usize {
        unsafe {
            (*self).ptr as usize
        }
    }

    /// Casting usize as ptr may cause UB.
    #[inline]
    pub const unsafe fn from_adr(src: usize) -> Self {
        Self {
            ptr: src as *mut T
        }
    }

    /// Box<T> -> Ptr<T>.
    #[inline]
    pub fn from_box(src: Box<T>) -> Self {
        Self {
            ptr: Box::leak(src) as *mut T
        }
    }

    /// &T -> Ptr<T>.
    #[inline]
    pub fn from_ref(src: &T) -> Self {
        Self {
            ptr: transmute!(*const T => *mut T, src as *const T)
        }
    }

    /// &mut T -> Ptr<T>.
    #[inline]
    pub fn from_mut_ref(src: &mut T) -> Self {
        Self {
            ptr: src as *mut T
        }
    }

    /// Pointers are not guaranteed to be not null!
    #[inline]
    pub unsafe fn from_ptr(src: *const T) -> Self {
        Self {
            ptr: transmute!(*const T => *mut T, src)
        }
    }

    /// Pointers are not guaranteed to not be null!
    #[inline]
    pub const unsafe fn from_mut_ptr(src: *mut T) -> Self {
        Self {
            ptr: src
        }
    }

    /// May cause UB!
    pub unsafe fn orient(&self, shift: isize) -> Self {
        Self::from_mut_ptr(get_mut(orient(self.as_ref(), shift)) as *mut T)
    }

    /// Check if our ptr is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.as_adr() == 0
    }

    /// Returns size of 'T' from Ptr<T>.
    #[inline]
    pub fn size(&self) -> usize {
        core::mem::size_of::<T>()
    }

    /// NonNull<T> -> Ptr<T>.
    #[inline]
    pub const fn from_non_null(src: core::ptr::NonNull<T>) -> Self {
        Self {
            ptr: src.as_ptr()
        }
    }

    /// Ptr<T> -> NonNull<T>.
    #[inline]
    pub fn as_non_null(&self) -> core::ptr::NonNull<T> {
        unsafe {
            core::ptr::NonNull::new_unchecked(self.ptr)
        }
    }

    /// Example:
    /// ```no_run
    /// let foo: Ptr<u8> = Ptr::from_ref(&48);
    /// let bar: char = unsafe {foo.transmute::<char>()};
    /// ```
    pub unsafe fn transmute<TO>(&self) -> TO {
        transmute!(T => TO, self.get())
    }
}

impl<T> core::ops::Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> core::ops::DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_ref()
    }
}

impl<T> core::ops::Index<isize> for Ptr<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        unsafe {
            orient(&*self.ptr, index)
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", **self)
    }
}

#[cfg(target_pointer_width = "32")]
pub type fsize = f32;
#[cfg(target_pointer_width = "64")]
pub type fsize = f64;