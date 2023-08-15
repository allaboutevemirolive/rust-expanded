use std::rc::Rc;
use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, BTreeMap};

trait MyTrait {
    fn do_something(&self);
}

impl<T: MyTrait> MyTrait for &T {
    fn do_something(&self) {
        (**self).do_something();
        // We also can use fully qualify syntax:
        // T::do_something(*self);
    }
}


impl<T: MyTrait> MyTrait for &mut T {
    fn do_something(&self) {
        (**self).do_something();
    }
}

impl<T: MyTrait> MyTrait for Box<T> {
    fn do_something(&self) {
        (**self).do_something();
    }
}

impl<T: MyTrait + Clone> MyTrait for Rc<T> {
    fn do_something(&self) {
        (**self).do_something();
    }
}

impl<T: MyTrait + Send + Sync> MyTrait for Arc<T> {
    fn do_something(&self) {
        (**self).do_something();
    }
}

impl<T: MyTrait + Copy> MyTrait for Cell<T> {
    fn do_something(&self) {
        self.get().do_something();
    }
}

impl<T: MyTrait> MyTrait for RefCell<T> {
    fn do_something(&self) {
        self.borrow().do_something();
    }
}

impl<T: MyTrait> MyTrait for Vec<T> {
    fn do_something(&self) {
        for item in self {
            item.do_something();
        }
    }
}

impl<K: MyTrait, V: MyTrait> MyTrait for HashMap<K, V> {
    fn do_something(&self) {
        for (key, value) in self {
            println!("Key:");
            key.do_something();
            println!("Value:");
            value.do_something();
        }
    }
}

impl<K: MyTrait, V: MyTrait> MyTrait for BTreeMap<K, V> {
    fn do_something(&self) {
        for (key, value) in self {
            println!("Key:");
            key.do_something();
            println!("Value:");
            value.do_something();
        }
    }
}

impl MyTrait for &str {
    fn do_something(&self) {
        println!("String slice: {}", self);
    }
}

// str` by itself isn't used directly in most code; 
// it's more common to work with `&str` references 
// or `String` owned strings.

// impl MyTrait for str {
//     fn do_something(&self) {
//         println!("String slice: {}", self);
//     }
// }

impl MyTrait for String {
    fn do_something(&self) {
        println!("Dynamically sized string: {}", self);
    }
}

impl<T: MyTrait, const N: usize> MyTrait for [T; N] {
    fn do_something(&self) {
        for item in self {
            item.do_something();
        }
    }
}

impl<T: MyTrait> MyTrait for Option<T> {
    fn do_something(&self) {
        match self {
            Some(item) => item.do_something(),
            None => println!("None"),
        }
    }
}

impl<T: MyTrait, E: MyTrait> MyTrait for Result<T, E> {
    fn do_something(&self) {
        match self {
            Ok(item) => item.do_something(),
            Err(error) => error.do_something(),
        }
    }
}

struct CustomStruct;

impl MyTrait for CustomStruct {
    fn do_something(&self) {
        println!("CustomStruct is doing something");
    }
}

enum CustomEnum {
    VariantA,
    VariantB,
}

impl MyTrait for CustomEnum {
    fn do_something(&self) {
        match self {
            CustomEnum::VariantA => println!("VariantA is doing something"),
            CustomEnum::VariantB => println!("VariantB is doing something"),
        }
    }
}

impl MyTrait for i32 {
    fn do_something(&self) {
        println!("i32: {}", self);
    }
}

fn main() {
    let mut value = 42;
    
    println!("==============================");
    
    println!("&T");
    let reference: &i32 = &value;
    reference.do_something();  // Calls the blanket implementation for &T
    
    println!("==============================");

    println!("&mut T");
    let mutable_reference: &mut i32 = &mut value;
    mutable_reference.do_something();  // Calls the blanket implementation for &mut T
    
    println!("==============================");

    println!("Box<T>");
    let boxed_value: Box<dyn MyTrait> = Box::new(value);
    boxed_value.do_something();  // Calls the blanket implementation for Box<T>

    println!("==============================");

    println!("Rc<T>");
    let rc_value: Rc<dyn MyTrait> = Rc::new(value);
    rc_value.do_something();  // Calls the blanket implementation for Rc<T>

    println!("==============================");

    println!("Arc<T>");
    let arc_value: Arc<dyn MyTrait> = Arc::new(value);
    arc_value.do_something();  // Calls the blanket implementation for Arc<T>

    println!("==============================");

    println!("Cell<T>");
    let cell_value: Cell<i32> = Cell::new(value);
    cell_value.do_something();  // Calls the blanket implementation for Cell<T>

    println!("==============================");

    println!("RefCell<T>");
    let ref_cell_value: RefCell<i32> = RefCell::new(value);
    ref_cell_value.do_something();  // Calls the blanket implementation for RefCell<T>

    println!("==============================");

    println!("Vec<T>");
    let vec_values: Vec<i32> = vec![value, value + 1, value + 2];
    vec_values.do_something();  // Calls the blanket implementation for Vec<T>

    println!("==============================");

    println!("HashMap<K, V>");
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    hash_map.insert(1, 10);
    hash_map.insert(2, 20);
    hash_map.do_something();  // Calls the blanket implementation for HashMap<K, V>

    println!("==============================");

    println!("BTreeMap<K, V>");
    let mut btree_map: BTreeMap<i32, i32> = BTreeMap::new();
    btree_map.insert(3, 30);
    btree_map.insert(4, 40);
    btree_map.do_something();  // Calls the blanket implementation for BTreeMap<K, V>

    println!("==============================");

    println!("&str");
    let string_slice: &str = "Hello";
    string_slice.do_something();  // Calls the trait implementation for &str

    println!("==============================");

    println!("str");
    let string_slice: &str = "Hello";
    string_slice.do_something();  // Calls the trait implementation for str

    println!("==============================");

    println!("String");
    let dynamic_string: String = String::from("Dynamic");
    dynamic_string.do_something();  // Calls the trait implementation for String

    println!("==============================");

    println!("[T; N]");
    let array: [i32; 3] = [1, 2, 3];
    array.do_something();  // Calls the blanket implementation for [T; N]

    println!("==============================");

    println!("Option<T>");
    let some_value: Option<i32> = Some(100);
    some_value.do_something();  // Calls the blanket implementation for Option<T>

    println!("==============================");

    println!("Result<T, E>");
    let result_value: Result<i32, &str> = Ok(200);
    result_value.do_something();  // Calls the blanket implementation for Result<T, E>

    println!("==============================");

    println!("CustomStruct");
    let custom_instance: CustomStruct = CustomStruct;
    custom_instance.do_something();  // Calls the trait implementation for CustomStruct

    println!("==============================");

    println!("CustomEnum");
    let custom_enum_variant_a: CustomEnum = CustomEnum::VariantA;
    custom_enum_variant_a.do_something();  // Calls the trait implementation for CustomEnum

    let custom_enum_variant_b: CustomEnum = CustomEnum::VariantB;
    custom_enum_variant_b.do_something();

    println!("==============================");
}