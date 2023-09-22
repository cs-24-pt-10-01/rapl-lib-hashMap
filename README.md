# rapl-rust-test

The repository for RAPL testing.

## Windows

Currently this does not work on Windows because `readmsr` requires kernel access. It will require a kernel driver to make it work. Intel Power Gadget can support it by design but it will only be for Intel processors in that case.

## RAPL test

https://github.com/djselbeck/rapl-read-ryzen

https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/

https://github.com/hubblo-org/windows-rapl-driver

https://github.com/amd/amd_energy

## Test for CPU

https://github.com/RRZE-HPC/likwid/issues/373

List the CPU's MSR's on Linux.

`ls -la /dev/cpu/*/msr`

Enable MSR.

`sudo modprobe msr`
