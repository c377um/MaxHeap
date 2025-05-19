import java.util.ArrayList;
import java.util.Collections;

public class MaxHeap {
    private ArrayList<Integer> baum;
    private long swapCount;

    public MaxHeap() {
        baum = new ArrayList<>();
        swapCount = 0;
    }

    public void fillWithRandomNumbers(int i) {
        baum.clear();
        for (int j = 0; j < i; j++) {
            baum.add((int) (Math.random() * 1000));
        }
        swapCount = 0;
    }

    private int getLChild(int i) {
        return 2 * i + 1;
    }

    private int getRChild(int i) {
        return 2 * i + 2;
    }

    private void swap(int i, int j) {
        Collections.swap(baum, i, j);
        swapCount++;
    }

    private void heapify(int i, int r) {
        while (true) {
            int left  = getLChild(i);
            int right = getRChild(i);
            int largest = i;

            if (left < r && baum.get(left)  > baum.get(largest)) largest = left;
            if (right < r && baum.get(right) > baum.get(largest)) largest = right;

            if (largest != i) {
                swap(i, largest);
                i = largest;
            } else {
                break;
            }
        }
    }

    private void buildHeap() {
        int n = baum.size();
        for (int i = n/2 - 1; i >= 0; i--) {
            heapify(i, n);
        }
    }

    public void heapSort() {
        swapCount = 0;
        buildHeap();
        int n = baum.size();
        for (int r = n - 1; r > 0; r--) {
            swap(0, r);
            heapify(0, r);
        }
        // Einfachste Lösung: Umkehren der Liste
        Collections.reverse(baum); 
    }

    public long getSwapCount() {
        return swapCount;
    }

    public static void main(String[] args) {
        MaxHeap heap = new MaxHeap();
        heap.fillWithRandomNumbers(10);
        System.out.println("Unsorted: " + heap.baum);

        long start = System.nanoTime();
        heap.heapSort();
        long end = System.nanoTime();

        System.out.println("Sorted: " + heap.baum);
        System.out.printf("Sortierdauer: %d ns (%.3f ms)%n", end - start, (end - start)/1e6);
        System.out.println("Anzahl Swaps: " + heap.getSwapCount());
    }
}