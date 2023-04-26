# `lib_config`
JSON Configuration Library in Rust

## Usage

#### Dependency

Copy into your `[dependencies]` section of your Cargo.toml file

```toml
lib_config = "0.1.0"
```

#### Example

Code taken from the tests

```rust
//example: open (or create) a config, then write and read a value
    {
        let mut conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

        conf.write_value("val0", 100).unwrap();
        let val0: i32 = conf.read_value("val0").unwrap();

        assert_eq!(val0, 100);

        conf.save().unwrap();
    }

//example: create & write a config section
    {
        let mut conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

        conf.write_value("sect0", json!({
            "val0" : 10,
            "val1" : "foo"
        })).unwrap();

        conf.save().unwrap();
    }

///example: get and read from config section
    {
        let conf = config::open_from_home(".lib_config", "conftest.json").unwrap();

        let sect0 = conf.get_section("sect0").unwrap();
        let val0 : i32 = sect0.read_value("val0").unwrap();
        let val1: String = sect0.read_value("val1").unwrap();

        assert_eq!(val0, 10);
        assert_eq!(val1, String::from("foo"));
    }
```

The resulting config file will be

```json
  {
    "sect0": {
      "val0": 10,
      "val1": "foo"
    },
    "val0": 100
  }
```

## Licence 

Licenced under

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
