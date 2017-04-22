use MutStatic;
use std::mem;
use std::ops::DerefMut;

struct TestStruct { value: usize }

impl TestStruct {
    pub fn new(value: usize) -> Self {
        TestStruct{ value: value }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value
    }
}

#[test]
fn test() {
    let test_obj = MutStatic::new();
    assert!(test_obj.is_set().unwrap() == false);

    test_obj.set(TestStruct::new(0)).unwrap();
    assert!(test_obj.is_set().unwrap() == true);

    {
        let test_obj = test_obj.read().unwrap();
        assert!(test_obj.get_value() == 0);
    }

    {
        let mut test_obj = test_obj.write().unwrap();
        test_obj.set_value(1);
        assert!(test_obj.get_value() == 1);
    }

    {
        let mut test_obj = test_obj.write().unwrap();
        mem::replace(test_obj.deref_mut(), TestStruct::new(2));
        assert!(test_obj.get_value() == 2);
    }
}
