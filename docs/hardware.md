# Sipeed LicheeRV Nano hardware

Documentation checked 2026-10-06. Sipeed spells the board **LicheeRV Nano**;
the local directory retains `LycheeRV-Nano`. The unit has booted Linux and
identified itself as LicheeRV Nano, but its networking variant and PCB revision
have not been established. Published capabilities are not measurements.

## Board and processor

| Component | Published specification |
| --- | --- |
| SoC | SOPHGO SG2002 |
| Main processor | Selectable 1 GHz RISC-V C906 or Arm Cortex-A53 |
| Additional processors | 700 MHz RISC-V C906 and 25–300 MHz 8051 |
| Memory | 256 MB DDR3 |
| Boot/storage | microSD/TF or SD NAND; SD NAND pads below the card slot |
| USB | USB 2.0 OTG Type-C |
| Vision/display | MIPI CSI input and two-lane MIPI DSI output |
| Audio | Onboard microphone and speaker amplifier |
| Expansion | Two 14-pin headers; UART, SPI and I2C |
| Network variants | E: 100 Mbps Ethernet; W: dual-band Wi-Fi 6 and BLE 5.4 |
| Dimensions | 22.86 × 35.56 mm |

Source: [Sipeed board specifications](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/1_intro).
Selectable CPU modes do not mean both main-processor architectures run Linux
simultaneously. This unit's observed Linux architecture is **riscv64**, not
AArch64. Ordinary programs must match the running OS architecture and ABI.

## Neural processing unit

SG2002 includes a neural accelerator called an NPU in Sipeed's board
description and TPU in SOPHGO's software stack. The advertised capability is
**1 TOPS INT8**, with **BF16 support**. Sipeed does not specify an equal
1 TOPS rate for BF16. No NPU throughput or inference accuracy was measured
on this unit. [Sipeed specification](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/1_intro)

Use the SG200x/CVITEK-compatible model compiler and runtime, not Rockchip
RKNN. SOPHGO's SG200x SDK includes TPU runtime support and the `cvimodel`
model format. Match the model compiler, chip target, firmware and runtime
versions; confirm the precise SG2002 target in the selected SDK before
conversion. A `.bmodel` intended for a different SOPHGO accelerator or an
`.rknn` file is not interchangeable with this deployment path.
[SOPHGO SG200x developer manual](https://github.com/sophgo/sophgo-doc/blob/main/SG200X/Software_Developers_Manual/SG200x_Software_Developer%27s_Manual_en.md)

The general workflow is host-side model conversion, supported quantization
and calibration, deployment of the compiled model, then board-side inference
through the matching runtime. TPU-MLIR is one relevant compiler ecosystem,
but its support varies by chip and version; follow the SG200x examples
rather than assuming every feature advertised for TPU-MLIR applies here.
[SOPHGO TPU-MLIR](https://github.com/sophgo/tpu-mlir)

The accelerator is separate from the C906's CPU/vector instruction set.
CPU compilation, RISC-V vectorization, and an ordinary k/Rust executable do
not automatically invoke it. A k integration would call a vendor inference
adapter, supplying tensors and returning results to the interpreter.

Practical assessment: the larger memory and advertised INT8 rate make this
a useful candidate for small vision models, but do not establish a twofold
speedup over RV1103. Operator coverage, quantization, memory reservations,
input sizes, preprocessing and runtime overhead determine actual results.
Begin with a known-supported model; do not extrapolate to arbitrary LLMs
from TOPS or from TPU-MLIR's support for other SOPHGO products.

## Observations of this unit

The prior bring-up notes record these results from 2026-09-28:

| Item | Observed result |
| --- | --- |
| OS architecture | `riscv64` |
| Device-tree model | `LicheeRv Nano` |
| OS | Buildroot 2023.11.2; Linux `5.10.4-tag-` |
| Linux-visible RAM | 128 MiB total; approximately 93 MiB available then |
| Storage | `/dev/mmcblk0p2`, approximately 14.6 GiB after expansion |
| USB identity | `359f:2120`, product `licheervnano` |
| USB networking | CDC NCM and RNDIS; NCM was the responsive connection |
| Network | Board `10.105.250.1`, host `10.105.250.100/24` over NCM |
| Applications | SSH, file transfer and a static RISC-V Rust program verified |
| NPU | No inference benchmark or runtime verification recorded |

The published 256 MB physical DDR and observed 128 MiB Linux memory are
different quantities. The reservation breakdown has not been inspected;
do not attribute all missing memory to the TPU without checking the image's
memory layout. Available-memory values are historical snapshots.

Local evidence: `docs/bringup.md` in this board repository. No fresh board inspection was performed for this document.

## First NPU validation

1. Record board markings/network variant, image version, `uname -a`,
   `/proc/cpuinfo`, and `/proc/meminfo`.
2. Identify the installed TPU runtime and reserved-memory configuration.
3. Run an SG2002-compatible vendor sample using fixed input and a matching
   compiled model; record the exact chip target and compiler/runtime versions.
4. Compare outputs with the source model, then measure memory use and warmed
   latency. Test INT8 and BF16 only where supported by that model and SDK.
5. Add camera preprocessing after fixed-input inference works.

## Sources

- [Sipeed LicheeRV Nano specifications](https://wiki.sipeed.com/hardware/en/lichee/RV_Nano/1_intro)
- [SOPHGO SG200x software developer manual](https://github.com/sophgo/sophgo-doc/blob/main/SG200X/Software_Developers_Manual/SG200x_Software_Developer%27s_Manual_en.md)
- [SOPHGO TPU-MLIR compiler](https://github.com/sophgo/tpu-mlir)
