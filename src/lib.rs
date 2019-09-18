use once_cell::sync::OnceCell;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

/// Equivalent of the `COZ_PROGRESS` and `COZ_PROGRESS_NAMED` macros
///
/// This can be executed as:
///
/// ```
/// coz::progress!();
/// ```
///
/// or ...
///
/// ```
/// coz::progress!("my unique name");
/// ```
#[macro_export]
macro_rules! progress {
    () => {{
        static COUNTER: $crate::Counter =
            $crate::Counter::progress(concat!(file!(), ":", line!(), "\0"));
        COUNTER.increment();
    }};
    ($name:expr) => {{
        static COUNTER: $crate::Counter = $crate::Counter::progress(concat!($name, "\0"));
        COUNTER.increment();
    }};
}

/// Equivalent of the `COZ_BEGIN` macro
///
/// This can be executed as:
///
/// ```
/// coz::begin!("foo");
/// ```
#[macro_export]
macro_rules! begin {
    ($name:expr) => {{
        static COUNTER: $crate::Counter = $crate::Counter::begin(concat!($name, "\0"));
        COUNTER.increment();
    }};
}

/// Equivalent of the `COZ_END` macro
///
/// This can be executed as:
///
/// ```
/// coz::end!("foo");
/// ```
#[macro_export]
macro_rules! end {
    ($name:expr) => {{
        static COUNTER: $crate::Counter = $crate::Counter::end(concat!($name, "\0"));
        COUNTER.increment();
    }};
}

/// Perform one-time initialization for `coz`.
///
/// This isn't necessary to call, but if you run into issues with segfaults
/// related to SIGPROF handlers this may help fix the issue since it installs a
/// bigger stack earlier on in the process.
pub fn init() {
    // As one-time program initialization, make sure that our sigaltstack is big
    // enough. By default coz uses SIGPROF on an alternate signal stack, but the
    // Rust standard library already sets up a SIGALTSTACK which is
    // unfortunately too small to run coz's handler. If our sigaltstack looks
    // too small let's allocate a bigger one and use it here.
    static SIGALTSTACK_DISABLE: OnceCell<()> = OnceCell::new();
    SIGALTSTACK_DISABLE.get_or_init(|| unsafe {
        let mut stack = mem::zeroed();
        libc::sigaltstack(ptr::null(), &mut stack);
        let size = 1 << 20; // 1mb
        if stack.ss_size >= size {
            return;
        }
        let ss_sp = libc::mmap(
            ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if ss_sp == libc::MAP_FAILED {
            panic!("failed to allocate alternative stack");
        }
        let new_stack = libc::stack_t {
            ss_sp,
            ss_flags: 0,
            ss_size: size,
        };
        libc::sigaltstack(&new_stack, ptr::null_mut());
    });
}

#[doc(hidden)]
pub struct Counter {
    slot: OnceCell<Option<&'static coz_counter_t>>,
    ty: libc::c_int,
    name: &'static str,
}

const COZ_COUNTER_TYPE_THROUGHPUT: libc::c_int = 1;
const COZ_COUNTER_TYPE_BEGIN: libc::c_int = 2;
const COZ_COUNTER_TYPE_END: libc::c_int = 3;

impl Counter {
    #[doc(hidden)]
    pub const fn progress(name: &'static str) -> Counter {
        Counter::new(COZ_COUNTER_TYPE_THROUGHPUT, name)
    }

    #[doc(hidden)]
    pub const fn begin(name: &'static str) -> Counter {
        Counter::new(COZ_COUNTER_TYPE_BEGIN, name)
    }

    #[doc(hidden)]
    pub const fn end(name: &'static str) -> Counter {
        Counter::new(COZ_COUNTER_TYPE_END, name)
    }

    const fn new(ty: libc::c_int, name: &'static str) -> Counter {
        Counter {
            slot: OnceCell::new(),
            ty,
            name,
        }
    }

    #[inline]
    pub fn increment(&self) {
        let counter = self.slot.get_or_init(|| self.create_counter());
        if let Some(counter) = counter {
            assert_eq!(
                mem::size_of_val(&counter.count),
                mem::size_of::<libc::size_t>()
            );
            counter.count.fetch_add(1, SeqCst);
        }
    }

    fn create_counter(&self) -> Option<&'static coz_counter_t> {
        let name = CStr::from_bytes_with_nul(self.name.as_bytes()).unwrap();
        let ptr = coz_get_counter(self.ty, name);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }
}

#[repr(C)]
#[doc(hidden)]
pub struct coz_counter_t {
    count: AtomicUsize,
    backoff: libc::size_t,
}

#[cfg(target_os = "linux")]
fn coz_get_counter(ty: libc::c_int, name: &CStr) -> *mut coz_counter_t {
    static PTR: AtomicUsize = AtomicUsize::new(1);
    let mut ptr = PTR.load(SeqCst);
    if ptr == 1 {
        let name = CStr::from_bytes_with_nul(b"_coz_get_counter\0").unwrap();
        ptr = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr() as *const _) as usize };
        PTR.store(ptr, SeqCst);
    }
    if ptr == 0 {
        return ptr::null_mut();
    }

    init(); // just in case we haven't already

    unsafe {
        mem::transmute::<
            usize,
            unsafe extern "C" fn(libc::c_int, *const libc::c_char) -> *mut coz_counter_t,
        >(ptr)(ty, name.as_ptr())
    }
}

#[cfg(not(target_os = "linux"))]
fn coz_get_counter(_ty: libc::c_int, _name: &CStr) -> *mut coz_counter_t {
    ptr::null_mut()
}
