/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// 定义泛型排序函数，接受可变切片，元素类型 T 需实现 PartialOrd 以支持比较
fn sort<T: PartialOrd>(array: &mut [T]) {
    let n = array.len();
    
    // 从第二个元素开始，逐个插入到已排序部分
    for i in 1..n {
        let mut j = i;
        
        // 将当前元素与前一个元素比较，若前一个元素较大，则后移
        while j > 0 && array[j - 1] > array[j] {
            // 交换 array[j] 和 array[j - 1]
            array.swap(j, j - 1);
            j -= 1;
        }
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