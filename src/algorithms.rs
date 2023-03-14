fn bubble_sort(vec: Vec<usize>) -> Vec<usize> { 
    // vec with N items, arr.len() = N
    for i in 0..arr.len() {
        // the last pair is the (N-2)-th and (N-1)-th items
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }				
}