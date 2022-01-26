fn main() {
    // 配列のデータ型は [T; N]
    // T: 要素の型, N: コンパイル時に決まる固定長
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}