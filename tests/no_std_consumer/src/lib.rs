#![no_std]

use collections::{bmap, bset, list, map, s, set};

pub fn invoke_all_macros() {
    let _: collections::alloc::string::String = s!("text");
    let _: collections::alloc::collections::BTreeMap<u8, u8> = bmap! {};
    let _: collections::alloc::collections::BTreeMap<u8, u8> = bmap! { 1 => 2 };
    let _: collections::alloc::collections::BTreeMap<
        collections::alloc::string::String,
        collections::alloc::string::String,
    > = bmap! { owned: "key" => "value" };
    let _: collections::alloc::collections::BTreeSet<u8> = bset![];
    let _: collections::alloc::collections::BTreeSet<u8> = bset![1, 2];
    let _: collections::alloc::collections::BTreeSet<collections::alloc::string::String> =
        bset! { owned: "one", "two" };
    let _: collections::alloc::collections::LinkedList<u8> = list! {};
    let _: collections::alloc::collections::LinkedList<u8> = list! { 1 => 2 };
    let _: collections::alloc::collections::LinkedList<collections::alloc::string::String> =
        list! { owned: "one" => "two" };
    let _: collections::std::collections::HashMap<u8, u8> = map! {};
    let _: collections::std::collections::HashMap<u8, u8> = map! { 1 => 2 };
    let _: collections::std::collections::HashMap<
        collections::alloc::string::String,
        collections::alloc::string::String,
    > = map! { owned: "key" => "value" };
    let _: collections::std::collections::HashSet<u8> = set![];
    let _: collections::std::collections::HashSet<u8> = set![1, 2];
    let _: collections::std::collections::HashSet<collections::alloc::string::String> =
        set! { owned: "one", "two" };
}
