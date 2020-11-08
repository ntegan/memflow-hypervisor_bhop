use memflow::connector::inventory::ConnectorInventory;
use memflow::mem::virt_mem::VirtualMemory;

use memflow_win32::win32::{Kernel, Keyboard, Win32Process};
use memflow_win32::{Error, Result};

use simplelog::{LevelFilter, TermLogger, Config, TerminalMode};

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();

    let inventory = unsafe { ConnectorInventory::try_new()? };
    let connector = unsafe { inventory.create_connector_default("kvm")? };

    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()?;



    // see https://github.com/Zer0Mem0ry/KernelBhop
    println!("\n[ hypervisor bhop ]");

    // If we don't do anything inside loop, sleep will limit
    // frequency to 333 loop iterations per second
    let max_update_frequency_hertz = 800u64;
    // (1000 milliseconds / 1 second) * 
    //      (1 second / max_update_frequency_hertz) ==
    //      (milliseconds to sleep per iteration)
    //
    // e.g. 333 hz, sleep for 3 milliseconds
    let time_sleep_ms = 1000.0f64 / (max_update_frequency_hertz as f64);

    let process_name = "csgo.exe";
    let process_module_name = "client.dll";
    let vk_spacebar = 0x20;
    let vk_insert = 0x2D;
    #[allow(non_snake_case)]
    let dwLocalPlayer = 0xD3DD14;
    #[allow(non_snake_case)]
    let m_fFlags = 0x104;
    #[allow(non_snake_case)]
    let dwForceJump = 0x51fbfa8;

    let mut local_player;
    let process_info = kernel.process_info(process_name)?;
    let mut process = Win32Process::with_kernel_ref(&mut kernel, process_info);
    let modules = process.module_list()?;
    let process_mod = modules.into_iter().find(|m| m.name == process_module_name)
        .ok_or(Error::Other("Could not find the module"))?;
    local_player = process.virt_mem.virt_read_addr32(process_mod.base + dwLocalPlayer)?;
    println!("csgo: got local player {:#?}", local_player);
    println!("csgo: doing bhop");
    process.destroy();


    let keyboard = Keyboard::try_with(&mut kernel)?;
    let process_info = kernel.process_info(process_name)?;
    let mut process = Win32Process::with_kernel_ref(&mut kernel, process_info);
    let modules = process.module_list()?;
    let process_mod = modules.into_iter().find(|m| m.name == process_module_name)
        .ok_or(Error::Other("Could not find the module"))?;
    loop {
        let mut yes_insert = false;
        let mut yes_spacebar = false;
        let keyboard_state = keyboard.state_with_process(&mut process)?;
        if keyboard_state.is_down(vk_spacebar) {
            yes_spacebar = true;
        }
        if keyboard_state.is_down(vk_insert) {
            yes_insert = true;
        }

        if yes_spacebar {
            local_player = process.virt_mem.virt_read_addr32(process_mod.base + dwLocalPlayer)?;
            let flags: u8 = process.virt_mem.virt_read(local_player + m_fFlags)?;
            if (flags & 1) == 1 {
                process.virt_mem.virt_write(process_mod.base + dwForceJump, &6)?;
            }
        }
        if yes_insert {
            println!("csgo: insert pressed, exiting");
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(time_sleep_ms as u64));
    }
    Ok(())
}

