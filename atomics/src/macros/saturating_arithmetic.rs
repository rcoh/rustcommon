// Copyright 2019-2020 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

macro_rules! saturating_arithmetic {
    ($name:ident, $type:ty) => {
        impl SaturatingArithmetic for $name {
            #[inline]
            fn fetch_saturating_add(
                &self,
                value: <Self as Atomic>::Primitive,
                ordering: Ordering,
            ) -> <Self as Atomic>::Primitive {
                let load_ordering = match ordering {
                    Ordering::AcqRel => Ordering::Acquire,
                    Ordering::Release => Ordering::Relaxed,
                    _ => ordering,
                };
                let mut previous = self.load(load_ordering);
                if previous == <$type>::max_value() {
                    // already at numeric bound, return previous value.
                    return previous;
                } else {
                    loop {
                        let new = previous.saturating_add(value);
                        let result = self.compare_and_swap(previous, new, ordering);
                        if result == previous {
                            // value updated, return previous.
                            return previous;
                        }
                        previous = result;
                        if previous == <$type>::max_value() {
                            // value concurrently updated and now at numeric bound.
                            // return its new value as the previous value.
                            return previous;
                        }
                    }
                }
            }

            #[inline]
            fn fetch_saturating_sub(
                &self,
                value: <Self as Atomic>::Primitive,
                ordering: Ordering,
            ) -> <Self as Atomic>::Primitive {
                let load_ordering = match ordering {
                    Ordering::AcqRel => Ordering::Acquire,
                    Ordering::Release => Ordering::Relaxed,
                    _ => ordering,
                };
                let mut previous = self.load(load_ordering);
                if previous == <$type>::min_value() {
                    // already at numeric bound, return previous value.
                    return previous;
                } else {
                    loop {
                        let new = previous.saturating_sub(value);
                        let result = self.compare_and_swap(previous, new, ordering);
                        if result == previous {
                            // value updated, return previous.
                            return previous;
                        }
                        previous = result;
                        if previous == <$type>::min_value() {
                            // value concurrently updated, and now at numeric bound.
                            // return its new value as the previous value.
                            return previous;
                        }
                    }
                }
            }
        }
    };
}
