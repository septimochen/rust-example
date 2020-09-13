use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

#[test]
pub fn cell_test1() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 1);

    }
    let total: i32 = shared_map.borrow().values().sum();
    println!("{:?}", total);
    assert_eq!(total, 1)
}