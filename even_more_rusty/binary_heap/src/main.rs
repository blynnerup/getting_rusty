use std::collections::BinaryHeap;

fn main() {
    // Create a new binary heap
    let mut bheap = BinaryHeap::new(); 
    
    // Add elements to it and check how it is ordered
    bheap.push(2);
    bheap.push(9);
    bheap.push(0);
    bheap.push(22);
    println!("{:?}", bheap); // Prints [22, 9, 0, 2]
    bheap.push(13);
    println!("{:?}", bheap); // Prints [22, 13, 0, 2, 9]
    bheap.push(16);
    println!("{:?}", bheap); // Prints [22, 13, 16, 2, 9, 0]

    bheap.pop(); // This will pop 22, and reorder the heap so that 16 is in front
    println!("{:?}", bheap); // Prints [16, 13, 0, 2, 9]
    
    // Peek allows to get a see a value from the font of the heap without removing it.
    println!("{:?}", bheap.peek()); // Prints Some(16). So peek returns an Option<T>
}
