#![cfg(test)]

use crate::config;

#[test]
fn test_main(){
    
    let mut conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

    conf.write_value("val0", 100).unwrap();
    let val0: i32 = conf.read_value("val0").unwrap();

    assert_eq!(val0, 100);

    conf.save().unwrap();
    
}