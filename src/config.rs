use std::{path::{Path, PathBuf}, fs::{OpenOptions, File}, io::{Read, Write, Seek}, borrow::{Borrow, BorrowMut}, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::map::Entry;

use crate::{JObject, AnyError};

///Opens or create the given JSON config file within the given folder path
///# Arguments
///* `config_folder_path` - Path to the config folder, will create any missing folders
///* `file_name` - Name of the config file, will create the file if it doesn't exist
pub fn open_from_path<P: AsRef<Path>>(config_folder_path: P, file_name: &str) -> Result<Config, AnyError>{

    let mut path = PathBuf::new();
    path.push(config_folder_path);
    std::fs::create_dir_all(&path)?;
    path.push(file_name);

    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)?;

    let mut str_json = String::new();
    file.read_to_string(&mut str_json)?;

    let obj_config = if str_json.is_empty() {
        JObject::new()
    }
    else {
        serde_json::from_str::<JObject>(&str_json)?
    };

    Ok(Config { file, data: obj_config })
}

///Opens or create the given JSON config file within the given folder path appended to the result of `directories::BaseDirs::new().home_dir()`
///# Arguments
///* `folder_path` - Path to the config folder, will create any missing folders
///* `file_name` - Name of the config file, will create the file if it doesn't exist
pub fn open_from_home<P: AsRef<Path>>(folder_path: P, file_name: &str) -> Result<Config, AnyError>{
    let dirs = directories::BaseDirs::new().ok_or("No valid home directory path could be retrived from OS")?;
    let home = dirs.home_dir();
    let mut buff = PathBuf::from(home);
    buff.push(folder_path);
    open_from_path(buff, file_name)
}

///Object representing an open config file, use `lib_config::open_from_path` or `lib_config::open_from_home` to get an instance
///# Usage
/// A `Config` may contain primitive values (such as strings or numbers), arrays or `Section`s. A `Section` is a JSON object that can contain the same values as a Config,
/// useful to logically split the config file. To save the contents of the config call the `save()` associated function.
#[derive(Debug)]
pub struct Config{
    file: File,
    data: JObject,
}

impl Config {

    ///Writes a valute to the given key, if it doesn't exist, inserts the key - value pair
    pub fn write_value<K: Into<String>, V: Serialize>(&mut self, key: K, value: V) -> Result<(), AnyError>{
        let key = key.into();
        let jvalue = serde_json::to_value(value)?;

        if let Entry::Vacant(e) = self.data.entry(&key) {
            e.insert(jvalue);
        }
        else {
            self.data[&key] = jvalue;
        }

        Ok(())
    }

    ///Reads a value from the given key, if the key does not exist returns `Err`
    pub fn read_value<K: Into<String>, V: DeserializeOwned>(&self, key: K) -> Result<V, AnyError>{
        let json = self.data.get(&key.into()).ok_or("Key not found")?.clone();
        let value = serde_json::from_value::<V>(json)?;

        Ok(value)
    }

    ///Reads a value from the given key, if the key does not exists, inserts it with the given value
    pub fn read_or_insert<K: Into<String>, V: DeserializeOwned + Serialize + Clone>(&mut self, key: K, value: V) -> Result<V, AnyError>{
        let key = key.into();

        let v = if let Entry::Vacant(e) = self.data.entry(&key) {
            let jvalue = serde_json::to_value(value.clone())?;
            e.insert(jvalue);
            value
        }
        else {
            let jvalue = &self.data[&key];
            serde_json::from_value(jvalue.clone())?
        };

        Ok(v)
    }

    ///Updates a value with the given key using the provided function, returns the final value of the key, if the key does not exist returns Err
    pub fn update_value<K, V, Out, F>(&mut self, key: &K, f_upd: F) -> Result<Out, AnyError>
    where
        K: ?Sized + Ord + Eq + Hash,
        String: Borrow<K>,
        V: DeserializeOwned,
        Out: Serialize,
        F: FnOnce(&V) -> Out,
    {
        let input = self.data.get(key).ok_or("Key not found")?;
        let value = serde_json::from_value(input.clone())?;
        let out = f_upd(&value);
        let jvalue = serde_json::to_value(&out)?;
        self.data[key] = jvalue;

        Ok(out)
    }

    ///Gets an immutable reference to `Section` at the given key
    pub fn get_section<K>(&self, key: &K) -> Result<Section<&JObject>, AnyError>
    where K: ?Sized + Ord + Eq + Hash, String: Borrow<K>{
        let value = self.data.get(key).ok_or("Key not found")?
        .as_object().ok_or("Key's Value is not a json object")?;

        Ok(Section(value))
    }

    ///Gets a mutable reference to `Section` at the given key
    ///# Remarks
    /// Changing the `Section`'s value will also change the `Config` data
    pub fn get_section_mut<K>(&mut self, key: &K) -> Result<Section<&mut JObject>, AnyError>
    where K: ?Sized + Ord + Eq + Hash, String: Borrow<K>{
        let value = self.data.get_mut(key).ok_or("Key not found")?
        .as_object_mut().ok_or("Key's Value is not a json object")?;

        Ok(Section(value))
    }

    ///Writes the `Config` object to the file
    pub fn save(&mut self) -> Result<String, AnyError>{
        let str = serde_json::to_string_pretty(&self.data)?;

        self.file.set_len(0)?;
        self.file.rewind()?;
        self.file.write_all(str.as_bytes())?;

        Ok(str)
    }

    ///Clones the `Config` data, the result does not have any reference to the original `Config`
    #[must_use]
    pub fn clone_data(&self) -> JObject{
        self.data.clone()
    }

    ///Replaces `Config` data with the provided data
    pub fn copy_from(&mut self, data: JObject){
        self.data = data;
    }

}

///Part of a `Config` object, may contain sub-sections
#[derive(Debug)]
pub struct Section<T: ?Sized + Borrow<JObject>>(T);

impl<T: ?Sized + Borrow<JObject>> Section<T>{

    ///Reads a value from the given key, if the key does not exist returns `Err`
    pub fn read_value<K, V>(&self, key: &K) -> Result<V, AnyError>
    where
        K: ?Sized + Ord + Eq + Hash,
        String: Borrow<K>,
        V: DeserializeOwned
    {
        let json = self.0.borrow().get(key).ok_or("Key not found")?.clone();
        let value = serde_json::from_value::<V>(json)?;

        Ok(value)
    }

    ///Gets an immutable reference to `Section` at the given key
    pub fn get_section<K>(&self, key: &K) -> Result<Section<&JObject>, AnyError>
    where K: ?Sized + Ord + Eq + Hash, String: Borrow<K>{
        let value = self.0.borrow().get(key).ok_or("Key not found")?
        .as_object().ok_or("Key's Value is not a json object")?;

        Ok(Section(value))
    }

    ///Clones the `Section` data, the result does not have any reference to the original `Config` nor `Section`
    #[must_use]
    pub fn clone_data(&self) -> JObject{
        self.0.borrow().clone()
    }
    
}

impl<T: ?Sized + BorrowMut<JObject>> Section<T>{

    ///Writes a valute to the given key, if it doesn't exist, inserts the key - value pair
    pub fn write_value<K: Into<String>, V: Serialize>(&mut self, key: K, value: V) -> Result<(), AnyError>{
        let key = key.into();
        let jvalue = serde_json::to_value(value)?;

        if let Entry::Vacant(e) = self.0.borrow_mut().entry(&key) {
            e.insert(jvalue);
        }
        else {
            self.0.borrow_mut()[&key] = jvalue;
        }

        Ok(())
    }

    ///Reads a value from the given key, if the key does not exists, inserts it with the given value
    pub fn read_or_insert<K: Into<String>, V: DeserializeOwned + Serialize + Clone>(&mut self, key: K, value: V) -> Result<V, AnyError>{
        let key = key.into();

        let v = if let Entry::Vacant(e) = self.0.borrow_mut().entry(&key) {
            let jvalue = serde_json::to_value(value.clone())?;
            e.insert(jvalue);
            value
        }
        else {
            let jvalue = &self.0.borrow()[&key];
            serde_json::from_value(jvalue.clone())?
        };

        Ok(v)
    }

    ///Updates a value with the given key using the provided function, returns the final value of the key, if the key does not exist returns Err
    pub fn update_value<K, V, Out, F>(&mut self, key: &K, f_upd: F) -> Result<Out, AnyError>
    where
        K: ?Sized + Ord + Eq + Hash,
        String: Borrow<K>,
        V: DeserializeOwned,
        Out: Serialize,
        F: FnOnce(&V) -> Out,
    {
        let input = self.0.borrow().get(key).ok_or("Key not found")?;
        let value = serde_json::from_value(input.clone())?;
        let out = f_upd(&value);
        let jvalue = serde_json::to_value(&out)?;
        self.0.borrow_mut()[key] = jvalue;

        Ok(out)
    }

    ///Gets a mutable reference to `Section` at the given key
    ///# Remarks
    /// Changing the `Section`'s value will also change the `Config` data
    pub fn get_section_mut<K>(&mut self, key: &K) -> Result<Section<&mut JObject>, AnyError>
    where K: ?Sized + Ord + Eq + Hash, String: Borrow<K>{
        let value = self.0.borrow_mut().get_mut(key).ok_or("Key not found")?
        .as_object_mut().ok_or("Key's Value is not a json object")?;

        Ok(Section(value))
    }

    ///Replaces `Section` data with the provided data
    pub fn copy_from(&mut self, data: JObject){
        self.0.borrow_mut().clear();

        for (k, v) in data {
            self.0.borrow_mut().insert(k, v);
        }
    }

}
