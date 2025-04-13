// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// Trait 对象，动态分发
// 动态分发的 trait 对象，表示任何实现了 Licensed 特质的类型的引用。
// 在运行时通过虚表（vtable）解析 licensing_info 方法的调用。

// 每个 &dyn Licensed 包含一个指向数据的指针和一个指向 vtable 的指针，
// vtable 存储了具体类型的函数实现。

// 当具体类型在编译时未知，或需要动态分配
// 参数必须是引用 
// 没有引用（或类似指针），Rust 无法处理 dyn Licensed 的动态大小，也无法在运行时解析具体类型
// 所有 dyn Trait（trait 对象）必须通过某种指针类型使用，因为 dyn Trait 是动态大小类型（DST, Dynamically Sized Type）。
// 要使用 dyn Trait，必须通过固定大小的指针类型（如引用 & 或智能指针 Box），因为指针本身是 Sized 的。

// &dyn Trait：借用引用，最常见。

// Box<dyn Trait>：拥有所有权的智能指针。

// Rc<dyn Trait> 或 Arc<dyn Trait>：引用计数的智能指针。



// fn compare_license_types(software: &dyn Licensed, software_two: &dyn Licensed) -> bool {


// 泛型，静态分发
// 在编译时为每种类型组合（T, U）生成具体的函数实现（单态化，monomorphization）

// fn compare_license_types<T: Licensed, U: Licensed>(software: T, software_two: U) -> bool {
// 必需是trait, 这里是错的，不能写类型
// fn compare_license_types<T: SomeSoftware, OtherSoftware>(software: T, software_two: T) -> bool {

fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
