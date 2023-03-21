pub struct Heap<T: Default> {
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            // add 1 item to offset counts when calculating parent indices
            items: vec![Default::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, item: T) {
        // start by placing item at the next available spot in the array (i.e., the bottom of the
        // heap)
        self.count += 1;
        self.items.push(item);

        // next heapify the element up the array until it reaches the desired spot
        self.heapify_up();
    }

    fn heapify_up(&mut self) {
        let mut idx = self.count;
        while self.parent_idx(idx) > 0 {
            let pdx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
            } else {
                // if the current index should not be swapped with th parent then there is no
                // reason to keep traversing up the heap
                break;
            }
            idx = pdx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_are_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                ldx
            } else {
                rdx
            }
        }
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        // remove the root node and replace it with the last node
        let next = Some(self.items.swap_remove(1));
        self.count -= 1;

        // heapify down
        if self.count > 0 {
            let mut idx = 1;
            while self.children_are_present(idx) {
                let child_idx = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                    self.items.swap(child_idx, idx);
                } else {
                    // if the value at the current index should not be replaced with the child
                    // index then we are finished heapifying, and the heap should satisfy the heap
                    // property
                    break;
                }
                idx = child_idx;
            }
        }

        next
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_max() -> Self {
        Self::new(|x, y| x > y)
    }

    pub fn new_min() -> Self {
        Self::new(|x, y| x < y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_items_from_heap_heapifies() {
        let mut heap: Heap<u32> = Heap::new_max();
        heap.add(14);
        heap.add(12);
        heap.add(13);
        heap.add(9);
        heap.add(9);
        heap.add(13);
        heap.add(12);

        println!("Current heap: {:?}", heap.items);

        let next = heap.next();
        // this will replace the root (14) with the last node (12)
        // the root (12) should be replaced with the greatest child (13)
        // then should be replaced with the greatest child (13)
        // Resulting in array rep. 0, 13, 12, 13, 9, 9, 12
        assert_eq!(Some(14), next);

        let expected_vals: Vec<u32> = vec![0, 13, 12, 13, 9, 9, 12];
        println!("Expected heap: {:?}", expected_vals);

        for (idx, expected) in expected_vals.iter().enumerate() {
            assert_eq!(expected, &heap.items[idx]);
        }
    }
}
