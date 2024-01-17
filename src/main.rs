#![feature(custom_test_frameworks)]
#![no_main] // disable all Rust-level entry points
#![no_std]  // don't link the rust standard library
#![reexport_test_harness_main = "test_main"]
#![test_runner(curt_os::test_runner)]

use core::panic::PanicInfo;
use curt_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  curt_os::init();

  // stack_overflow();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  curt_os::test_panic_handler(info)
}

#[cfg(test)]
#[allow(unconditional_recursion, dead_code)]
fn stack_overflow() {
  stack_overflow();
}