# radix-sort

Single file radix sort implemented for u32, i32 and u32 pair (sorted by either of its coordinates). Radix of size 8 bits. Sorts array in 5 passes, should be faster than std::sort for anything above 10k entries(maybe even 1k, haven't tested). 

Based on radix sort by Pierre Terdiman, published at [http://codercorner.com/RadixSortRevisited.htm](http://codercorner.com/RadixSortRevisited.htm), with select optimizations published by Michael Herf at [http://stereopsis.com/radix.html](http://stereopsis.com/radix.html).

I did modify it a bit so the bucket count is reused as the offset table.

You can probably modify the code so only one additional array is allocated instead of two, but you'd need to change the type (for example a constant size array would work). 

Also, I've looked at [https://docs.rs/radsort/latest/radsort/index.html](https://docs.rs/radsort/latest/radsort/index.html) implementation, and there's a bunch of unsafe voodoo with array pointers to not have to require 2 additional vecs. 
See [https://docs.rs/crate/radsort/latest/source/src/sort.rs](https://docs.rs/crate/radsort/latest/source/src/sort.rs) DoubleBuffer struct.

Anyways, this is not supposed to be optimal, just faster than the standard (un)stable sort, as that one is a O(n log n) algorithm.

