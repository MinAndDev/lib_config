#![cfg(test)]

use serde_json::json;

use crate::config;

#[test]
fn test_main(){
    
    let mut conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

    conf.write_value("val0", 100).unwrap();
    let val0: i32 = conf.read_value("val0").unwrap();

    assert_eq!(val0, 100);

    conf.save().unwrap();
    
}

#[test]
fn test_sections(){

    {
        let mut conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

        conf.write_value("sect0", json!({
            "val0" : 10,
            "val1" : "foo"
        })).unwrap();

        conf.save().unwrap();
    }

    {
        let conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

        let sect0 = conf.get_section("sect0").unwrap();
        let val0 : i32 = sect0.read_value("val0").unwrap();
        let val1: String = sect0.read_value("val1").unwrap();

        assert_eq!(val0, 10);
        assert_eq!(val1, String::from("foo"));
    }

}