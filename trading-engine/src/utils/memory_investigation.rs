use std::alloc::{alloc, Layout};

fn type_name<T>(_: &T) -> &'static str {
    let full_name = std::any::type_name::<T>();
    let parts: Vec<&str> = full_name.split("::").collect();
    parts.last().unwrap_or(&full_name)
}

fn print_memory(ptr: *const u8, size: usize) {
    println!("\nMemory contents:");
    for i in 0..size {
        if i % 4 == 0 {
            print!("{:04X}: ", i);
        }
        unsafe {
            print!("{:02X} ", *ptr.add(i));
        }
        if (i + 1) % 4 == 0 || i == size - 1 {
            println!();
        }
    }
}

fn print_vtable<T>() {
    let vtable = std::ptr::null::<T>() as usize;
    if vtable != 0 {
        println!("VTable pointer: 0x{:X}", vtable);
    } else {
        println!("No VTable (not a trait object)");
    }

}

pub fn print_struct_info<T>(instance: T) {
    println!("Type: {}", type_name::<T>(&instance));
    // Get the size and alignment of T
    let size = std::mem::size_of::<T>();
    let align = std::mem::align_of::<T>();

    println!("Size: {} bytes", size);
    println!("Alignment: {} bytes", align);

    // Allocate memory for T
    let layout = Layout::from_size_align(size, align).unwrap();
    let ptr = unsafe { alloc(layout) };

    // Copy the instance to our allocated memory
    unsafe {
        std::ptr::copy_nonoverlapping(&instance as *const T as *const u8, ptr, size);
    }

    // Print the memory contents
    //print_memory(ptr as *const u8, size);

    // Print the memory of the original instance
    print_memory(&instance as *const T as *const u8, size);

    // Print the pointer to the instance
    println!("\nPointer to instance: {:p}", &instance as *const T);

    // Print the vtable pointer (if any)
    print_vtable::<T>();

    // Clean up
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }

    println!();
}
