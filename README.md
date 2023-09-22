# rapl-rust-test

This does not work on Windows because `readmsr` requires kernel access. It will require a kernel driver to make it work

## RAPL test

https://github.com/djselbeck/rapl-read-ryzen

https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/

https://github.com/hubblo-org/windows-rapl-driver

https://github.com/amd/amd_energy

## Test for CPU

https://github.com/RRZE-HPC/likwid/issues/373

`ls -la /dev/cpu/*/msr`

`sudo modprobe msr`
