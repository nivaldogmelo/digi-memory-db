#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(not(feature = "std"))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
		     // use panic_abort as _; // requires nightly
		     // use panic_itm as _; // logs messages over ITM; requires ITM support
		     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f1xx_hal::{
    flash::{FlashExt, FlashSize, SectorSize},
    pac,
};

// use cortex_m::asm;
#[cfg(not(feature = "std"))]
use cortex_m_rt::entry;

#[cfg(not(feature = "std"))]
use cortex_m_semihosting::hprintln;

mod domain;
mod services;

use domain::DataStore;
use services::Database;

#[cfg(not(feature = "std"))]
#[entry]
fn entry() -> ! {
    use domain::{Key, Value};

    let mut db = Database::new();

    let key = Key::parse("hello").unwrap();
    let value = Value::parse("world").unwrap();

    let _insert = db.set(key.clone(), value);

    let v = db.get(&key);
    hprintln!("value: {:?}", v);

    loop {}

    // use stm32f1xx_hal::{
    //	flash::{FlashExt, FlashSize, SectorSize},
    //	pac,
    // };

    // let dp = pac::Peripherals::take().unwrap();
    // let mut flash = dp.FLASH.constrain();

    // let sector_sz = SectorSize::Sz1K;
    // let flash_sz = FlashSize::Sz64K;

    // let mut writer = flash.writer(sector_sz, flash_sz);

    // let mut database = Database::new();

    // // clean init /////////////////////////////////////////////////////////////
    // let e = writer.erase(PERSISTENT_OFFSET, PERSISTENT_STORAGE_SIZE);
    // if e.is_err() {
    //	hprintln!("FLASH_START: {}", PERSISTENT_OFFSET);
    //	hprintln!("Erase failed: {:?}", e);
    //	loop {}
    // }

    // // normal init ////////////////////////////////////////////////////////////
    // let r = writer.read(PERSISTENT_OFFSET, PERSISTENT_STORAGE_SIZE);
    // if r.is_err() {
    //	hprintln!("FLASH_START: {}", PERSISTENT_OFFSET);
    //	hprintln!("Erase failed: {:?}", e);
    //	loop {}
    // }

    // let data = Value::parse("Hello, World!").unwrap();
    // let key = Key::parse("key").unwrap();

    // let _ = database.set(key, data);

    // let key = Key::parse("key").unwrap();
    // let get = database.get(&key).unwrap();
    // hprintln!("Get: {:?}", get);

    // let w = store.write(&mut data);
    // if w.is_err() {
    //	hprintln!("Write failed: {:?}", w);
    //	loop {}
    // }

    // let buffer = Vec::<u8, 1024>::new();

    // let buffer = store.read(4_usize, buffer);
    // if buffer.is_err() {
    //	hprintln!("Read failed: {:?}", buffer);
    //	loop {}
    // }

    // let r = buffer.unwrap();
    // hprintln!("Read successful: {:?}", r);

    // if writer.write(PERSISTENT_OFFSET, &data).is_err() {
    //	hprintln!("Write failed");
    //	loop {}
    // }

    // hprintln!("Write successful");
}

#[cfg(feature = "std")]
fn main() {}
