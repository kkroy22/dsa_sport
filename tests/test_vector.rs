use dsa_sport::datastruct::vec_struct::Vector;


#[test]
fn vector_all() {
    let mut v: Vector<isize> = Vector::new();
    
    v.push_back(1);
    assert_eq!(v.len(), 1);

}
