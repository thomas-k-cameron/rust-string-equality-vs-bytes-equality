# Equality Comparison for String Known at Compile Time with Length of 33 or More with Non-Repeating bytes is Slower than Array of `u8`

I created a function that performs equality comparison agaisnt a uuid4.
match statement where each match arm is a uuid v4 that I randomly generated. 

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

## How to reproduce

Simply run `cargo bench` on the root of this repository, and get the result with criterion.

## Why is it happening?

As you can see it on compiler explorer (<https://rust.godbolt.org/z/fbnP1Ph9c>), the match statement uses `bcmp`, a `libc` function for byte-by-byte conparison, to perform equality comparison. However, when the data type is `&[u8]`, it creates instruction that is an equivalent of `strcmp` in c lang.

When the length of a String is 33 or less, it will use other instructions for equality comparison.
