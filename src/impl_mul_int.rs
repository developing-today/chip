macro_rules! impl_mul_int {
    ($($t:ty)*) => ($(
        impl Mul<$t> for Amount {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self { value: self.value * rhs as u64 }
            }
        }

        impl Mul<Amount> for $t {
            type Output = Amount;
            fn mul(self, rhs: Amount) -> Self::Output {
                Self::Output { value: self as u64 * rhs.value }
            }
        }
    )*)
}
impl_mul_int! { u8 u16 u32 u64 usize }
// todo learn macros
