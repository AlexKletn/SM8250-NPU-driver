# SM8250 NPU driver

[Русская версия](README.ru.md)

A Linux driver project for the Qualcomm SM8250 neural processing unit. The
project uses a Rust workspace. Any board-specific configuration belongs in the
integrating project. Firmware and model files are supplied separately.

## Workspace

| Path | Responsibility |
| --- | --- |
| `crates/npu-domain` | Hardware-independent lifecycle of the accelerator, firmware, and graphs. |
| `crates/npu-sm8250` | SM8250 firmware protocol, IPC queues, and startup and shutdown sequences. |
| `crates/npucc-sm8250` | SM8250 clock, reset, and power-domain descriptions. |
| `kernel/` | Linux module entry points, kernel adapters, and Kbuild. |

## Linux module

Build the accelerator module against a configured kernel build with Rust support
and its matching `Module.symvers`:

```sh
make -C "$KERNEL_BUILD" ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- M="$PWD/kernel" modules
```

`sm8250_npu.ko` matches `qcom,sm8250-npu`. Probe runs a firmware boot and
shutdown check through Qualcomm PAS. The device tree must provide a
`firmware-name` property with the signed MDT image path, and a
`memory-region` for the image. Install the signed image and its segments
through the usual Linux firmware search path.

### Board configuration

The clock controller accepts an optional `qcom,cx-performance-state` cell for
the CX performance vote. Boards provide the value in their Device Tree.

### Optional graph validation

To enable a validation graph, the accelerator node provides the
`qcom,validation-aco`, `qcom,validation-weights`, `qcom,validation-relocations`,
and `qcom,validation-input` firmware paths. `qcom,validation-output` optionally
provides a reference tensor for byte-for-byte comparison.

Each of the following properties has the prefix `qcom,validation-`.

| Property suffix | Value |
| --- | --- |
| `aco-size`, `weights-size`, `workspace-size`, `input-size`, `output-size` | Sizes in bytes. |
| `relocation-count` | Number of 8-byte relocation records. |
| `network-id`, `first-block-size`, `layer-count` | Graph load parameters. The first block size is in bytes. |
| `input-patch-id`, `output-patch-id` | Tensor patch identifiers. |
| `load-patches` | Optional array of six-cell firmware patch records. |

The validation runs the graph twice and compares the outputs. The integrating
project supplies the layout, model, and tensors.
