# rapl-rust-test

This does not work on Windows because `readmsr` requires kernel access. It will require a kernel driver to make it work.

## Test for CPU

https://github.com/RRZE-HPC/likwid/issues/373

`ls -la /dev/cpu/*/msr`
