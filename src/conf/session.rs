use std::collections::HashMap;

static mut SESSION: Option<HashMap<String, String>> = None;

pub fn get(key: &str) -> Option<&String> {
    def();
    unsafe {
        let map = SESSION.as_ref().unwrap();
        map.get(key)
    }
}


pub fn set(k: String, v: String) {
    def();
    unsafe
        {
        let map =  SESSION.as_mut().unwrap();
        map.insert(k, v);
    }
}


fn def() {
    unsafe{
        if SESSION.is_none() {
            SESSION = Some(HashMap::new())
        }
    }
}