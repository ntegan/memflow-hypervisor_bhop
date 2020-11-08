use memflow::connector::inventory::ConnectorInventory;
use memflow::mem::virt_mem::VirtualMemory;
use memflow::types::Address;
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

    println!("\n[ hypervisor bhop ]");


    let process_name = "csgo.exe";
    let vk_spacebar = 0x20;
    let vk_insert = 0x2D;
    let process_module_name = "client.dll";
    let dwLocalPlayer = 0xD3DD14;
    let m_fFlags = 0x104;
    let dwForceJump = 0x51fbfa8;

    let mut local_player = Address::from(0u32);
    let process_info = kernel.process_info(process_name)?;
    let mut process = Win32Process::with_kernel_ref(&mut kernel, process_info);
    let modules = process.module_list()?;
    let process_mod = modules.into_iter().find(|m| m.name == process_module_name)
        .ok_or(Error::Other("Could not find the module"))?;
    local_player = process.virt_mem.virt_read_addr32(process_mod.base + dwLocalPlayer)?;
    println!("csgo: got local player {:#?}", local_player);
    println!("csgo: doing bhop");
    process.destroy();


    Ok(())
}

