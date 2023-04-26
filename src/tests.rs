#![cfg(test)]

use crate::{config, JObject};

#[test]
fn test_main(){
    
    let mut conf = config::open_from_home(".andrea", "config.json").unwrap();

    let _s0 = conf.get_section_mut("alberi").unwrap();

    conf.copy_from(JObject::new());

    conf.save().unwrap();
    
}