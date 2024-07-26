use std::collections::BTreeMap;
use rustc_hash::FxHashMap;
use std::mem::size_of;

fn estimate_fxhashmap_memory<K, V>(map: &FxHashMap<K, V>) -> usize {
    let capacity = map.capacity();
    let entry_size = size_of::<K>() + size_of::<V>() + size_of::<usize>();  // Key + Value + HashValue
    let bucket_size = size_of::<usize>();  // Pointer size for each bucket
    
    let array_size = capacity * bucket_size;
    let entries_size = map.len() * entry_size;
    let overhead = size_of::<FxHashMap<K, V>>();
    
    array_size + entries_size + overhead
}

fn estimate_btreemap_memory<K, V>(map: &BTreeMap<K, V>) -> usize {
    // Assuming a B-tree node contains about 16 elements on average
    const ELEMENTS_PER_NODE: usize = 16;
    let node_count = (map.len() + ELEMENTS_PER_NODE - 1) / ELEMENTS_PER_NODE;
    
    let entry_size = size_of::<K>() + size_of::<V>();
    let node_overhead = size_of::<usize>() * (ELEMENTS_PER_NODE + 1);  // Pointers to children
    
    let entries_size = map.len() * entry_size;
    let nodes_size = node_count * (ELEMENTS_PER_NODE * entry_size + node_overhead);
    let overhead = size_of::<BTreeMap<K, V>>();
    
    entries_size + nodes_size + overhead
}

fn main() {
    let mut fxhash_map = FxHashMap::default();
    let mut btree_map = BTreeMap::new();

    for i in 0..100_000 {
        fxhash_map.insert(i as u64, i as u64);
        btree_map.insert(i as u64, i as u64);
    }

    let fxhash_bytes = estimate_fxhashmap_memory(&fxhash_map);
    let btree_bytes = estimate_btreemap_memory(&btree_map);

    println!("FxHashMap with 100,000 u64 -> u64 elements:");
    println!("  Estimated memory usage: {} bytes", fxhash_bytes);
    println!("  Number of elements: {}", fxhash_map.len());
    println!("  Capacity: {}", fxhash_map.capacity());

    println!("\nBTreeMap with 100,000 u64 -> u64 elements:");
    println!("  Estimated memory usage: {} bytes", btree_bytes);
    println!("  Number of elements: {}", btree_map.len());

    println!("\nMemory usage comparison:");
    println!("  FxHashMap / BTreeMap ratio: {:.2}", fxhash_bytes as f64 / btree_bytes as f64);
}