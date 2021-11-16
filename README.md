# language-speed-test
compare the speeds of programming languages by computing the answer to https://projecteuler.net/problem=12

the programs include increment, decrement, divide, square root, comparators, and casting between `int` and `float`

1. C (0.19s)
2. Rust (1.58s)

## C (0.19s)
```txt
> gcc c.c -lm -o c
> date +%S.%N; ./c > /dev/null; date +%S.%N
52.282206335
52.470491317
```

## Rust (1.58s)
```txt
> rustc rust.rs
> date +%S.%N; ./rust > /dev/null; date +%S.%N
57.223556672
58.803250322
```
