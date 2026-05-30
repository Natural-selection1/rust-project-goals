async fn 简单(compute: impl Future<Output = ()>) {
    async {}.await;
    compute.await;
}

#[test]
fn show_size() {
    use std::mem::size_of_val;
    let blob = [0u8; 10];
    let 单层 = 简单(async move {
        async {}.await;
        println!("{:?}", blob)
    });
    let 多层 = 简单(
        简单(
            简单(
                简单(async move {
        async {}.await;
        println!("{:?}", blob)
    }))));
    println!("{}, {}", size_of_val(&单层), size_of_val(&多层));
}
