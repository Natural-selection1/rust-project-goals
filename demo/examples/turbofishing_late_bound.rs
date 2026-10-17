fn foo<'a, T>(value: &'a T) -> &'a T {
    value
}

fn main() {
    let foo_fn: fn(&'static u32) -> &'static u32 = foo;
    const NUMBER: u32 = 42;
    assert_eq!(*foo_fn(&NUMBER), 42);
}

/* //! 运行: cargo run -p demo --example turbofishing_late_bound
//!
//! 提案: https://goals.rust-lang.org/2026/turbofishing-late-bound.html
//! 跟踪: https://github.com/rust-lang/goals/issues/711
//! 以下演示现有规则. 提案希望支持显式实例化 late-bound 参数,
//! 但具体语法和部分实例化的语义仍在探索, 不代表已经稳定.

// 'a 是 late-bound: 拿到函数时仍保留对不同生命周期的适用能力.
fn late<'a>(value: &'a u32) -> &'a u32 {
    value
}

// 添加生命周期约束后, 'a 成为 early-bound 参数.
// 'a: 'a 本身是平凡约束, 这里仅用来演示绑定方式的差异.
fn early<'a>(value: &'a u32) -> &'a u32
where
    'a: 'a,
{
    value
}

// 提案中的特殊情况: 'b 是 early-bound, 'a 仍是 late-bound.
fn mixed<'b, 'a>(value: &'a u32) -> &'a u32
where
    'b: 'b,
{
    value
}

static NUMBER: u32 = 42;

fn main() {
    // 1. 取得函数项, 并没有把生命周期固定成 'static.
    let f = late;
    let outer = 10;
    println!("函数项, 外层借用: {}", f(&outer));
    {
        let inner = 20;
        println!("同一个函数项, 内层借用: {}", f(&inner));
    }

    // 2. 显式展示其高阶生命周期能力: 对任意 'a 都适用.
    // 这里发生了函数项到函数指针的转换, 两者不是同一种类型.
    let any_lifetime: for<'a> fn(&'a u32) -> &'a u32 = late;
    println!("高阶函数指针: {}", any_lifetime(&outer));

    // 3. 当前规则下不能直接用 turbofish 指定 late-bound 生命周期.
    // 取消下一行注释, 可观察 E0794.
    // let explicitly_static = late::<'static>;

    // 已有的另一种表达: 转换为只接受 'static 引用的函数指针.
    // 这不等同于对原函数项显式传入 late-bound 参数.
    let static_pointer: fn(&'static u32) -> &'static u32 = late;
    println!("限定签名的函数指针: {}", static_pointer(&NUMBER));

    // 4. early-bound 参数可以通过 turbofish 指定.
    let explicitly_static = early::<'static>;
    println!("显式指定 early-bound 参数: {}", explicitly_static(&NUMBER));
    // 生命周期参数不会延长数据的存活时间. 局部变量无法满足这里的要求:
    // let invalid = explicitly_static(&outer);

    // 5. 原文中混合两类参数的例子, 按现有规则会产生未来兼容性警告:
    // let mixed_static = mixed::<'static>;
    // 这里 'static 对应的是 early-bound 的 'b, 不能据此认为输入的 'a
    // 已被固定为 'static. 默认不启用这行, 避免给整个 workspace 引入警告.
    let mixed_inferred = mixed;
    println!("混合参数, 由编译器推断: {}", mixed_inferred(&outer));
}
 */
