# LicheeRV Nano — temporary identification notes

## APL extended workspaces (LIB 2)

Installed from `../../sw-vibe-coding/sw-apl-workspaces/ws` on 2026-09-28:
all nine workspace files and `library.json` are in `/root/sw-apl/ws/lib2`.
Registered as `2 EXTENDED` in `/root/.config/sw-apl/config.toml` using an
absolute library path. All transferred files passed SHA-256 verification.
APL modes 70 and 75 both listed LIB 2; loading COURSE in mode 70 and DRILL in
mode 75 succeeded. Start a new APL process to read the configuration; an
already-running `sw-apl-server` needs a restart to pick it up.

```sh
cd /root/sw-apl
./sw-apl
```

Then use `)LIB 2`, `)LOAD 2 COURSE`, and `START`.

### Mode-specific servers

Installed executable scripts in `/root/sw-apl/serve70.sh` and
`/root/sw-apl/serve75.sh`; source copies are in
[`scripts/licheerv-nano`](../scripts/).
Run either script on the board from any directory. It prints the host-side
connection command, then runs the server in the foreground; Ctrl-C stops it.

| Script | Host connection command |
|---|---|
| `/root/sw-apl/serve70.sh` | `aplterm --connect 10.105.250.1:2770` |
| `/root/sw-apl/serve75.sh` | `aplterm --connect 10.105.250.1:2775` |

Both modes can run concurrently. Each allows two sessions, uses
`/root/sw-apl` for libraries, and explicitly registers LIB 2 as EXTENDED.
Browser listeners remain on board loopback ports 8370 and 8375. Existing
servers on port 2741 are unaffected. Both scripts were launched and verified
over the line protocol: mode 70 reported (A), mode 75 reported (B), and each
listed LIB 2 correctly. Temporary validation servers were stopped after testing.

Inspection date: 2026-09-28. Filename retains the requested spelling;
Sipeed spells the product **LicheeRV Nano**.

## Identification status

**Update after flashing:** The owner reported the flash script completed with
SUCCESS, moved the card into the Nano, and powered it on. At 14:50:27 the board
enumerated as **`359f:2120`, manufacturer `sipeed`, product `licheervnano`**,
USB serial **`e5c8c9b01519c372`**. This confirms the running firmware identifies
as a Sipeed LicheeRV Nano. The exact networking variant and PCB revision remain
unverified.

The firmware exposed two USB network interfaces:

| Host interface | Driver | Host address |
|---|---|---|
| `enp0s20f0u9i2` | `cdc_ncm` | `10.105.250.100/24` |
| `enp0s20f0u9` | `rndis_host` | `10.105.251.100/24` |

`10.105.250.1` initially responded with ports 22, 80, and 443 closed. A later
check found SSH port 22 open, and login with the factory account succeeded.
The host logged a transmit timeout on the RNDIS
interface; the NCM path was responsive. The earlier repeating CVITEK connection
was replaced by the running firmware's USB network gadget.

## Connecting and running applications

Verified SSH login from the attached host:

```sh
ssh root@10.105.250.1
```

Factory password: `root`. The default account is also documented in
[Sipeed's connection guide](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/5_peripheral.html).
Use `passwd` on the board to change it. The host's address on this USB link is
`10.105.250.100`; the board's address is `10.105.250.1`.

Read directly through SSH after boot:

| Property | Running system |
|---|---|
| OS | Buildroot 2023.11.2, version `-gd4003f15b` |
| Kernel | Linux `5.10.4-tag-` |
| Architecture | `riscv64` |
| Hostname | `licheervnano-69fa` |
| Device-tree model | `LicheeRv Nano` |
| RAM visible to Linux (`free -m`) | 128 MiB total, approximately 93 MiB available at inspection |
| Root filesystem | `/dev/mmcblk0p2`, 14.6 GiB, approximately 13.2 GiB available |
| Python | `/usr/bin/python3` present |
| Native C compiler | Neither `gcc` nor `cc` found in PATH |

The root partition has expanded to use the card. The published physical memory
is 256 MB, but this image exposes about 128 MiB to Linux; the remaining memory's
allocation has not been inspected.

This is an embedded Linux environment. Applications are files on the microSD
filesystem; updating an application normally means copying it over SSH, not
rewriting the OS image. Buildroot is not Debian and does not provide the usual
Debian `apt` workflow.

For a Python application, run these commands on the host (with a local
`app.py` file):

```sh
ssh root@10.105.250.1 'mkdir -p /root/apps'
ssh root@10.105.250.1 'cat > /root/apps/app.py' < app.py
ssh root@10.105.250.1 'python3 /root/apps/app.py'
```

For a compiled application, build a Linux RISC-V executable with the matching
Sipeed SDK/toolchain. This release's board configuration selects
`riscv64-unknown-linux-musl-`. Copy the resulting `myapp` executable from the host:

```sh
ssh root@10.105.250.1 'cat > /root/myapp' < myapp
ssh root@10.105.250.1 'chmod +x /root/myapp && /root/myapp'
```

Use a compatible ABI and libraries; host x86-64 or ESP32 firmware binaries will
not run here. See the [official SDK](https://github.com/sipeed/LicheeRV-Nano-Build)
and [board build configuration](https://github.com/sipeed/LicheeRV-Nano-Build/blob/d4003f/build/boards/sg200x/sg2002_licheervnano_sd/sg2002_licheervnano_sd_defconfig).
For automatic startup, integrate the application into `/etc/init.d/` or the
existing `/etc/rc.local`, preserving existing startup commands. No application
or startup configuration was changed during this inspection.

### Rust example (subsequent verification)

The repository now contains [hello-lychee](../hello-lychee/README.md), built with
Rust 1.88.0 for `riscv64gc-unknown-linux-musl` using `rust-lld` and static linking.
The executable was copied to `/tmp/hello-lychee` using SCP and successfully run
on the board. Greeting, help, version, and invalid-argument handling were checked.
Both `/usr/bin/scp` and `/usr/libexec/sftp-server` were found on this image, and
modern SCP file transfer works. See the example README for rebuild, persistent
copy, and SSH execution commands.

Possible projects include Python automation, sensor logging with GPIO/I2C/SPI,
small network services, audio capture/playback, and camera/display applications.
NPU inference requires compatible models and the vendor runtime. Networking
beyond USB depends on the installed board variant and its configuration.

### Before flashing

The owner replaced the Lonely Binary ESP32-S3 with a Lychee board. A new
**CVITEK USB Com Port** was detected on USB topology path `1-9`.
This is consistent with the reported LicheeRV Nano, but the USB descriptors
alone do not establish the exact board model, variant, PCB revision, or SoC.
No running Linux shell was obtained and no memory or storage capacity was
read from the board.

The Hosyond ESP32-S3 remains connected as `/dev/ttyACM0`, with USB serial
`E0:72:A1:D6:87:CC`. It is a separate device.

## Directly observed USB details

| Property | Observed value |
|---|---|
| Vendor/product ID | `3346:1000` |
| Manufacturer string | `CVITEK` |
| Product string | `USB Com Port` |
| USB serial string | `123456789ABC` |
| Negotiated speed | High Speed, 480 Mbps |
| USB descriptor version | 2.01 |
| Device release descriptor | 2.00 (not a confirmed PCB revision) |
| Interface | CDC ACM communications and CDC data |
| Linux driver | `cdc_acm` |
| Transient serial device | `/dev/ttyACM1` |
| Host USB topology path | `1-9` |
| Power descriptor | Bus-powered, maximum 500 mA (not measured consumption) |

The serial string looks generic; it should not be treated as a proven unique
hardware identifier. USB device numbers and Linux serial port numbers may change.

## Connection behavior

The board repeatedly enumerated approximately every 8–9 seconds, remained
connected for about one second, then disconnected. Kernel messages showed this
pattern repeatedly between 14:15 and 14:16 local host time. A later descriptor
capture caught it as bus 001 device 057.

Representative kernel events:

```text
usb 1-9: New USB device found, idVendor=3346, idProduct=1000, bcdDevice= 2.00
usb 1-9: Product: USB Com Port
usb 1-9: Manufacturer: CVITEK
usb 1-9: SerialNumber: 123456789ABC
cdc_acm 1-9:1.0: ttyACM1: USB ACM device
usb 1-9: USB disconnect
```

This explains why a single `lsusb` scan initially missed it. No new USB network
interface was present during the network-interface snapshot. The repeating
connection is observed behavior, not a confirmed diagnosis: boot state, boot
media, firmware, and power/cable issues have not been distinguished.

## Published LicheeRV Nano specifications

These are manufacturer specifications, not measurements of this unit.

| Feature | Published specification |
|---|---|
| SoC | SOPHGO SG2002 |
| Main CPU | Selectable 1 GHz RISC-V C906 or Arm Cortex-A53 |
| Additional cores | 700 MHz RISC-V C906; 25–300 MHz 8051 |
| RAM | 256 MB DDR3 |
| NPU | 1 TOPS INT8; BF16 support |
| Boot storage | microSD/TF or SD NAND; SD NAND pads beneath card slot |
| USB | One USB 2.0 OTG Type-C port |
| Expansion | Two 14-pin, 2.54 mm headers; SPI, UART, I2C |
| Video | MIPI CSI camera input and MIPI DSI display output |
| Audio | Onboard microphone and speaker amplifier |
| Networking options | E suffix: 100 Mbps Ethernet; W suffix: dual-band Wi-Fi 6 and BLE 5.4 |
| Controls | RESET and BOOT buttons; power and user LEDs |
| Supported systems | Buildroot Linux / Debian |
| Dimensions | 22.86 × 35.56 mm |

Source: [Sipeed LicheeRV Nano specifications](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/1_intro).
The installed networking variant and selected CPU architecture remain unverified.

## Inspection scope and follow-up

### SD image preparation (2026-09-28)

The owner confirmed no microSD card was installed during the USB checks.
A nominal 16 GB card was subsequently detected in a Super Top SY-T18 USB
reader (`14cd:121f`), as `/dev/sdc`, with capacity 15,931,539,456 bytes.
Its existing unmounted partition was VFAT. The reader's stable path is
`/dev/disk/by-id/usb-Mass_Storage_Device_121F20110712-0:0`.

Downloaded the official [Sipeed release 20260114](https://github.com/sipeed/LicheeRV-Nano-Build/releases/tag/20260114):

- Archive: `2026-01-14-16-03-d4003f.tar.xz`
- Archive SHA-256 (matches GitHub's published asset digest):
  `d6478170e923615ca28c97592a2c68a67971e6d07fcb967371b58791938698dd`
- Extracted image: `2026-01-14-16-03-d4003f.img`, 1,694,499,328 bytes;
  fits the card.
- Image SHA-256 (computed locally):
  `79ef71f3cf2a0732466fc48f9783a7f8ae674d2642833bcc52bf872229ec6524`
- Download directory: `/disk1/tmp/licheerv-nano/`

**Flashing completed:** the owner ran the script with sudo and reported SUCCESS.
The agent observed the script's `cmp` verification progressing before completion.
Initially `sudo` required a password, and `pkexec` could not create an
authentication agent in this session. The prepared script checks reader identity,
capacity, mount state, and image checksum before writing, then flushes and
compares every image byte against the card:

```sh
sudo bash /disk1/tmp/licheerv-nano/flash-card.sh
```

This command erases the identified card. After successful verification, unplug
the board's power, insert the card, and reconnect power to try normal boot.

### Original inspection

Inspection used `lsusb`, `lsusb -t`, `lsusb -d 3346:1000 -v`, `ip -brief link`,
serial-device listings, and recent kernel logs. No firmware, boot media, or board
settings were changed.

To complete identification, inspect the board markings and boot media, then
capture its UART boot log or access a running system to read the model, CPU,
RAM, and storage information. See the
[Sipeed unboxing and setup guide](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/2_unbox.html).
