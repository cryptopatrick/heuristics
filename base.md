# Rust Implementation Guide for Computer Science Heuristics

This document maps each heuristic from `computer-science.md` to specific Rust crates, standard library types, and implementation patterns.

---

## General-Purpose Performance Heuristics

### Need O(1) average-case lookups or inserts?
**Action:** Default to hash tables unless order matters or you need to minimize worst-case latency.

- **Std types:** `std::collections::HashMap`, `std::collections::HashSet`
- **Crates:**
  - `hashbrown` - Fast hash table implementation (used by std)
  - `rustc-hash` - Fast hash functions (FxHashMap)
  - `ahash` - Fast, DOS-resistant hashing
- **When to use:** Detecting `Vec::iter().find()` in loops, or linear searches
- **Example:**
```rust
// Instead of:
let mut items = Vec::new();
items.iter().find(|x| x.id == target_id);

// Use:
use std::collections::HashMap;
let mut items: HashMap<u64, Item> = HashMap::new();
items.get(&target_id);
```

---

### Need fast search on static or mostly-static data?
**Action:** Sort the data once and use binary search (or keep it sorted with a balanced tree).

- **Std types:** `Vec::binary_search`, `std::collections::BTreeMap`, `std::collections::BTreeSet`
- **Crates:**
  - `binary-search-tree` - Custom BST implementations
- **When to use:** Repeated searches on data that rarely changes
- **Example:**
```rust
// Sort once:
let mut data = vec![5, 2, 8, 1, 9];
data.sort_unstable();

// Fast searches:
match data.binary_search(&5) {
    Ok(pos) => println!("Found at {}", pos),
    Err(_) => println!("Not found"),
}

// Or maintain sorted order:
use std::collections::BTreeSet;
let mut sorted_data = BTreeSet::new();
sorted_data.insert(5);
```

---

### Need maximum write throughput?
**Action:** Prefer append-only logs or structures (immutable + new versions).

- **Crates:**
  - `append-only-bytes` - Append-only byte buffer
  - `sled` - Embedded DB with append-only design
  - `redb` - Embedded DB optimized for append-only writes
- **When to use:** High write volume, audit trails, event logs
- **Example:**
```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut log = OpenOptions::new()
    .create(true)
    .append(true)
    .open("events.log")?;

writeln!(log, "{:?}: {}", std::time::SystemTime::now(), event)?;
```

---

### Need absolute fastest possible reads/writes?
**Action:** Keep hot data fully in-memory; only spill to disk when necessary.

- **Std types:** `Vec`, `HashMap`, `Box`, `Arc`
- **Crates:**
  - `dashmap` - Concurrent in-memory hash map
  - `evmap` - Eventually-consistent concurrent hash map
  - `parking_lot` - Faster synchronization primitives
- **When to use:** Caching frequently accessed data
- **Example:**
```rust
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static! {
    static ref CACHE: Arc<RwLock<HashMap<String, Data>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
```

---

### Need to save space and can tolerate small false-positive rates?
**Action:** Reach for probabilistic data structures.

- **Crates:**
  - `probabilistic-collections` - Bloom filters, Count-Min Sketch, HyperLogLog
  - `bloom` - Simple Bloom filter
  - `hyperloglogplus` - Cardinality estimation
  - `count-min-sketch` - Frequency estimation
- **When to use:** Large-scale deduplication, membership testing, cardinality estimation
- **Example:**
```rust
use probabilistic_collections::bloom::BloomFilter;

let mut filter = BloomFilter::new(1000, 0.01); // 1000 items, 1% false positive
filter.insert(&"key1");
filter.contains(&"key1"); // true
filter.contains(&"key2"); // probably false
```

---

### Need to cache expensive results?
**Action:** Always add an LRU or TTL cache layer in front of slow operations.

- **Crates:**
  - `lru` - LRU cache implementation
  - `moka` - High-performance concurrent cache with TTL
  - `cached` - Procedural macro for function memoization
  - `quick_cache` - Fast, lightweight cache
- **When to use:** Expensive computations, database queries, API calls
- **Example:**
```rust
use lru::LruCache;
use std::num::NonZeroUsize;

let mut cache = LruCache::new(NonZeroUsize::new(100).unwrap());
cache.put("key", expensive_computation());

// Or with memoization:
use cached::proc_macro::cached;

#[cached(size=100)]
fn expensive_function(n: u64) -> u64 {
    // expensive computation
    n * n
}
```

---

## Disk & Persistence Heuristics

### Need log(n) lookups when working with disk-backed data?
**Action:** Use B-trees or B+ trees.

- **Std types:** `std::collections::BTreeMap` (in-memory)
- **Crates:**
  - `sled` - B+ tree embedded database
  - `redb` - B+ tree embedded database
  - `bptree` - B+ tree implementation
- **When to use:** Persistent key-value stores, range queries
- **Example:**
```rust
use sled::Db;

let db: Db = sled::open("my_db")?;
db.insert(b"key", b"value")?;
let value = db.get(b"key")?;
```

---

### Need to check "does this exist?" millions of times with tiny memory?
**Action:** Use a Bloom filter (accept ~1% false positives).

- **Crates:** `probabilistic-collections`, `bloom`, `bloomfilter`
- **When to use:** Large-scale existence checks, pre-filtering before expensive lookups
- **Example:**
```rust
use bloom::BloomFilter;

let mut bloom = BloomFilter::with_rate(0.01, 1_000_000);
for item in large_dataset {
    bloom.insert(&item);
}

if bloom.contains(&query) {
    // Maybe exists, check actual storage
    check_database(query);
}
```

---

### Need durability without blocking writes?
**Action:** Write to a write-ahead log (WAL) first, then checkpoint asynchronously.

- **Crates:**
  - `wal` - Write-ahead log implementation
  - `sled` - Built-in WAL
  - `redb` - Built-in WAL
- **When to use:** Database-like systems, critical data persistence
- **Example:**
```rust
// Most embedded DBs handle this internally
let db = sled::open("my_db")?;
db.insert(b"key", b"value")?; // Writes to WAL first
db.flush_async().await?; // Async checkpoint
```

---

### Need fast analytical/column scans?
**Action:** Store data in columnar format.

- **Crates:**
  - `parquet` - Apache Parquet format
  - `arrow` - Apache Arrow columnar format
  - `polars` - Fast DataFrame library with columnar storage
- **When to use:** Analytics, OLAP queries, aggregations
- **Example:**
```rust
use polars::prelude::*;

let df = df! {
    "id" => &[1, 2, 3],
    "value" => &[100, 200, 300],
}?;

// Fast column operations
let sum = df.column("value")?.sum()?;
```

---

### Need extremely high write throughput on disk?
**Action:** Use an LSM-tree.

- **Crates:**
  - `rocksdb` - RocksDB bindings (LSM-tree)
  - `sled` - Embedded DB with LSM-like design
  - `fjall` - Pure Rust LSM-tree storage engine
- **When to use:** Write-heavy workloads, time-series data
- **Example:**
```rust
use rocksdb::DB;

let db = DB::open_default("data")?;
db.put(b"key", b"value")?; // Very fast writes
```

---

### Need to compress data aggressively?
**Action:** Apply compression (zstd, LZ4, Snappy) unless CPU is the bottleneck.

- **Crates:**
  - `zstd` - Zstandard compression
  - `lz4` - LZ4 compression
  - `snap` - Snappy compression
  - `flate2` - DEFLATE/gzip compression
- **When to use:** Large data storage, network transfers
- **Example:**
```rust
use zstd::stream::{encode_all, decode_all};

let compressed = encode_all(&data[..], 3)?; // compression level 3
let decompressed = decode_all(&compressed[..])?;
```

---

### Need delta updates or versioned data with low storage?
**Action:** Store only diffs with delta compression.

- **Crates:**
  - `similar` - Text diffing
  - `diffy` - Text and binary diffs
  - `xdelta3` - Binary delta compression
- **When to use:** Version control, incremental backups
- **Example:**
```rust
use similar::{ChangeTag, TextDiff};

let diff = TextDiff::from_lines(old_text, new_text);
for change in diff.iter_all_changes() {
    match change.tag() {
        ChangeTag::Insert => println!("+{}", change),
        ChangeTag::Delete => println!("-{}", change),
        _ => {}
    }
}
```

---

## Distributed Systems Heuristics

### Need horizontal scalability?
**Action:** Shard data across nodes (by key range or hash).

- **Crates:**
  - `consistent-hash` - Consistent hashing implementation
  - `jumphash` - Jump consistent hash
- **When to use:** Distributed databases, caches, load balancing
- **Example:**
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn shard_key<T: Hash>(key: &T, num_shards: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % num_shards
}
```

---

### Need high availability and read scaling?
**Action:** Replicate data (leader-follower or multi-master).

- **Crates:**
  - `raft` - Raft consensus protocol
  - `async-raft` - Async Raft implementation
  - `openraft` - Modern Raft implementation
- **When to use:** Distributed consensus, replicated state machines
- **Example:**
```rust
// Typically implemented at the application level
// See openraft documentation for full examples
```

---

### Need minimal data movement when adding/removing nodes?
**Action:** Use consistent hashing + virtual nodes.

- **Crates:**
  - `consistent-hash` - Consistent hashing ring
  - `consistent-hash-ring` - Ring with virtual nodes
- **When to use:** Distributed caching (Memcached, Redis clusters)
- **Example:**
```rust
use consistent_hash::ConsistentHash;

let mut ring = ConsistentHash::new();
ring.add("node1", 150); // 150 virtual nodes
ring.add("node2", 150);

let node = ring.get("my_key"); // Minimal reassignment on node changes
```

---

### Need eventual consistency without coordination?
**Action:** Use CRDTs for counters, sets, maps, registers, etc.

- **Crates:**
  - `crdts` - CRDT implementations (counters, sets, maps)
  - `automerge` - Rich CRDTs for collaborative editing
- **When to use:** Offline-first apps, collaborative editing, distributed counters
- **Example:**
```rust
use crdts::{Orswot, CmRDT};

let mut set1 = Orswot::new();
let mut set2 = Orswot::new();

let op = set1.add("item", ctx1);
set2.apply(op); // Sets converge without coordination
```

---

### Need efficiency when building a distributed ledger or sync system?
**Action:** Use Merkle trees for fast proofs and reconciliation.

- **Crates:**
  - `merkle` - Merkle tree implementation
  - `rs-merkle` - Simple Merkle tree
  - `merkle-tree-stream` - Streaming Merkle tree
- **When to use:** Blockchain, data synchronization, tamper detection
- **Example:**
```rust
use rs_merkle::{MerkleTree, algorithms::Sha256};

let leaves = vec![b"data1", b"data2", b"data3"];
let tree = MerkleTree::<Sha256>::from_leaves(&leaves);
let root = tree.root(); // Compact verification
```

---

## Concurrency & Lock-Free Heuristics

### Need high-concurrency random access with simple implementation?
**Action:** Use skip lists.

- **Crates:**
  - `crossbeam-skiplist` - Lock-free concurrent skip list
  - `skiplist` - Skip list implementation
- **When to use:** Concurrent sorted collections
- **Example:**
```rust
use crossbeam_skiplist::SkipMap;

let map = SkipMap::new();
map.insert(1, "value1");
map.insert(2, "value2");

// Concurrent access is safe
let value = map.get(&1);
```

---

### Need concurrent data structure without locks?
**Action:** Prefer lock-free designs (atomics + retries) or copy-on-write.

- **Std types:** `std::sync::atomic::*`
- **Crates:**
  - `crossbeam` - Lock-free data structures and utilities
  - `arc-swap` - Lock-free Arc swapping
  - `lockfree` - Lock-free data structures
- **When to use:** High-contention scenarios, low-latency requirements
- **Example:**
```rust
use std::sync::atomic::{AtomicU64, Ordering};

let counter = AtomicU64::new(0);
counter.fetch_add(1, Ordering::SeqCst);

// Or use arc-swap for complex data:
use arc_swap::ArcSwap;
let data = ArcSwap::from_pointee(vec![1, 2, 3]);
data.store(Arc::new(vec![4, 5, 6]));
```

---

### Need shared immutable data with occasional mutations?
**Action:** Use copy-on-write or persistent data structures.

- **Crates:**
  - `im` - Immutable data structures (HashMap, Vector, etc.)
  - `rpds` - Persistent data structures
  - `arc-swap` - Copy-on-write pattern
- **When to use:** Functional programming patterns, snapshots
- **Example:**
```rust
use im::HashMap;

let map1 = HashMap::from([(1, "a"), (2, "b")]);
let map2 = map1.update(3, "c"); // map1 unchanged, structural sharing
```

---

## Specialized Data Structure Heuristics

### Need prefix matching or autocomplete?
**Action:** Use a Trie (or Radix/Patricia tree for memory savings).

- **Crates:**
  - `radix_trie` - Radix trie implementation
  - `qp-trie` - QP-trie (adaptive radix tree)
  - `trie-rs` - Fast trie library
- **When to use:** Autocomplete, prefix search, IP routing
- **Example:**
```rust
use radix_trie::Trie;

let mut trie = Trie::new();
trie.insert("test", 1);
trie.insert("testing", 2);

// Find all keys with prefix "test"
for (key, value) in trie.get_raw_descendant(&"test") {
    println!("{}: {}", key, value);
}
```

---

### Need bounded queue/FIFO with zero allocations after init?
**Action:** Use a ring/circular buffer.

- **Crates:**
  - `ringbuf` - Lock-free ring buffer
  - `circular-buffer` - Circular buffer
  - `heapless` - Fixed-capacity collections (no_std)
- **When to use:** Audio/video processing, embedded systems, fixed-size queues
- **Example:**
```rust
use ringbuf::HeapRb;

let rb = HeapRb::<i32>::new(10); // capacity 10
let (mut producer, mut consumer) = rb.split();

producer.try_push(1).unwrap();
let value = consumer.try_pop().unwrap();
```

---

### Need efficient range queries (sum, min, max) over arrays?
**Action:** Use segment trees or Fenwick trees (Binary Indexed Tree).

- **Crates:**
  - `segment-tree` - Segment tree implementation
  - `fenwick` - Fenwick tree (BIT)
- **When to use:** Range sum queries, range minimum queries
- **Example:**
```rust
// Typically hand-coded for competitive programming
// See crate documentation for usage examples
```

---

### Need priority queue with fast peek and extract-min/max?
**Action:** Use a binary heap.

- **Std types:** `std::collections::BinaryHeap`
- **Crates:**
  - `priority-queue` - Priority queue with changeable priorities
- **When to use:** Dijkstra's algorithm, task scheduling, top-K problems
- **Example:**
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(1);
heap.push(10);

assert_eq!(heap.pop(), Some(10)); // Max heap by default
```

---

### Need very fast union-find / connected components?
**Action:** Implement disjoint-set (Union-Find) with path compression + union-by-rank.

- **Crates:**
  - `union-find` - Union-find data structure
  - `disjoint-sets` - Disjoint set implementation
- **When to use:** Graph connectivity, Kruskal's algorithm
- **Example:**
```rust
use union_find::UnionFind;

let mut uf = UnionFind::new(10);
uf.union(0, 1);
uf.union(1, 2);
assert!(uf.equiv(0, 2));
```

---

### Need full-text search?
**Action:** Build an inverted index (term → document list).

- **Crates:**
  - `tantivy` - Full-text search engine (like Lucene)
  - `meilisearch-sdk` - Meilisearch client
- **When to use:** Search engines, document search
- **Example:**
```rust
use tantivy::*;

let mut index = Index::create_in_ram(schema);
let mut index_writer = index.writer(50_000_000)?;

index_writer.add_document(doc!(
    title_field => "My Document",
    body_field => "This is the content"
))?;
index_writer.commit()?;
```

---

### Need geospatial queries?
**Action:** Use R-tree, Quad-tree, or Geohash partitioning.

- **Crates:**
  - `rstar` - R*-tree for spatial indexing
  - `geo` - Geospatial primitives and algorithms
  - `geohash` - Geohash encoding/decoding
- **When to use:** Location-based services, spatial databases
- **Example:**
```rust
use rstar::RTree;
use geo::Point;

let mut tree = RTree::new();
tree.insert(Point::new(1.0, 2.0));
tree.insert(Point::new(3.0, 4.0));

// Nearest neighbor search
let nearest = tree.nearest_neighbor(&Point::new(2.0, 3.0));
```

---

### Need efficient data structure when manipulating huge strings with many splices?
**Action:** Use Rope data structure.

- **Crates:**
  - `ropey` - Fast rope for text editing
  - `crop` - Rope implementation
  - `xi-rope` - Rope from Xi editor
- **When to use:** Text editors, large document manipulation
- **Example:**
```rust
use ropey::Rope;

let mut rope = Rope::from_str("Hello world");
rope.insert(5, ", beautiful");
rope.remove(0..5);

let text = rope.to_string(); // ", beautiful world"
```

---

### Need fast substring search on static text?
**Action:** Build a suffix array or suffix tree.

- **Crates:**
  - `suffix` - Suffix array construction
  - `aho-corasick` - Multiple pattern matching
  - `memchr` - Fast byte searching
- **When to use:** Pattern matching, bioinformatics
- **Example:**
```rust
use suffix::SuffixTable;

let sa = SuffixTable::new("banana");
assert_eq!(sa.positions("ana"), &[1, 3]);
```

---

## System Design & Architecture Heuristics

### Need full audit trail and temporal queries?
**Action:** Use event sourcing (store events, derive state on read).

- **Crates:**
  - `eventually` - Event sourcing framework
  - `cqrs-es` - CQRS and Event Sourcing framework
- **When to use:** Financial systems, audit logs, domain-driven design
- **Example:**
```rust
// Store events instead of state
enum Event {
    AccountCreated { id: String },
    MoneyDeposited { amount: u64 },
    MoneyWithdrawn { amount: u64 },
}

// Derive current state by replaying events
fn apply_event(state: &mut Account, event: Event) {
    match event {
        Event::MoneyDeposited { amount } => state.balance += amount,
        Event::MoneyWithdrawn { amount } => state.balance -= amount,
        _ => {}
    }
}
```

---

### Need time-series data?
**Action:** Use specialized TSDB with downsampling + compression.

- **Crates:**
  - `influxdb` - InfluxDB client
  - `prometheus` - Prometheus client
  - `tikv-client` - TiKV distributed DB client
- **When to use:** Metrics, monitoring, IoT sensor data
- **Example:**
```rust
use influxdb::{Client, InfluxDbWriteable};

#[derive(InfluxDbWriteable)]
struct Metric {
    time: DateTime<Utc>,
    #[influxdb(tag)]
    host: String,
    value: f64,
}

let client = Client::new("http://localhost:8086", "mydb");
client.query(Metric { ... }).await?;
```

---

### Need speed when dealing with complex queries?
**Action:** Add materialized views or pre-aggregated summary tables.

- **Implementation:** Application-level caching, pre-computed results
- **Crates:**
  - `cached` - Memoization
  - `moka` - In-memory cache
- **When to use:** Expensive aggregations, repeated complex queries
- **Example:**
```rust
use cached::proc_macro::cached;

#[cached(time=3600)] // Cache for 1 hour
fn expensive_aggregation(filter: Filter) -> Summary {
    // Complex computation
    compute_summary(filter)
}
```

---

### Need fast throughput when dealing with many small operations?
**Action:** Batch them (network, disk, lock overhead gets amortized).

- **Crates:**
  - `tokio` - Async batching with channels
  - `crossbeam-channel` - Efficient channels for batching
- **When to use:** Database writes, API calls, disk I/O
- **Example:**
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

// Batch consumer
tokio::spawn(async move {
    let mut batch = Vec::with_capacity(100);
    while let Some(item) = rx.recv().await {
        batch.push(item);
        if batch.len() >= 100 {
            process_batch(&batch).await;
            batch.clear();
        }
    }
});
```

---

### Need flexibility when dealing with access patterns that change over time?
**Action:** Consider adaptive structures.

- **Crates:**
  - `qp-trie` - Adaptive radix tree
  - Built-in adaptive behaviors in many crates
- **When to use:** Unpredictable workloads, learning systems
- **Example:**
```rust
// Use data structures that adapt to workload
// E.g., splay trees (self-balancing based on access)
// Most Rust collections use adaptive strategies internally
```

---

### Need tamper detection + efficient sync between nodes?
**Action:** Merkle tree.

- **Crates:** `merkle`, `rs-merkle`, `merkle-tree-stream`
- **When to use:** Distributed sync, blockchain, git-like systems
- **Example:**
```rust
use rs_merkle::{MerkleTree, algorithms::Sha256};

let tree1 = MerkleTree::<Sha256>::from_leaves(&leaves1);
let tree2 = MerkleTree::<Sha256>::from_leaves(&leaves2);

if tree1.root() != tree2.root() {
    // Trees differ, need to sync
}
```

---

## Decision Checklist

When approaching a performance or architecture problem, apply these in order:

1. **Identify the primary bottleneck** → Profile to determine if it's CPU, memory, disk, network, latency, or throughput
   - Tools: `perf`, `flamegraph`, `cargo flamegraph`, `criterion`

2. **Understand the access patterns** → Random vs sequential, read-heavy vs write-heavy, single vs range
   - Instrument your code to measure actual patterns

3. **Assess exactness requirements** → Exact answers vs probabilistic approximations
   - Consider false positive rates and accuracy tradeoffs

4. **Evaluate data size constraints** → Known and bounded vs unbounded growth
   - Plan for memory limits and disk usage

5. **Determine workload concurrency** → Single-threaded, multi-threaded, or distributed
   - Use `rayon` for parallelism, `tokio`/`async-std` for async, `crossbeam` for concurrency primitives

6. **Confirm system distribution** → Single machine vs distributed system
   - Consider consistency, availability, partition tolerance tradeoffs

---

## Additional Resources

- **Performance profiling:** `cargo flamegraph`, `criterion`, `pprof`
- **Benchmarking:** `criterion`, `bencher`, `divan`
- **Async runtime:** `tokio`, `async-std`, `smol`
- **Parallelism:** `rayon`, `crossbeam`
- **Database:** `sqlx`, `diesel`, `sea-orm`
- **Serialization:** `serde`, `bincode`, `postcard`
