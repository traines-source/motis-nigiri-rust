/* automatically generated by rust-bindgen 0.69.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 35;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    const UNINIT: ::std::mem::MaybeUninit<__fsid_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_timetable {
    _unused: [u8; 0],
}
pub type nigiri_timetable_t = nigiri_timetable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_transport {
    pub route_idx: u32,
    pub n_event_mams: u16,
    pub event_mams: *mut i16,
    pub name: *const ::std::os::raw::c_char,
    pub name_len: u32,
}
#[test]
fn bindgen_test_layout_nigiri_transport() {
    const UNINIT: ::std::mem::MaybeUninit<nigiri_transport> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<nigiri_transport>(),
        32usize,
        concat!("Size of: ", stringify!(nigiri_transport))
    );
    assert_eq!(
        ::std::mem::align_of::<nigiri_transport>(),
        8usize,
        concat!("Alignment of ", stringify!(nigiri_transport))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).route_idx) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_transport),
            "::",
            stringify!(route_idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_event_mams) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_transport),
            "::",
            stringify!(n_event_mams)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mams) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_transport),
            "::",
            stringify!(event_mams)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_transport),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_transport),
            "::",
            stringify!(name_len)
        )
    );
}
pub type nigiri_transport_t = nigiri_transport;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_location {
    pub id: *const ::std::os::raw::c_char,
    pub id_len: u32,
    pub name: *const ::std::os::raw::c_char,
    pub name_len: u32,
    pub lon: f64,
    pub lat: f64,
    pub transfer_time: u16,
    pub parent: u32,
}
#[test]
fn bindgen_test_layout_nigiri_location() {
    const UNINIT: ::std::mem::MaybeUninit<nigiri_location> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<nigiri_location>(),
        56usize,
        concat!("Size of: ", stringify!(nigiri_location))
    );
    assert_eq!(
        ::std::mem::align_of::<nigiri_location>(),
        8usize,
        concat!("Alignment of ", stringify!(nigiri_location))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(id_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lon) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(lon)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lat) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(lat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).transfer_time) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(transfer_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_location),
            "::",
            stringify!(parent)
        )
    );
}
pub type nigiri_location_t = nigiri_location;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_route_stop {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_nigiri_route_stop() {
    assert_eq!(
        ::std::mem::size_of::<nigiri_route_stop>(),
        4usize,
        concat!("Size of: ", stringify!(nigiri_route_stop))
    );
    assert_eq!(
        ::std::mem::align_of::<nigiri_route_stop>(),
        4usize,
        concat!("Alignment of ", stringify!(nigiri_route_stop))
    );
}
impl nigiri_route_stop {
    #[inline]
    pub fn location_idx(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 30u8) as u32) }
    }
    #[inline]
    pub fn set_location_idx(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub fn in_allowed(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_in_allowed(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn out_allowed(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_out_allowed(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        location_idx: ::std::os::raw::c_uint,
        in_allowed: ::std::os::raw::c_uint,
        out_allowed: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 30u8, {
            let location_idx: u32 = unsafe { ::std::mem::transmute(location_idx) };
            location_idx as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let in_allowed: u32 = unsafe { ::std::mem::transmute(in_allowed) };
            in_allowed as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let out_allowed: u32 = unsafe { ::std::mem::transmute(out_allowed) };
            out_allowed as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type nigiri_route_stop_t = nigiri_route_stop;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_route {
    pub n_stops: u16,
    pub stops: *mut nigiri_route_stop_t,
    pub clasz: u16,
}
#[test]
fn bindgen_test_layout_nigiri_route() {
    const UNINIT: ::std::mem::MaybeUninit<nigiri_route> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<nigiri_route>(),
        24usize,
        concat!("Size of: ", stringify!(nigiri_route))
    );
    assert_eq!(
        ::std::mem::align_of::<nigiri_route>(),
        8usize,
        concat!("Alignment of ", stringify!(nigiri_route))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_stops) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_route),
            "::",
            stringify!(n_stops)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stops) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_route),
            "::",
            stringify!(stops)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clasz) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_route),
            "::",
            stringify!(clasz)
        )
    );
}
pub type nigiri_route_t = nigiri_route;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nigiri_event_change {
    pub transport_idx: u32,
    pub day_idx: u16,
    pub stop_idx: u32,
    pub is_departure: bool,
    pub delay: i16,
    pub cancelled: bool,
}
#[test]
fn bindgen_test_layout_nigiri_event_change() {
    const UNINIT: ::std::mem::MaybeUninit<nigiri_event_change> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<nigiri_event_change>(),
        20usize,
        concat!("Size of: ", stringify!(nigiri_event_change))
    );
    assert_eq!(
        ::std::mem::align_of::<nigiri_event_change>(),
        4usize,
        concat!("Alignment of ", stringify!(nigiri_event_change))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).transport_idx) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(transport_idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).day_idx) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(day_idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stop_idx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(stop_idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_departure) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(is_departure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).delay) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(delay)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cancelled) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nigiri_event_change),
            "::",
            stringify!(cancelled)
        )
    );
}
pub type nigiri_event_change_t = nigiri_event_change;
extern "C" {
    pub fn nigiri_load(
        path: *const ::std::os::raw::c_char,
        from_ts: i64,
        to_ts: i64,
    ) -> *mut nigiri_timetable_t;
}
extern "C" {
    pub fn nigiri_destroy(t: *const nigiri_timetable_t);
}
extern "C" {
    pub fn nigiri_get_start_day_ts(t: *const nigiri_timetable_t) -> i64;
}
extern "C" {
    pub fn nigiri_get_day_count(t: *const nigiri_timetable_t) -> u16;
}
extern "C" {
    pub fn nigiri_get_transport_count(t: *const nigiri_timetable_t) -> u32;
}
extern "C" {
    pub fn nigiri_get_transport(t: *const nigiri_timetable_t, idx: u32) -> *mut nigiri_transport_t;
}
extern "C" {
    pub fn nigiri_destroy_transport(transport: *const nigiri_transport_t);
}
extern "C" {
    pub fn nigiri_is_transport_active(
        t: *const nigiri_timetable_t,
        transport_idx: u32,
        day_idx: u16,
    ) -> bool;
}
extern "C" {
    pub fn nigiri_get_route(t: *const nigiri_timetable_t, idx: u32) -> *mut nigiri_route_t;
}
extern "C" {
    pub fn nigiri_destroy_route(route: *const nigiri_route_t);
}
extern "C" {
    pub fn nigiri_get_location_count(t: *const nigiri_timetable_t) -> u32;
}
extern "C" {
    pub fn nigiri_get_location(t: *const nigiri_timetable_t, idx: u32) -> *mut nigiri_location_t;
}
extern "C" {
    pub fn nigiri_destroy_location(location: *const nigiri_location_t);
}
extern "C" {
    pub fn nigiri_update_with_rt(
        t: *const nigiri_timetable_t,
        gtfsrt_pb_path: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<
            unsafe extern "C" fn(arg1: nigiri_event_change_t, context: *mut ::std::os::raw::c_void),
        >,
        context: *mut ::std::os::raw::c_void,
    );
}
