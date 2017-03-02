A series of benchmarks to compare several utf8 decoder.


So far rust built-in wins on my system.

```
test bench_encoding_ignore             ... bench:       1,530 ns/iter (+/- 4)
test bench_encoding_replace            ... bench:       1,531 ns/iter (+/- 19)
test bench_encoding_rs                 ... bench:         489 ns/iter (+/- 3)
test bench_encoding_rs_w_bom           ... bench:         484 ns/iter (+/- 8)
test bench_encoding_rs_wo_bom          ... bench:         452 ns/iter (+/- 2)
test bench_encoding_rs_wo_bom_and_repl ... bench:         455 ns/iter (+/- 2)
test bench_encoding_strict             ... bench:       1,531 ns/iter (+/- 25)
test bench_std_str                     ... bench:         344 ns/iter (+/- 1)
test bench_std_string                  ... bench:         324 ns/iter (+/- 1)
```
