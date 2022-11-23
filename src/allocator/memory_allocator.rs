use std::collections::*;
struct MemoryChunk {
    id: i32,
    begin: i32,
    end: i32,
    size: i32,
}

impl MemoryChunk {
    fn new(id: i32, begin: i32, end: i32) -> Self {
        MemoryChunk {
            id,
            begin,
            end,
            size: end - begin,
        }
    }
}

trait MemoryAllocator {
    fn alloc(id: i32, size: i32) -> bool;
    fn free(id: i32) -> bool;
    fn print();
}

struct FirstFitAllocator {
    alloc_chunk_list: LinkedList<MemoryChunk>,
    free_chunk_list: LinkedList<MemoryChunk>,
}

struct BestFitAllocator {
    alloc_chunk_list: LinkedList<MemoryChunk>,
    free_chunk_list: LinkedList<MemoryChunk>,
}

impl MemoryAllocator for FirstFitAllocator {
    fn alloc(id: i32, size: i32) -> bool {}

    fn free(id: i32) -> bool {}

    fn print() {}
}

impl MemoryAllocator for BestFitAllocator {
    fn alloc(id: i32, size: i32) -> bool {}

    fn free(id: i32) -> bool {}

    fn print() {}
}
