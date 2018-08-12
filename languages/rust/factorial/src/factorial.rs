#![no_std]
#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![feature(lang_items)]

// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[no_mangle]
pub extern "C" fn factorial(n: i32) -> i32 {
    if n > 1 {
        return n * factorial(n - 1);
    } else {
        return 1;
    }
}

#[test]
fn test_factorial() {
    let factorial_table = [
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 6),
        (4, 24),
        (5, 120),
        (6, 720),
        (7, 5040),
        (8, 40320),
        (9, 362880),
        (10, 3628800),
        (11, 39916800),
        (12, 479001600),
        (13, 6227020800),
        (14, 87178291200),
        (15, 1307674368000),
        (16, 20922789888000),
        (17, 355687428096000),
        (18, 6402373705728000),
        (19, 121645100408832000),
        (20, 2432902008176640000),
    ];

    for pair in factorial_table.iter() {
        assert_eq!(factorial(pair.0), pair.1);
    }
}
