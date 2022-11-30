# VM16
## Attempt at writing 16-bit virtual machine in Rust following [Low Byte Productions guide](https://www.youtube.com/playlist?list=PLP29wDx6QmW5DdwpdwHCRJsEubS5NrQ9b)
Doesn't have any functionality as of now, just ten registers, seven instructions and a handful of bad design decisions
- main.rs contains an example program in assembly and code to run it
- constants.rs contains register indexes and instruction codes 
- byte_ops.rs contains useful functions for u8 and u16 conversions
- memory.rs contains memory and registers api
- cpu.rs contains crude cpu api
