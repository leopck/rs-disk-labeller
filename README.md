# Disk Labeler

Disk Labeler is a Rust-based command-line tool that allows you to label different types of filesystems on your disks. It supports ext2/ext3/ext4, vfat/fat32, ntfs, and swap filesystems. This tool also provides a way to verify the label using `lsblk`.

## Features

- Detect filesystem type
- Label ext2/ext3/ext4, vfat/fat32, ntfs, and swap filesystems
- Verify labels using `lsblk`
- Handles error and provides meaningful output

## Prerequisites

Before you can use Disk Labeler, ensure you have the following installed:

- Rust (latest stable version)
- `lsblk` (part of `util-linux` package)
- `blkid` (part of `util-linux` package)
- Filesystem-specific tools:
  - `e2label` for ext2/ext3/ext4 filesystems
  - `fatlabel` for vfat/fat32 filesystems
  - `ntfslabel` for ntfs filesystems
  - `mkswap` for swap filesystems

You can install these prerequisites on a Debian-based system using:

```sh
sudo apt-get update
sudo apt-get install -y util-linux ntfs-3g dosfstools e2fsprogs
```

## Installation

Clone the repository to your local machine:

```sh
[git clone https://github.com/yourusername/disk_labeler.git](https://github.com/leopck/rs-disk-labeller/)
cd disk_labeler
```

## Compilation

To compile the project, ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

Once Rust is installed, compile the project using `cargo`:

```sh
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Usage

### Command-line Arguments

- `<device>`: The device to be labeled (e.g., `/dev/sda1`, `/dev/loop0p1`).
- `<label>`: The new label for the device.

### Example Usage

To label a device:

```sh
./target/release/disk_labeler <device> <label>
```

For example, to label `/dev/sda1` with `new_label`:

```sh
./target/release/disk_labeler /dev/sda1 new_label
```

### Verifying the Label

You can verify the label using the `lsblk` command:

```sh
lsblk -o NAME,LABEL
```

## Example Workflow

### Creating and Using a Loopback Device for Testing

1. **Create a Loopback Device:**

```sh
dd if=/dev/zero of=mock_disk.img bs=1M count=100
sudo losetup /dev/loop0 mock_disk.img
```

2. **Partition the Loopback Device:**

```sh
sudo parted /dev/loop0 mklabel gpt
sudo parted /dev/loop0 mkpart primary ext4 1MiB 100MiB
sudo mkfs.ext4 /dev/loop0p1
```

3. **Run Disk Labeler on the Loopback Device:**

```sh
./target/release/disk_labeler /dev/loop0p1 new_label
```

4. **Verify the Label:**

```sh
lsblk -o NAME,LABEL
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or fixes.

## License

This project is licensed under the MIT License.
