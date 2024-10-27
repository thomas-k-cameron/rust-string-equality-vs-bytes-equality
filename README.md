# Optimizing Rust lang's match statement for compile time known strings

I created a match statement where each match arm is a uuid v4 that I randomly generated. When I compared it against new random uuid v4, it showed that match statement with bytes were lot faster.

These two functions will always produce the same reuslt, however, the benchmark shows that bytes variant is faster.

LINK: <https://rust.godbolt.org/z/fbnP1Ph9c>

```rust
fn str_match(totally_random_uuid_v4: &str) -> i64
{
    match totally_random_uuid_v4 {
        "68975c1f-bb5e-4640-a1bd-66bcce78b73e" => 1,
        "0cbb46f2-2bed-4168-8263-501dad40f515" => 2,
        // lots of uuid-s
        _ => -1
    }
}

fn bytes_match(totally_random_uuid_v4: &str) -> i64
{
    match totally_random_uuid_v4.as_bytes() {
        b"68975c1f-bb5e-4640-a1bd-66bcce78b73e" => 1,
        b"0cbb46f2-2bed-4168-8263-501dad40f515" => 2,
        // lots of uuid-s
        _ => -1
    }
}
```

I ran a benchmark on my github codespace with different number of match arms, and here is a benchmark result.
This is the result that I got running on github codespace instance.  

| string or bytes | number of arms | median time |
| --------------- | -------------- | ----------- |
| string          | 10             | 31.083 ns   |
| bytes           | 10             | 3.2405 ns   |
| string          | 100            | 294.98 ns   |
| bytes           | 100            | 3.7074 ns   |
| string          | 500            | 1.4685 µs   |
| bytes           | 500            | 4.6413 ns   |
| string          | 1000           | 3.0453 µs   |
| bytes           | 1000           | 4.3324 ns   |

You can find the code **[HERE](https://github.com/thomas-k-cameron/articles/stuff/rust-string-equality-vs-bytes-equality)**.

## What does the assembly code look lie?  

As you can see it on compiler explorer (<https://rust.godbolt.org/z/fbnP1Ph9c>), the match statement uses `bcmp`, a `libc` function for comparing byte-by-byte, to evaluate the equality of a `string`. However, when the data type is `&[u8]`, it creates an assembly instruction that is an equivalent of `strcmp` in c lang.

## Conclusion

Rust implements String's `Eq` on compiler's side that implementation is not visible as Rust code.

On compiler explorer, you can see that string is byte-packed, and Rust performs a SIMD compare when the string is longer.

<https://rust.godbolt.org/z/P6qEvj8n8>

I think String's Eq is implemented in a way that reflects the context more than bytes.  
However, I haven't been able to find an example where string's compare for equality is faster than bytes.
