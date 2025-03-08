/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn heapify<T: PartialOrd>(array: &mut [T], n: usize, i: usize) {
    let mut largest = i; // 当前节点
    let left = 2 * i + 1; // 左子节点
    let right = 2 * i + 2; // 右子节点

    // 找到最大值所在的节点
    if left < n && array[left] > array[largest] {
        largest = left;
    }
    if right < n && array[right] > array[largest] {
        largest = right;
    }

    // 如果最大值不是当前节点，交换并递归调整
    if largest != i {
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    // 构建最大堆
    let n = array.len();
    for i in (0..n / 2).rev() {
        heapify(array, n, i);
    }

    // 排序
    for i in (1..n).rev() {
        array.swap(0, i); // 将堆顶元素（最大值）与最后一个元素交换
        heapify(array, i, 0); // 对剩余部分重新调整堆
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
