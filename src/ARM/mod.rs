pub mod STM32;

pub type VectorHandler = Option<extern "C" fn()>;

#[allow(unused)]
pub extern "C" fn breakpoint() {
    unsafe {
        asm!("bkpt");
    }

    loop {}
}

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

macro_rules! vectors {
    () => {};
    ( $($x:expr),+) => {
    use ARM::VectorHandler;
    build_table!(count_tts!($($x)+),$($x),+);
    };
}

macro_rules! build_table {

    ( $c:expr, $( $x:expr ),+ ) => {
        #[export_name = "_VECTORS"]
        #[allow(unused)]
        #[no_mangle]
        pub static VECTORS: [VectorHandler; $c ] =
         [ $($x),+ ];
    }
}
