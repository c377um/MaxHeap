use rand::Rng;
use std::time::Instant;

struct MaxHeap {
    baum: Vec<i32>,
    swap_count: usize,
}

impl MaxHeap {
    fn new() -> Self {
        MaxHeap { baum: Vec::new(), swap_count: 0 }
    }

    fn fill_with_random_numbers(&mut self, n: usize) {
        self.baum.clear();
        let mut rng = rand::rng();
        for _ in 0..n {
            self.baum.push(rng.random_range(0..100));
        }
        self.swap_count = 0;
    }

    fn get_lchild(i: usize) -> usize { 2 * i + 1 }
    fn get_rchild(i: usize) -> usize { 2 * i + 2 }


    fn swap(&mut self, i: usize, j: usize) {
        self.baum.swap(i, j);
        self.swap_count += 1;
    }

    fn heapify(&mut self, mut i: usize, r: usize) {
        loop {
            let left  = MaxHeap::get_lchild(i);
            let right = MaxHeap::get_rchild(i);
            let mut largest = i;

            if left < r && self.baum[left]  > self.baum[largest] {
                largest = left;
            }
            if right < r && self.baum[right] > self.baum[largest] {
                largest = right;
            }

            if largest != i {
                self.swap(i, largest);
                i = largest;
            } else {
                break;
            }
        }
    }

    fn build_heap(&mut self) {
        let n = self.baum.len();
        for i in (0..=(n/2).saturating_sub(1)).rev() {
            self.heapify(i, n);
        }
    }

    fn heap_sort(&mut self) {
        self.swap_count = 0;
        self.build_heap();
        let n = self.baum.len();
        for r in (1..n).rev() {
            self.swap(0, r);
            self.heapify(0, r);
        }
        self.baum.reverse();
    }

    fn print_heap_tree(&self) {
        fn rec(baum: &Vec<i32>, idx: usize, depth: usize) {
            if idx >= baum.len() { return; }
            rec(baum, MaxHeap::get_rchild(idx), depth + 1);
            println!("{}{}", "   ".repeat(depth), baum[idx]);
            rec(baum, MaxHeap::get_lchild(idx), depth + 1);
        }
        rec(&self.baum, 0, 0);
    }
}

fn main() {
    let mut heap = MaxHeap::new();
    heap.fill_with_random_numbers(10);
    println!("Unsorted: {:?}", heap.baum);

    let start = Instant::now();
    heap.heap_sort();
    let duration = start.elapsed();

    println!("Sorted (desc): {:?}", heap.baum);
    println!(
        "Sortierdauer: {} ns ({} ms)",
        duration.as_nanos(),
        duration.as_secs_f64() * 1_000.0
    );
    println!("Anzahl Swaps: {}", heap.swap_count);

    println!("\nHeap-Tree:");
    heap.print_heap_tree();
}