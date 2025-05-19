import random
import time

class MaxHeap:
    def __init__(self):
        self.baum = []
        self.swap_count = 0

    def fill_with_random(self, n, *, lo=0, hi=100):
        self.baum = [random.randrange(lo, hi) for _ in range(n)]
        self.swap_count = 0

    def _lchild(self, i): return 2*i + 1
    def _rchild(self, i): return 2*i + 2

    def _swap(self, i, j):
        self.baum[i], self.baum[j] = self.baum[j], self.baum[i]
        self.swap_count += 1

    def _heapify(self, i, size):
        while True:
            left  = self._lchild(i)
            right = self._rchild(i)
            largest = i
            if left < size and self.baum[left]  > self.baum[largest]:
                largest = left
            if right < size and self.baum[right] > self.baum[largest]:
                largest = right
            if largest != i:
                self._swap(i, largest)
                i = largest
            else:
                break

    def _build_heap(self):
        n = len(self.baum)
        for i in range(n//2 - 1, -1, -1):
            self._heapify(i, n)

    def heap_sort(self):
        self.swap_count = 0
        self._build_heap()
        n = len(self.baum)
        for end in range(n-1, 0, -1):
            self._swap(0, end)
            self._heapify(0, end)
        self.baum.reverse()

    def print_tree(self):
        def _rec(idx, depth):
            if idx >= len(self.baum):
                return
            _rec(self._rchild(idx), depth+1)
            print("   " * depth + str(self.baum[idx]))
            _rec(self._lchild(idx), depth+1)
        _rec(0, 0)

if __name__ == "__main__":
    heap = MaxHeap()
    heap.fill_with_random(10)
    print("Unsorted:", heap.baum)

    start = time.perf_counter()
    heap.heap_sort()
    duration = time.perf_counter() - start

    print("Sorted (desc):", heap.baum)
    print(f"Sortierdauer: {duration*1e3:.3f} ms ({duration*1e9:.0f} ns)")
    print("Anzahl Swaps:", heap.swap_count)
    print("\nHeap-Tree:")
    heap.print_tree()
