#![feature(custom_test_frameworks)]
#![no_main] // disable all Rust-level entry points
#![no_std]  // don't link the rust standard library
#![reexport_test_harness_main = "test_main"]
#![test_runner(curt_os::test_runner)]

use core::panic::PanicInfo;
use curt_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("                   __  ____  _____  ");
  println!("  _______  _______/ /_/ __ \\/ ___/  ");
  println!(" / ___/ / / / ___/ __/ / / /\\__ \\   ");
  println!("/ /__/ /_/ / /  / /_/ /_/ /___/ /   ");
  println!("\\___/\\__,_/_/   \\__/\\____//____/  ");
  println!("");
  println!("A Rust-Based Operating System for x86 architecture");
  println!("Written by Curtis Wilson");
  println!("Based on the tutorial by Philipp Oppermann");

  curt_os::init();

  #[allow(unconditional_recursion, dead_code)]
  fn stack_overflow() {
    stack_overflow();
  }

  // stack_overflow();

  #[cfg(test)]
  test_main();

  curt_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  curt_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  curt_os::test_panic_handler(info)
}