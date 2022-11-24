use std::collections::*;

const INVALID: i32 = 0;
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
    fn alloc(&mut self, id: i32, size: i32) -> bool;
    fn free(id: i32) -> bool;
    fn print();
}

struct FirstFitAllocator {
    alloc_chunk_list: Option<LinkedList<MemoryChunk>>,
    free_chunk_list: Option<LinkedList<MemoryChunk>>,
}

struct BestFitAllocator {
    alloc_chunk_list: Option<LinkedList<MemoryChunk>>,
    free_chunk_list: Option<LinkedList<MemoryChunk>>,
}

impl FirstFitAllocator {
    fn new(size: i32) -> Self {
        let list: LinkedList<MemoryChunk> = LinkedList::new();
        list.push_back(MemoryChunk::new(INVALID, 0, size));

        FirstFitAllocator {
            free_chunk_list: Some(list),
            alloc_chunk_list: None,
        }
    }
}

impl BestFitAllocator {
    fn new(size: i32) -> Self {
        let list: LinkedList<MemoryChunk> = LinkedList::new();
        list.push_back(MemoryChunk::new(INVALID, 0, size));

        BestFitAllocator {
            free_chunk_list: Some(list),
            alloc_chunk_list: None,
        }
    }
}

impl MemoryAllocator for FirstFitAllocator {
    fn alloc(&mut self, id: i32, size: i32) -> bool {
        for it in self.free_chunk_list.take().iter_mut() {
            let chunk: MemoryChunk = *it;
            if chunk.size >= size {
                self.alloc_chunk_list.take().push_back(MemoryChunk::new(
                    id,
                    chunk.begin,
                    chunk.begin + size,
                ));
                //delete this free chunk
                if self.free_chunk_list.take().empty() {
                    return false;
                }
                self.free_chunk_list.take().erase(it += 1);
                //add left memory
                self.free_chunk_list.take().cursor_back_mut();
                break;
            }
        }
        true
    }

    fn free(id: i32) -> bool {
        true
    }

    fn print() {}
}

impl MemoryAllocator for BestFitAllocator {
    fn alloc(&mut self, id: i32, size: i32) -> bool {}

    fn free(id: i32) -> bool {}

    fn print() {}
}
