use std::ptr;

struct Parent {
    name: String
}

struct Child{
    parent: *const Parent
}
pub fn unsafe_rust() {
    // let parent = Parent {
    //     name: String::from("Master")
    // };
    // let child  = Child {
    //     parent: &parent as *const Parent
    // };
    // drop(parent);
    // unsafe {
    //     println!("{:?}", (*child.parent).name);
    // }
    // let p: *mut i32 = ptr::null_mut();
    // // user must ensure the p is not null
    // if !p.is_null(){
    //     unsafe {
    //         println!("{}", *p);
    //     }
    // }    
    let mut s = [1,2,3];
    let ptr = s.as_mut_ptr();
    // let first_ele = unsafe {
    //     &mut *ptr
    // };
    // *first_ele = 5;


    unsafe{println!("{:?}", *ptr.add(1))}; 
    /*
    ptr = pointer address of s
      ptr.add(0) = 1
      ptr.add(1) = 2
      ptr.add(2) = 3
    */
    println!("{:?}", s);
    println!("{:?}", ptr);

    println!("{:?}", s);

}

