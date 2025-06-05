fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_index);
    i
}

fn main() {
    // 测试整数排序
    println!("测试整数排序：");
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", numbers);
    quicksort(&mut numbers);
    println!("排序后: {:?}", numbers);
    
    // 测试字符串排序
    println!("\n测试字符串排序：");
    let mut strings = vec!["香蕉", "苹果", "橙子", "葡萄", "西瓜"];
    println!("排序前: {:?}", strings);
    quicksort(&mut strings);
    println!("排序后: {:?}", strings);
}
