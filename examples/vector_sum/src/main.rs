use krnl::macros::module;

#[module]
mod kernels {
    #[cfg(not(target_arch = "spirv"))]
    use krnl::krnl_core;
    use krnl_core::macros::kernel;

    #[kernel]
    pub fn vector_sum(#[item] a: f32, #[item] b: &mut f32) {
        *b += a
    }
}


fn main() {

}
