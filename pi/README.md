Build the binary

`cargo build --release`

Test the binary

```
time ./target/release/pi
3.1415926445762157

real	0m0,120s
user	0m0,118s
sys	0m0,002s
```

Compare with python :

```
time python3 pi.py
3.1415926445762157

real	0m15,018s
user	0m14,990s
sys	0m0,008s
```


