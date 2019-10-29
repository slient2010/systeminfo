extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    let sys = System::new();
    // // 获取挂载文件信息
    // match sys.mounts() {
    //     Ok(mounts) => {
    //         println!("\nMounts:");
    //         for mount in mounts.iter() {
    //             println!("{} ---{}---> {} (available {} of {})",
    //                      mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
    //         }
    //     }
    //     Err(x) => println!("\nMounts: error: {}", x)
    // }

    // 获取挂磁盘信息
    match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                println!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => println!("\nBlock statistics error: {}", x.to_string())
    }

    // 获取网卡信息
    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetworks:");
            for netif in netifs.values() {
                println!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }

    // 获取网卡流量信息
    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }

    // 获取电池状态
    match sys.battery_life() {
        Ok(battery) =>
            print!("\nBattery: {}%, {}h{}m remaining",
                   battery.remaining_capacity*100.0,
                   battery.remaining_time.as_secs() / 3600,
                   battery.remaining_time.as_secs() % 60),
        Err(x) => print!("\nBattery: error: {}", x)
    }
    
    // 获取系统开机信息
    match sys.on_ac_power() {
        Ok(power) => println!(", AC power: {}", power),
        Err(x) => println!(", AC power: error: {}", x)
    }

    //获取内存情况
    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }

    // 获取当前系统负载情况
    match sys.load_average() {
        Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\nLoad average: error: {}", x)
    }

    // 获取开机时间
    match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x)
    }

    // 获取系统启动时间
    match sys.boot_time() {
        Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        Err(x) => println!("\nBoot time: error: {}", x)
    }

    // CPU 负载
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU load: error: {}", x)
    }

    // 获取CPU 温度
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }
    // 获取socket状态
    match sys.socket_stats() {
        Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
    }
}
