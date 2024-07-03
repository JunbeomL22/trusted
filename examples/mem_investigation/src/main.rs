use std::mem;
use std::alloc::{Layout, alloc};
use std::ptr;
use std::marker::PhantomData;
use quantlib::util::type_name;
use serde::{Serialize, Deserialize};    
use serde_json::json;

pub struct MiniStruct {
    pub a: bool,
}

#[repr(C)]
pub enum MyEnum {
    A(u8),
    B(u8),
    C(u8),
}

#[repr(C)]
pub enum MyEnum16_32_64 {
    A(u16),
    B(u32),
    C(u64),
}

#[repr(C)]
pub enum MyEnum16_64_32 {
    A(u16),
    B(u64),
    C(u32),
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
struct MyStruct64_8_16 {
    a: u64,
    b: u8,
    c: u16,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
struct MyStruct8_64_16 {
    a: u8,
    b: u64,
    c: u16,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct F64 {
    pub val: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct F64Pad {
    pub val: f64,
    _pad: u8,
}

#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct F32 {
    pub val: f32,
}

#[repr(C)]
pub struct F32Pad {
    pub val: f32,
    _pad: u8,
}

#[repr(C)]
pub struct MockStruct;

pub trait MockTrait {}
    
impl MockTrait for MockStruct {}

#[repr(C)]
pub struct F32Phantom<T: MockTrait> {
    pub val: f32,
    _phantom: PhantomData<T>,
}

impl<T: MockTrait> F32Phantom<T> {
    pub fn new(val: f32) -> Self {
        F32Phantom {
            val,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct F64Phantom<T: MockTrait> {
    pub val: f64,
    _phantom: PhantomData<T>,
}

impl<T: MockTrait> F64Phantom<T> {
    pub fn new(val: f64) -> Self {
        F64Phantom {
            val,
            _phantom: PhantomData,
        }
    }
}

fn main() {
    let mini_instance = MiniStruct { a: true };
    println!("MiniStruct");
    print_struct_info(mini_instance);

    let enum_instance = MyEnum::A(1);
    println!("MyEnum::A");
    print_struct_info(enum_instance);

    let enum_instance = MyEnum16_32_64::A(0xAAAA);
    println!("MyEnumABC::A");
    print_struct_info(enum_instance);

    let enum_instance = MyEnum16_64_32::B(0xBBBBBBBBBBBBBBBB);

    println!("MyEnumACB::B");
    print_struct_info(enum_instance);


    let instance = MyStruct8_64_16 { a: 0xAA, b: 0xAAAAAAAAAAAAAAAA, c: 0xCCCC };
    println!("MyStruct8_64_16");
    print_struct_info(instance);

    let f64_instance = F64 { val: 3.14 };
    //print_struct_info(f64_instance);

    let f64_pad_instance = F64Pad { val: 3.14, _pad: 0x00 };
    //print_struct_info(f64_pad_instance);

    let phantom_instance = F64Phantom::<MockStruct>::new(3.14);
    print_struct_info(phantom_instance);
    
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
    unsafe {
        let vtable = std::mem::transmute::<_, usize>(std::ptr::null::<T>());
        if vtable != 0 {
            println!("VTable pointer: 0x{:X}", vtable);
        } else {
            println!("No VTable (not a trait object)");
        }
    }
}

fn print_struct_info<T>(instance: T) {
    //println!("Type: {}", type_name::<T>(&instance));
    // Get the size and alignment of T
    let size = mem::size_of::<T>();
    let align = mem::align_of::<T>();
    
    println!("Size: {} bytes", size);
    println!("Alignment: {} bytes", align);
    
    // Allocate memory for T
    let layout = Layout::from_size_align(size, align).unwrap();
    let ptr = unsafe { alloc(layout) };
    
    // Copy the instance to our allocated memory
    unsafe { ptr::copy_nonoverlapping(&instance as *const T as *const u8, ptr, size); }
    
    // Print the memory contents
    //print_memory(ptr as *const u8, size);
    
    // Print the memory of the original instance
    print_memory(&instance as *const T as *const u8, size);
    
    // Print the pointer to the instance
    println!("\nPointer to instance: {:p}", &instance as *const T);
    
    // Print the vtable pointer (if any)
    print_vtable::<T>();
    
    // Clean up
    unsafe { std::alloc::dealloc(ptr, layout); }

    println!();
}