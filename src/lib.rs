pub mod data_structures;

#[cfg(test)]
mod tests {
    use crate::data_structures::Heap;

    #[test]
    fn it_works() {
        let mut heap: Heap<u32> = Heap::new_max();
        heap.add(14);
        heap.add(12);
        
        let next = heap.next();
        assert_eq!(Some(14), next);
    }
}
