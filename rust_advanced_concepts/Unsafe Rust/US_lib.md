Rust's standard library (`std`) provides various built-in libraries that can be used in both safe and unsafe code. Here's an exhaustive list of some key built-in libraries that are often used in conjunction with Unsafe Rust:

1. **std::ptr:**
   - Provides low-level pointer manipulation and dereferencing.
   - Functions like `read`, `write`, `copy`, `null`, `null_mut`, etc.

2. **std::mem:**
   - Offers memory-related operations.
   - Functions like `size_of`, `align_of`, `transmute`, `uninitialized`, etc.

3. **std::sync::atomic:**
   - Provides atomic operations for concurrency and synchronization.
   - Types like `AtomicBool`, `AtomicIsize`, `AtomicUsize`, etc.

4. **std::sync::Mutex and std::sync::RwLock:**
   - Offers synchronization primitives for multi-threading.
   - Provides mutex and read-write lock implementations.

5. **std::alloc:**
   - Provides basic memory allocation and deallocation functionality.
   - Functions like `alloc`, `dealloc`, `Layout`, etc.

6. **std::intrinsics:**
   - Offers access to compiler intrinsics for low-level operations.
   - Functions like `copy`, `write_bytes`, `volatile_copy_memory`, etc.

7. **std::arch:**
   - Provides access to architecture-specific intrinsics for SIMD (Single Instruction, Multiple Data) operations.
   - Different modules like `x86_64`, `arm`, `wasm32`, etc.

8. **std::os::raw:**
   - Contains basic types for FFI (Foreign Function Interface) operations.
   - Types like `c_void`, `c_char`, `c_int`, etc.

9. **std::sys:**
   - Contains platform-specific system-level types and functions.
   - Often used in implementing low-level abstractions.

10. **std::panic:**
    - Provides the ability to catch and handle panics.
    - Functions like `catch_unwind`, `AssertUnwindSafe`, `UnwindSafe`, etc.

11. **std::slice:**
    - Provides functions to work with slices and raw slices.
    - Functions like `from_raw_parts`, `from_raw_parts_mut`, etc.

12. **std::arch::core:**
    - Architecture-specific intrinsics for core operations like arithmetic, comparisons, etc.

These libraries allow for fine-grained control over memory, synchronization, and other low-level operations. However, using them requires a deep understanding of the Rust language and the associated safety guarantees, as incorrect usage can lead to memory unsafety, data races, and other issues. Always exercise caution and thoroughly test your code when working with Unsafe Rust and these built-in libraries.