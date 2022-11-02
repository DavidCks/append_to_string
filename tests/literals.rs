use append_to_string::*;

#[test]
fn test_macro(){
    let a = append_to_string!("strref");
    assert_eq!(a, "strref".to_owned());
}
    
    