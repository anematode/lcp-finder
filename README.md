Example usage:

```
$ cargo run --release -- --file ../stockfish/src/stockfish
Found 5 instructions that would cause a length-changing prefix stall on Intel CPUs.
==========
   144fb: 66 81 fb ff 0f                                cmp bx,0FFFh
   1450d: 66 81 e6 00 0f                                and si,0F00h
   14782: 66 25 00 0f                                   and ax,0F00h
   2d840: 66 81 fa 02 7d                                cmp dx,7D02h
   31625: 66 3d 02 7d                                   cmp ax,7D02h
```
