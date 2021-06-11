// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ptr;
use std::sync::atomic;

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! hashset {
    ($( $key: expr ),*) => {{
         let mut map = ::std::collections::HashSet::new();
         $( map.insert($key); )*
         map
    }}
}

#[inline]
pub fn zero<T>(input: &mut [T]) {
    atomic::fence(atomic::Ordering::SeqCst);
    unsafe {
        ptr::write_bytes(input.as_mut_ptr(), 0, input.len());
    }
}
