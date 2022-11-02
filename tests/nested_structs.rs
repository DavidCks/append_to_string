use append_to_string::*;

#[derive(Debug, PartialEq, Clone)]
struct TestStruct {
    a: String,
    c: String,
    d: TestStruct2,
    k: String,
} 

#[derive(Debug, PartialEq, Clone)]
struct TestStruct2 {
    g: String,
    h: String,
    l: TestStruct3,
    u: String,
} 

#[derive(Debug, PartialEq, Clone)]
struct TestStruct3 {
    o: String,
    i: String,
} 

#[test]
fn test_macro(){
    let test_struct = append_to_string! (
        TestStruct {
            a: 1,
            c: "d",
            d: TestStruct2 {
                g: String::from("lll"),
                h: "ooo".to_owned(),
                l: TestStruct3 {
                    o: 42,
                    i: "lololo",
                },
                u: "lllllllooooo"
            },
            k: "oooookkkkkkk"
        }
    );

    assert_eq!(test_struct, TestStruct {
        a: 1.to_string(),
        c: "d".to_string(),
        d: TestStruct2 {
            g: "lll".to_string(),
            h: "ooo".to_string(),
            l: TestStruct3 {
                o: 42.to_string(),
                i: "lololo".to_string(),
            },
            u: "lllllllooooo".to_string(),
        },
        k: "oooookkkkkkk".to_string(),
    });
}
    
    