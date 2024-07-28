use std::process::{Command, Output};
use std::env;
use std::str;

fn execute_command(command: &str, args: &[&str]) -> Output {
    println!("Executing command: {} {:?}", command, args); // Debug print
    Command::new(command)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {}", command))
}

fn is_block_device(device: &str) -> bool {
    let output = execute_command("lsblk", &["-no", "NAME", device]);
    output.status.success() && !String::from_utf8_lossy(&output.stdout).trim().is_empty()
}

fn get_filesystem_type(device: &str) -> String {
    let output = execute_command("blkid", &["-o", "value", "-s", "TYPE", device]);
    if !output.status.success() {
        eprintln!("blkid error: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    let fs_type = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if fs_type.is_empty() {
        eprintln!("Failed to detect filesystem type for {}", device);
        std::process::exit(1);
    }
    fs_type
}

fn label_disk(filesystem: &str, device: &str, label: &str) {
    match filesystem {
        "ext2" | "ext3" | "ext4" => {
            let output = execute_command("e2label", &[device, label]);
            if output.status.success() {
                println!("Successfully labeled {} with {}", device, label);
            } else {
                eprintln!("Failed to label {}: {}", device, String::from_utf8_lossy(&output.stderr));
            }
        }
        "vfat" | "fat32" => {
            let output = execute_command("fatlabel", &[device, label]);
            if output.status.success() {
                println!("Successfully labeled {} with {}", device, label);
            } else {
                eprintln!("Failed to label {}: {}", device, String::from_utf8_lossy(&output.stderr));
            }
        }
        "ntfs" => {
            let output = execute_command("ntfslabel", &[device, label]);
            if output.status.success() {
                println!("Successfully labeled {} with {}", device, label);
            } else {
                eprintln!("Failed to label {}: {}", device, String::from_utf8_lossy(&output.stderr));
            }
        }
        "swap" => {
            let output = execute_command("mkswap", &["-L", label, device]);
            if output.status.success() {
                println!("Successfully labeled {} with {}", device, label);
            } else {
                eprintln!("Failed to label {}: {}", device, String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => eprintln!("Unsupported filesystem: {}", filesystem),
    }
}

fn verify_label(device: &str) {
    let output = execute_command("lsblk", &["-o", "NAME,LABEL"]);
    println!("Verification of labels:\n{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <device> <label>", args[0]);
        return;
    }

    let device = &args[1];
    let label = &args[2];

    if !is_block_device(device) {
        eprintln!("Error: {} is not a valid block device", device);
        return;
    }

    let filesystem = get_filesystem_type(device);
    println!("Detected filesystem type: {}", filesystem);
    
    label_disk(&filesystem, device, label);
    verify_label(device);
}

