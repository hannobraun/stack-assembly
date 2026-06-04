# Changelog

## v1.0.0 (2026-06-04)

- Update documentation ([#78], [#120], [#121], [#122])
- Extract example host into a dedicated crate ([#79], [#80])
- Improve command-line interface of example host ([#81])
- Add `Memory::to_i32_slice`, `to_u32_slice` ([#82])
- Expand integer literals to support full range of unsigned integers ([#83])
- Refer explicitly to "operand stack" instead of just "stack" ([#84])
- Add `call`, `call_either`, and `return` operators ([#85])
- Update dependencies ([#87], [#90], [#91], [#92], [#93], [#94], [#110], [#112], [#113], [#114], [#117], [#118], [#125])
- Add `Value::to_bool` and implement `From<bool>` for `Value` ([#89])
- Add `Script`, `OperatorIndex`; decouple compilation from `Eval` ([#96], [#104], [#106], [#115], [#123])
- Fix some possible runtime panics ([#97], [#99], [#100])
- Remove `Value::to_usize` ([#101], [#111])
- Implement `Default` for `Eval`, `Memory`, and `OperandStack` ([#105])
- Simplify effect handling ([#107])
- Support mapping operator that triggers an effect to source code ([#108], [#109])
- Expose call stack to the host ([#116])
- Add Snake game as an expanded example ([#119], [#124])

[#78]: https://github.com/hannobraun/stack-assembly/pull/78
[#79]: https://github.com/hannobraun/stack-assembly/pull/79
[#80]: https://github.com/hannobraun/stack-assembly/pull/80
[#81]: https://github.com/hannobraun/stack-assembly/pull/81
[#82]: https://github.com/hannobraun/stack-assembly/pull/82
[#83]: https://github.com/hannobraun/stack-assembly/pull/83
[#84]: https://github.com/hannobraun/stack-assembly/pull/84
[#85]: https://github.com/hannobraun/stack-assembly/pull/85
[#87]: https://github.com/hannobraun/stack-assembly/pull/87
[#89]: https://github.com/hannobraun/stack-assembly/pull/89
[#90]: https://github.com/hannobraun/stack-assembly/pull/90
[#91]: https://github.com/hannobraun/stack-assembly/pull/91
[#92]: https://github.com/hannobraun/stack-assembly/pull/92
[#93]: https://github.com/hannobraun/stack-assembly/pull/93
[#94]: https://github.com/hannobraun/stack-assembly/pull/94
[#96]: https://github.com/hannobraun/stack-assembly/pull/96
[#97]: https://github.com/hannobraun/stack-assembly/pull/97
[#99]: https://github.com/hannobraun/stack-assembly/pull/99
[#100]: https://github.com/hannobraun/stack-assembly/pull/100
[#101]: https://github.com/hannobraun/stack-assembly/pull/101
[#104]: https://github.com/hannobraun/stack-assembly/pull/104
[#105]: https://github.com/hannobraun/stack-assembly/pull/105
[#106]: https://github.com/hannobraun/stack-assembly/pull/106
[#107]: https://github.com/hannobraun/stack-assembly/pull/107
[#108]: https://github.com/hannobraun/stack-assembly/pull/108
[#109]: https://github.com/hannobraun/stack-assembly/pull/109
[#110]: https://github.com/hannobraun/stack-assembly/pull/110
[#111]: https://github.com/hannobraun/stack-assembly/pull/111
[#112]: https://github.com/hannobraun/stack-assembly/pull/112
[#113]: https://github.com/hannobraun/stack-assembly/pull/113
[#114]: https://github.com/hannobraun/stack-assembly/pull/114
[#115]: https://github.com/hannobraun/stack-assembly/pull/115
[#116]: https://github.com/hannobraun/stack-assembly/pull/116
[#117]: https://github.com/hannobraun/stack-assembly/pull/117
[#118]: https://github.com/hannobraun/stack-assembly/pull/118
[#119]: https://github.com/hannobraun/stack-assembly/pull/119
[#120]: https://github.com/hannobraun/stack-assembly/pull/120
[#121]: https://github.com/hannobraun/stack-assembly/pull/121
[#122]: https://github.com/hannobraun/stack-assembly/pull/122
[#123]: https://github.com/hannobraun/stack-assembly/pull/123
[#124]: https://github.com/hannobraun/stack-assembly/pull/124
[#125]: https://github.com/hannobraun/stack-assembly/pull/125

## v0.1.0 (2025-12-15)

Initial release. See [release announcement](https://www.hannobraun.com/stack-assembly-0.1/) for more information.
