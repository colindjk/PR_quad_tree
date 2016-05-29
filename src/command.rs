
/// Command enum, takes 
pub enum Command<K, V> {
    Insert { key: K, value: V },
    RemoveKey { value: V },
    RemoveVal { key: K },
    Search { key: K },
    Match { key: K }
}
