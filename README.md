# reqstatic

A very thin wrapper of [reqwest](https://github.com/seanmonstar/reqwest)

This is intended to simplify the use of pooling and alleviate the following problems.

- [reqwest never close socket connection in windows ! · Issue #284 · seanmonstar/reqwest](https://github.com/seanmonstar/reqwest/issues/284)
- [[Windows] Presence notification fill OS socket queue · Issue #170 · nymtech/nym](https://github.com/nymtech/nym/issues/170)

## Usage

```rust
let resp = reqstatic::client().get("https://httpbin.org/ip")
  .await?
  .json::<HashMap<String, String>>()
  .await?;
```
