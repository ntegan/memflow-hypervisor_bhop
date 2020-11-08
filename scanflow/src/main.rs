use memflow::connector::inventory::ConnectorInventory;
use memflow::mem::virt_mem::VirtualMemory;
use memflow::error::*;

use memflow_win32::win32::{Kernel, Win32Process};
use memflow_win32::{Error, Result};

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode};

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();

    let inventory = unsafe { ConnectorInventory::try_new()? };
    let connector = unsafe { inventory.create_connector_default("kvm")? };

    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()?;


    Ok(())
}
