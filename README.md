![master build](https://github.com/dkubiszews/rust-lru-cache/actions/workflows/rust.yml/badge.svg)

# Overview

LRU cache implememtnation in Rust.

# Example

```
use rust_lru_cache::dkubiszewski::LruCache;

let mut cache = LruCache::new(2);

cache.put(1, 15);
cache.put(2, 50);

println!("{}", cache.get(&1).unwrap());
println!("{}", cache.get(&2).unwrap());
```