# quick-cal
CLI for quick decimal, hexadecimal, binary, and octal basic calcs.

## Examples
### Add
```
$ qcal add 30 2 5
> dec: 37         hex: 25         oct: 45         bin: 100101
```

```
$ qcal add 21 14
> dec: 35         hex: 23         oct: 43         bin: 100011
```

```
$ qcal add 30 0xFF o24 b111
> dec: 312        hex: 138        oct: 470        bin: 100111000
```

### Subtract
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
$ qcal mul 30 2 5
> dec: 300       hex: 12c         oct: 454        bin: 100101100
```
```
$ qcal mul 21 14
> dec: 294        hex: 126        oct: 446         bin: 100100110
```

### Divide
```
$ qcal div 30 2 5
> dec: 3          hex: 3          oct: 3          bin: 11
```
```
$ qcal mul 21 14
> dec: 1          hex: 1          oct: 1          bin: 1
```
