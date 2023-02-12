extern crate cursock;
extern crate curerr;

mod command;
mod commands;
mod config;
mod consts;
mod sys;

use curerr::*;

use cursock::utils::*;
use command::Command;
use consts::COMMANDS;
use cursock::*;

use config::Config;

use std::{alloc, time};

static mut MEM_COUNTER: usize = 0;

struct CursedAllocator;

unsafe impl alloc::GlobalAlloc for CursedAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        MEM_COUNTER += layout.size();
        alloc::System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        alloc::System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: CursedAllocator = CursedAllocator;

fn main() {
    let start: time::Instant = time::Instant::now();

    let result: Result<(), CursedErrorHandle> = run_config();

    if let Err(err) = result {
        println!("Program ended with {}", err)
    }

    println!("Program took {} secconds", start.elapsed().as_secs_f64());
}

fn run_config() -> Result<(), CursedErrorHandle> {
    let args: Vec<String> = std::env::args().collect();

    let config: Config = match Config::from(&args) {
        Ok(config) => config,
        Err(err) => return Err(err),
    };

    if let Err(err) = config.run() {
        return Err(err);
    }

    Ok(())
}
