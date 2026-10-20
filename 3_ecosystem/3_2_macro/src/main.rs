use std::collections::BTreeMap;

macro_rules! btree {
    ( $( $key:expr => $val:expr ),*$(,)? ) => {
        {
            let mut temp_btree = BTreeMap::new();
            $(
                temp_btree.insert($key, $val);
            )*
            temp_btree
        }
    };
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_btree() {
        let map: BTreeMap<i32, i32> = btree!();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_single_element() {
        let map = btree! { "key" => 42 };
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_multiple_elements() {
        let map = btree! {
            1 => "one",
            2 => "two",
            3 => "three",
        };
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }
}
