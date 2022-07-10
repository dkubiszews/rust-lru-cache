// MIT License

// Copyright (c) 2022 Dawid Kubiszewski (dawidkubiszewski@gmail.com)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod dkubiszewski {
    use self::utils::ListNode;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::rc::Rc;

    /// LRU cache
    /// # Example
    /// ```
    /// use rust_lru_cache::dkubiszewski::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, 15);
    /// cache.put(2, 50);
    ///
    /// println!("{}", cache.get(&1).unwrap());
    /// println!("{}", cache.get(&2).unwrap());
    /// ```
    pub struct LruCache<KeyType, ValueType>
    where
        KeyType: Eq + Hash,
        ValueType: Eq + Hash,
    {
        capacity: usize,
        map: HashMap<KeyType, (ValueType, Rc<utils::ListNode<KeyType>>)>,
        queue: utils::List<KeyType>,
    }

    impl<KeyType, ValueType> LruCache<KeyType, ValueType>
    where
        KeyType: Eq + Hash + Copy,
        ValueType: Eq + Hash,
    {
        /// Creates LRU cache with specific capacity.
        ///
        /// # Arguments
        ///
        /// * `capacity` Capacity of the cache.
        pub fn new(capacity: usize) -> Self {
            LruCache {
                capacity: capacity,
                map: HashMap::new(),
                queue: utils::List::new(),
            }
        }

        /// Put data in the cache.
        ///
        /// # Arguments
        ///
        /// * `key` The key.
        /// * `value` The value.
        pub fn put(&mut self, key: KeyType, value: ValueType) {
            if let Some((_map_value, node)) = self.map.get_mut(&key) {
                self.queue.remove_node(node.clone());
                self.map.remove(&key);
            } else if self.map.len() == self.capacity {
                self.map.remove(
                    if let ListNode::Link {
                        value,
                        prev: _,
                        next: _,
                    } = self.queue.back().as_ref()
                    {
                        value
                    } else {
                        panic!("Logic error");
                    },
                );
                self.queue.pop_back();
            }
            let front_node = self.queue.push_front(key);
            self.map.insert(key, (value, front_node));
        }

        /// Get data from the cache.
        ///
        /// # Arguments
        ///
        /// * `key` The key.
        pub fn get(&mut self, key: &KeyType) -> Option<&ValueType> {
            match self.map.get(key) {
                Some((value, node)) => {
                    self.queue.remove_node(node.clone());
                    self.queue.push_node_front(node.clone());
                    Some(&value)
                }
                None => None,
            }
        }
    }
    mod utils {
        use std::{cell::RefCell, rc::Rc};

        #[derive(Debug, PartialEq)]
        pub enum ListNode<T> {
            None,
            Link {
                value: T,
                prev: RefCell<Rc<ListNode<T>>>,
                next: RefCell<Rc<ListNode<T>>>,
            },
        }

        pub struct List<T> {
            head: RefCell<Rc<ListNode<T>>>,
            tail: RefCell<Rc<ListNode<T>>>,
        }

        impl<T> List<T>
        where
            T: Copy,
        {
            pub fn new() -> Self {
                Self {
                    head: RefCell::new(Rc::new(ListNode::None)),
                    tail: RefCell::new(Rc::new(ListNode::None)),
                }
            }

            pub fn push_front(&mut self, value: T) -> Rc<ListNode<T>> {
                let new_node = Rc::new(ListNode::Link {
                    value: value,
                    prev: RefCell::new(Rc::new(ListNode::None)),
                    next: RefCell::new(self.head.borrow().clone()),
                });

                if let ListNode::Link {
                    value: _,
                    prev,
                    next: _,
                } = self.head.get_mut().as_ref()
                {
                    prev.replace(new_node.clone());
                }
                self.head.replace(new_node.clone());

                if let ListNode::None = self.tail.get_mut().as_ref() {
                    self.tail.replace(new_node.clone());
                }

                new_node
            }

            pub fn push_node_front(&mut self, node: Rc<ListNode<T>>) {
                if let ListNode::Link {
                    value: _,
                    prev,
                    next,
                } = node.as_ref()
                {
                    prev.replace(Rc::new(ListNode::None));
                    next.replace(self.head.borrow().clone());
                }

                if let ListNode::Link {
                    value: _,
                    prev,
                    next: _,
                } = self.head.get_mut().as_ref()
                {
                    prev.replace(node.clone());
                }
                self.head.replace(node.clone());

                if let ListNode::None = self.tail.get_mut().as_ref() {
                    self.tail.replace(node.clone());
                }
            }

            pub fn remove_node(&mut self, node: Rc<ListNode<T>>) {
                if let ListNode::Link {
                    value: _,
                    prev,
                    next,
                } = node.as_ref()
                {
                    let new_next = next;
                    if let ListNode::Link {
                        value: _,
                        prev: _,
                        next,
                    } = prev.borrow_mut().as_ref()
                    {
                        next.replace(new_next.borrow().clone());
                    } else {
                        self.head.replace(new_next.borrow().clone());
                    }

                    let new_prev = prev;
                    if let ListNode::Link {
                        value: _,
                        prev,
                        next: _,
                    } = next.borrow_mut().as_ref()
                    {
                        prev.replace(new_prev.borrow().clone());
                    } else {
                        self.tail.replace(new_prev.borrow().clone());
                    }
                }
            }

            pub fn back(&self) -> Rc<ListNode<T>> {
                self.tail.borrow().clone()
            }

            pub fn pop_back(&mut self) {
                let new_tail: RefCell<Rc<ListNode<T>>> = RefCell::new(Rc::new(ListNode::None));

                if let ListNode::Link {
                    value: _,
                    prev,
                    next: _,
                } = self.tail.get_mut().as_ref()
                {
                    new_tail.replace(prev.borrow().clone());

                    if let ListNode::Link {
                        value: _,
                        prev: _,
                        next,
                    } = prev.borrow_mut().as_ref()
                    {
                        next.replace(Rc::new(ListNode::None));
                    }
                }

                if let ListNode::None = new_tail.borrow().as_ref() {
                    self.head.replace(new_tail.borrow().clone());
                }

                self.tail.replace(new_tail.borrow().clone());
            }
        }

        #[cfg(test)]
        mod tests {
            use crate::dkubiszewski::utils::{List, ListNode};

            #[test]
            fn empty_list() {
                let ll: List<i32> = List::new();
                assert_eq!(&ListNode::None, ll.back().as_ref());
            }

            #[test]
            fn add_remove_elements() {
                let mut ll: List<i32> = List::new();

                ll.push_front(1);
                assert_eq!(
                    &1,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );
                ll.push_front(2);
                assert_eq!(
                    &1,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(
                    &2,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(&ListNode::None, ll.back().as_ref());
            }

            #[test]
            fn add_remove_elements_and_add() {
                let mut ll: List<i32> = List::new();

                ll.push_front(1);
                assert_eq!(
                    &1,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );
                ll.push_front(2);
                assert_eq!(
                    &1,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(
                    &2,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(&ListNode::None, ll.back().as_ref());

                ll.push_front(15);
                assert_eq!(
                    &15,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );
            }

            #[test]
            fn remove_middle_node() {
                let mut ll: List<i32> = List::new();

                ll.push_front(1);
                let middle_node = ll.push_front(2);
                ll.push_front(3);

                ll.remove_node(middle_node);

                assert_eq!(
                    &1,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(
                    &3,
                    match ll.back().as_ref() {
                        ListNode::None => panic!("Value should be set"),
                        ListNode::Link {
                            value,
                            prev: _,
                            next: _,
                        } => value,
                    }
                );

                ll.pop_back();
                assert_eq!(&ListNode::None, ll.back().as_ref());
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::dkubiszewski::LruCache;

        #[test]
        fn put_get() {
            let mut lru: LruCache<i32, i32> = LruCache::new(1);

            lru.put(1, 5);

            assert_eq!(5, *lru.get(&1).unwrap());
        }

        #[test]
        fn get_non_existing_key() {
            let mut lru: LruCache<i32, i32> = LruCache::new(1);

            assert_eq!(None, lru.get(&1));
        }

        #[test]
        fn drop_oldest_put_item_when_hit_capacity() {
            let mut lru: LruCache<i32, i32> = LruCache::new(2);

            lru.put(1, 5);
            lru.put(2, 6);
            lru.put(3, 7);

            assert_eq!(None, lru.get(&1));
            assert_eq!(6, *lru.get(&2).unwrap());
            assert_eq!(7, *lru.get(&3).unwrap());
        }

        #[test]
        fn drop_oldest_accessed_item_when_hit_capacity() {
            let mut lru: LruCache<i32, i32> = LruCache::new(2);

            lru.put(1, 5);
            lru.put(2, 6);

            assert_eq!(5, *lru.get(&1).unwrap());

            lru.put(3, 7);

            assert_eq!(5, *lru.get(&1).unwrap());
            assert_eq!(None, lru.get(&2));
            assert_eq!(7, *lru.get(&3).unwrap());
        }

        #[test]
        fn update_exisitng_key() {
            let mut lru: LruCache<i32, i32> = LruCache::new(2);

            lru.put(1, 5);
            lru.put(2, 6);

            lru.put(1, 11);

            assert_eq!(11, *lru.get(&1).unwrap());
            assert_eq!(6, *lru.get(&2).unwrap());
        }
    }
}
