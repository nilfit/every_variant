use crate::*;

// ======================= Implementations ===================
//
// These are some kind of dumb implementations for "every variant" that can be basic structs
// There are infinite numbers of variants for vectors and the like, this will just give a small
// taste

impl EveryVariant for String {
    fn every_variant() -> Vec<Self> {
        let mut vec = vec!["example String".into()];

        // add an empty string
        vec.push(String::new());

        // add a string with special chars
        vec.push(String::from("!\"¤%!#/&/(^^ÄÖ"));

        // null
        vec.push(String::from("null"));

        vec
    }
}

impl EveryVariant for &'static str {
    fn every_variant() -> Vec<Self> {
        let vec = vec!["&static str!"];
        vec
    }
}

impl EveryVariant for u32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![0, 1, 43, u32::MAX];
        vec
    }
}

impl EveryVariant for u64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![0, 1, 43, u64::MAX];
        vec
    }
}
impl EveryVariant for u16 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![0, 1, 43, u16::MAX];
        vec
    }
}
impl EveryVariant for u8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![0, 1, 43, u8::MAX];
        vec
    }
}

impl EveryVariant for i32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![i32::MIN, -1, 0, 1, i32::MAX];
        vec
    }
}

impl EveryVariant for i64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![i64::MIN, -1, 0, 1, i64::MAX];
        vec
    }
}
impl EveryVariant for i16 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![i16::MIN, -1, 0, 1, i16::MAX];
        vec
    }
}
impl EveryVariant for i8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![i8::MIN, -1, 0, 1, i8::MAX];
        vec
    }
}

impl EveryVariant for bool {
    fn every_variant() -> Vec<Self> {
        let vec = vec![true, false];
        vec
    }
}

impl EveryVariant for usize {
    fn every_variant() -> Vec<Self> {
        let vec = vec![0, 1, usize::MAX];
        vec
    }
}

impl EveryVariant for f32 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![f32::MIN, -1.0, 0.0, 1.0, f32::MAX, f32::NAN];
        vec
    }
}
impl EveryVariant for f64 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![f64::MIN, -1.0, 0.0, 1.0, f64::MAX, f64::NAN];
        vec
    }
}

impl EveryVariant for std::num::NonZeroU8 {
    fn every_variant() -> Vec<Self> {
        let vec = vec![Self::new(4).unwrap()];
        vec
    }
}

impl<T: EveryVariant + Clone + Sized> EveryVariant for Option<T> {
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push(None);

        let tvec = T::every_variant();
        let mut tvec = tvec.into_iter().map(|t| Some(t)).collect();
        vec.append(&mut tvec);

        vec
    }
}

impl EveryVariant for () {
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();
        vec.push(());
        vec
    }
}

impl<T: EveryVariant + Clone + Sized, E: EveryVariant + Clone + Sized> EveryVariant
    for Result<T, E>
{
    fn every_variant() -> Vec<Self> {
        let mut vec = Vec::new();

        let tvec = T::every_variant();
        let mut tvec = tvec.into_iter().map(|t| Ok(t)).collect();
        vec.append(&mut tvec);

        let evec = E::every_variant();
        let mut evec = evec.into_iter().map(|t| Err(t)).collect();
        vec.append(&mut evec);

        vec
    }
}

impl<T: EveryVariant + Clone + Sized> EveryVariant for Vec<T> {
    fn every_variant() -> Vec<Self> {
        // vector that contains one of every variant of the subsequent types
        let mut retvec = vec![T::every_variant()];

        // add an empty vector
        retvec.push(Vec::new());

        let single_variant = T::every_variant().first().cloned().unwrap();
        // add a single element vector
        let singlevec = vec![single_variant.clone()];
        retvec.push(singlevec);

        // add a large vector
        let mut largevec = Vec::new();
        while largevec.len() < 1000000 {
            largevec.push(single_variant.clone());
        }
        retvec.push(largevec);

        retvec
    }
}

#[cfg(feature = "ev_heapless")]
use heapless::{String as HString, Vec as HVec};

#[cfg(feature = "ev_heapless")]
impl<T, const N: usize> EveryVariant for HVec<T, N>
where
    T: EveryVariant + Clone + Sized,
{
    fn every_variant() -> Vec<Self> {
        let mut vec = HVec::new();

        for v in T::every_variant() {
            vec.push(v).ok();
        }

        vec![vec]
    }
}

#[cfg(feature = "ev_heapless")]
impl<const N: usize> EveryVariant for HString<N> {
    fn every_variant() -> Vec<Self> {
        let mut s = HString::new();
        s.push_str("hello").ok();
        vec![s]
    }
}

#[cfg(test)]
mod tests {
    use crate::EveryVariant;
    #[cfg(feature = "ev_heapless")]
    use heapless::{String as HString, Vec as HVec};

    /// Type of the message
    #[derive(EveryVariant, Debug, Clone)]
    enum MessageType {
        Codified,
        Markdown,
        Html,
    }

    /// This type can come in  4 different variants due the option
    #[derive(EveryVariant, Debug, Clone)]
    struct FormattedMessage {
        /// Enum dictating how to render the string, None means its hidden
        rendermethod: Option<MessageType>,
        /// The optional content of the message
        text: String,
    }

    #[test]
    fn small_example() {
        let all_diferent_messages = FormattedMessage::every_variant();
        println!("{:#?}", all_diferent_messages);
        assert_eq!(4, all_diferent_messages.len());
    }

    #[cfg(feature = "ev_heapless")]
    #[test]
    fn heapless() {
        let _s = HString::<16>::every_variant();
        let _v = HVec::<u8, 16>::every_variant();
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Message {
        pub message: String,
        pub number: u32,
        pub opt: Option<u64>,
        pub nest: Top,
        pub second: SecondTop,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum SecondTop {
        One,
        Two(Nested),
        Three,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Top {
        One,
        Nested(Nested),
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Nested {
        First,
        Second,
        Third,
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum TestUnnamed1 {
        UnnamedSingle(u16),
        UnnamedMultiple1(u16, u32),
        UnnamedMultiple2(u16, u32, u64),
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub enum TestNamed1 {
        NamedSingle { first: u16 },
        NamedMultiple1 { first: u16, second: u32 },
        NamedMultiple2 { first: u16, second: u32, third: u64 },
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct TestUnnamed2(u16, u32, u64);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Gen1<A: EveryVariant + Clone>(A);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Gen2<A: EveryVariant + Clone, B: EveryVariant + Clone>(A, B);

    #[derive(EveryVariant, Debug, Clone)]
    pub struct Generic1(Gen1<u8>, Gen2<u16, u32>);

    #[derive(EveryVariant, Debug, Clone)]
    pub enum Generic2 {
        G1(Gen1<i8>),
        G2(Gen2<i16, i32>),
    }

    #[derive(EveryVariant, Debug, Clone)]
    pub struct TestUnnamed3(pub u16);

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub struct GenericDerive<T> {
        value: T,
    }

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub enum GenericEnum<T> {
        One(T),
        Two(u32),
    }

    #[allow(unused)]
    #[derive(EveryVariant)]
    pub struct MultiGeneric<A, B>(A, B);

    #[test]
    fn messages_number() {
        let msgs = Message::every_variant().len();
        assert_eq!(40, msgs);
    }

    #[test]
    fn opts_number() {
        let msgs = Option::<u64>::every_variant().len();
        assert_eq!(2, msgs);
    }

    #[test]
    fn unnamed1() {
        let msgs = TestUnnamed1::every_variant().len();
        assert_eq!(3, msgs);
    }

    #[test]
    fn unnamed2() {
        let msgs = TestUnnamed2::every_variant().len();
        assert_eq!(1, msgs);
    }

    #[test]
    fn generic1() {
        let msgs = Generic1::every_variant().len();
        assert_eq!(1, msgs);
    }

    #[test]
    fn generic2() {
        let msgs = Generic2::every_variant().len();
        assert_eq!(2, msgs);
    }

    #[test]
    fn named_enum() {
        let msgs = TestNamed1::every_variant().len();
        assert_eq!(3, msgs);
    }

    #[test]
    fn generic_everyvariant() {
        let msgs = GenericDerive::<u32>::every_variant().len();
        assert_eq!(1, msgs);

        let msgs = GenericEnum::<u32>::every_variant().len();
        assert_eq!(2 * 1, msgs);

        let msgs = MultiGeneric::<u32, u32>::every_variant().len();
        assert_eq!(1, msgs);
    }
}
