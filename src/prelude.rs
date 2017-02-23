pub type VectorHandler = Option<extern "C" fn()>;

#[allow(unused)]
pub extern "C" fn breakpoint() {
    unsafe {
        asm!("bkpt");
    }

    loop {}
}

#[macro_export]
macro_rules! count_tts {
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $_k:tt $_l:tt $_m:tt $_n:tt $_o:tt
     $_p:tt $_q:tt $_r:tt $_s:tt $_t:tt
     $($tail:tt)*)
        => {20usize + count_tts!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $($tail:tt)*)
        => {10usize + count_tts!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $($tail:tt)*)
        => {5usize + count_tts!($($tail)*)};
    ($_a:tt
     $($tail:tt)*)
        => {1usize + count_tts!($($tail)*)};
    () => {0usize};
}

#[macro_export]
macro_rules! vectors {
    () => {};
    ( $($x:expr),+) => {
    use stm32::prelude::VectorHandler;
    build_table!(count_tts!($($x)+),$($x),+);
    };
}

#[macro_export]
macro_rules! build_table {

    ( $c:expr, $( $x:expr ),+ ) => {
        #[export_name = "_VECTORS"]
        #[allow(unused)]
        #[no_mangle]
        pub static VECTORS: [VectorHandler; $c ] =
         [ $($x),+ ];
    }
}

#[macro_export]
macro_rules! iomap {
    () => {};
    ($( $templ:ident @ $adr:tt -> $dev:tt);+) => {
    $(
        pub const $dev: *mut $templ = $adr as *mut $templ;
    )+
    };
}

#[macro_export]
macro_rules! io {
 (with $dev:ident do $op:ident ($($args:expr),+) ) => {
    unsafe { (*$dev).$op($($args),*) }
 };
  ( $dev:ident . $op:ident ($($args:expr),+) ) => {
    unsafe { (*$dev).$op($($args),*) }
 };
( $dev:ident . $port:ident = $val:expr ) => {
 unsafe {
            ::core::intrinsics::volatile_store(&mut (*$dev).$port, $val as u32);
        }
 };
 ( set $port:ident in $dev:ident to $val:expr ) => {
 unsafe {
            ::core::intrinsics::volatile_store(&mut (*$dev).$port, $val as u32);
        }
 };
 ( $dev:ident . $port:ident ) => {
 unsafe {
    ::core::intrinsics::volatile_load::<u32>(&((*$dev).$port))
        }
 };
 (get $port:ident from $dev:ident) => {
 unsafe {
    ::core::intrinsics::volatile_load::<u32>(&((*$dev).$port))
        }
 };
}
