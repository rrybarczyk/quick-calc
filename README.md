`$ cargo run -- port 4848` -> still yields 4949
# quick-calc
A cli tool for quick decimal, hexadecimal, binary, and octal basic formatting and calculations.

```
$ cargo install quick-calc
```

## Examples

### Format as Decimal, Hexadecimal, Octal, and Binary
```
$ qcal format 0xdeadbeef
> dec: 3735928559       hex: 0xdeadbeef         oct: o33653337357           bin: b11011110101011011011111011101111
```

```
$ qcal format 0x1234 234 b011101 o24
> dec: 4660         hex: 0x1234         oct: o11064         bin: b1001000110100
> dec: 234          hex: 0xea           oct: o352           bin: b11101010
> dec: 29           hex: 0x1d           oct: o35            bin: b11101
> dec: 20           hex: 0x14           oct: o24            bin: b10100
```

### Swap Endianness of Hexadecimal
```
$ qcal endian 0x55bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000
> 0x000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55
```

```
$ qcal endian 0xdeadbeef abcdef56
> 0xefbeadde
> 0x56efcdab
```

### Count Byte Length of Hexadecimal
```
$ qcal bytelen 0xdeadbeef
> 0xdeadbeef:   4 bytes
```

```
$ qcal bytelen 0xdeadbeef 0x12
> 0xdeadbeef:   4 bytes
> 0x12:         1 byte
```
You can use `bytelen` or `len` to access this operation.

### Count Character Length
```
$ qcal charlen 0x1234 234 b011101 o24 4
> 0x1234:   4 chars
> 234:      3 chars
> b011101:  6 chars
> o24:      2 chars
```

### Add
```
$ qcal add 0xFF 30 o24 b111
> dec: 312        hex: 138        oct: 470        bin: 100111000
```

```
$ qcal add 30 2 5
> dec: 37         hex: 25         oct: 45         bin: 100101
```

```
$ qcal add 21 14
> dec: 35         hex: 23         oct: 43         bin: 100011
```

### Subtract
```
$ qcal sub 0xFF 30 o24 b111
> dec: 198        hex: c6         oct: 306        bin: 11000110
```

```
$ qcal sub 30 2 5
> dec: 23         hex: 17         oct: 27         bin: 10111
```

```
$ qcal sub 21 14
> dec: 7          hex: 7          oct: 7          bin: 111
```

### Multiply
```
$ qcal mul 0xFF 30 o24 b111
> dec: 1071000    hex: 105798     oct: 4053630    bin: 100000101011110011000
```

```
$ qcal mul 30 2 5
> dec: 300       hex: 12c         oct: 454        bin: 100101100
```

```
$ qcal mul 21 14
> dec: 294        hex: 126        oct: 446         bin: 100100110
```

### Divide
```
$ qcal div 0xFF 30 o24 b111
> dec: 0          hex: 0          oct: 0          bin: 0
```

```
$ qcal div 30 2 5
> dec: 3          hex: 3          oct: 3          bin: 11
```

```
$ qcal mul 21 14
> dec: 1          hex: 1          oct: 1          bin: 1
```
