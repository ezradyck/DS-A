/// heapsort is an in place sort that is done by splitting the source array into two parts: the
/// heap section and the sorted section.
/// While sorting the array the heap section will get smaller as elements are sorted, and the
/// sorted portion will grow.
/// After each element is removed from the heap section and added to the array section new root
/// will be heapified in order to get the next max element to the top of the heap
pub fn heap_sort<T: Ord>(source: &mut [T]) {
    // check if it's already a sorted array
    if source.len() <= 1 {
        return;
    }
    // Step 1. convert the source to a max-heap
    for idx in (0..=parent_idx(source.len() - 1)).rev() {
        heapify_down(source, idx, source.len());
    }

    // Loop over all items in the array starting at the top and decrementing our available space
    for end in (1..source.len()).rev() {
        // Step 2. swap the first element in the heap with the next available space in the sorted
        // array portion
        source.swap(0, end);
        if end > 0 {
            // Step 4. heapify the heap space
            heapify_down(source, 0, end - 1);
        }
    }
}

fn heapify_down<T: Ord>(source: &mut [T], start: usize, end: usize) {
    let mut root = start;
    while left_child_idx(root) <= end {
        let child_idx = smallest_child_idx(source, end, root);
        if source[child_idx] > source[root] {
            source.swap(child_idx, root);
        }
        root = child_idx;
    }
}

fn parent_idx(idx: usize) -> usize {
    (idx - 1) / 2
}

fn left_child_idx(idx: usize) -> usize {
    (idx * 2) + 1
}

fn right_child_idx(idx: usize) -> usize {
    left_child_idx(idx) + 1
}

fn smallest_child_idx<T: Ord>(source: &[T], end: usize, idx: usize) -> usize {
    if right_child_idx(idx) > end {
        left_child_idx(idx)
    } else {
        let ldx = left_child_idx(idx);
        let rdx = right_child_idx(idx);
        if source[ldx] > source[rdx] {
            ldx
        } else {
            rdx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsorted_array() {
        let mut source = [5, 1, 2, 1, 1, 69, 420];
        heap_sort(&mut source);
        assert_eq!([1, 1, 1, 2, 5, 69, 420], source);
    }

    #[test]
    fn no_elements() {
        let mut arr:  Vec<i32> = Vec::new();
        heap_sort(&mut arr);
        assert_eq!(&arr, &[]);
    }
}
