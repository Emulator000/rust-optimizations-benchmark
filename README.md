# Rust optimizations benchmark

This project is a benchmark comparison between different Cargo compiler options (LTO, codegen-units, etc.)

## Profiles available

There are three main profile available (plus the standard `release`):
- `release-light`
- `release-medium`
- `release-extreme`

## Original benchmark source-codes

This repository is based on the work of [kostya](https://github.com/kostya), original repo could be found [here](https://github.com/kostya/benchmarks).

## Sample benchmark

Here a list of executed single-threads local benchmark split by optimization level.

### release-light

```
$ cargo run --profile=release-light
ZYXWVUTSRQPONMLKJIHGFEDCBA
[brainfuck] elapsed: 1.80s

encode aaaa... to YWFh...: 1431666688, 0.46385828
decode YWFh... to aaaa...: 1073741824, 0.6469056
[base64] elapsed: 1.11s

-18.6716666
[matmul] elapsed: 1.41ms

Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
[json] elapsed: 19.00µs

[323381, 323383, 3233803, 3233809, 3233851, 3233863, 3233873, 3233887, 3233897]
[primes] elapsed: 140.19ms
```

### release-medium

```
$ cargo run --profile=release-medium
ZYXWVUTSRQPONMLKJIHGFEDCBA
[brainfuck] elapsed: 1.51s

encode aaaa... to YWFh...: 1431666688, 0.5003623
decode YWFh... to aaaa...: 1073741824, 0.476198
[base64] elapsed: 976.88ms

-18.6716666
[matmul] elapsed: 1.07ms

Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
[json] elapsed: 23.26µs

[323381, 323383, 3233803, 3233809, 3233851, 3233863, 3233873, 3233887, 3233897]
[primes] elapsed: 133.76ms
```

### release-extreme

```
$ cargo run --profile=release-extreme
ZYXWVUTSRQPONMLKJIHGFEDCBA
[brainfuck] elapsed: 1.82s

encode aaaa... to YWFh...: 1431666688, 0.48744947
decode YWFh... to aaaa...: 1073741824, 0.4698408
[base64] elapsed: 957.65ms

-18.6716666
[matmul] elapsed: 1.07ms

Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
Coordinate { x: 2e0, y: 5e-1, z: 0.25 }
[json] elapsed: 14.67µs

[323381, 323383, 3233803, 3233809, 3233851, 3233863, 3233873, 3233887, 3233897]
[primes] elapsed: 145.75ms

```

WARNING: Those results may vary in each execution, overall `medium` and `extreme` doesn't seem to differ substantially.

## Machine specs for the sample benchmark

Lenovo ThinkPad Z16 AMD (16″)

```
$ hwinfo --short --cpu
cpu:                                                            
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 2525 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1858 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1875 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1766 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 4166 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1556 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1985 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 3300 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 3075 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 2744 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1850 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 3300 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1850 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 3300 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 1859 MHz
                       AMD Ryzen 9 PRO 6950H with Radeon Graphics, 2246 MHz
```
