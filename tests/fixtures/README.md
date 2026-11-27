# Motor configuration fixture

The mcconf-*.bin files are synthetic configurations, not hardware captures.
Each includes its big-endian configuration signature, but no command
ID, frame header or CRC. Field order, widths and scales come from the official
[VESC Tool 6.05 schema](https://github.com/vedderb/vesc_tool/blob/master/res/config/6.05/parameters_mcconf.xml).

The fixtures were serialized independently from each official XML schema,
not from the Rust decoder offsets. Zero-based serialized field index `i` sets
floating-point values to `(i + 1) / 1024`, booleans to `i % 2`, enum values to
`i % enum_count`, and integer/bitfield values to `i % 100 + 1` (negative for signed
integers). Six overrides retain the report-related example values:
l_current_max=178.75, l_abs_current_max=268.125, l_max_duty=0.95,
l_current_max_scale=0.8, foc_fw_current_max=100.5, foc_fw_duty_start=0.9.
Fixed-point fields are truncated after scaling, as in the firmware.

`tests/mcconf/groups.rs` checks every transmitted setting in every supported
layout, including signed values, arrays and fields absent in some layouts.
These are synthetic fixtures; physical-controller verification remains pending.

| Schema | Signature | Configuration bytes |
| --- | --- | --- |
| 6.05 | 1065524471 | 477 |
| 6.06 | 788332866 | 483 |
| 7.00 | 1470992211 | 488 |
| 8.00 | 471314414 | 492 |

Each fixture uses its corresponding official schema under the same res/config
path. Signatures were independently recomputed using VESC Tool's getSignature:
CRC32C of each serialized field's name, type number, transmission type number
(default 0), and enum names, concatenated in serialization order.
