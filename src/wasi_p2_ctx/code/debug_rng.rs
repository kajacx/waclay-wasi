pub(super) mod internal {
    pub struct DebugRng(pub simple_rng::RNG);

    impl std::fmt::Debug for DebugRng {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "simple_rng::RNG doesn't implement Debug :(")
        }
    }
}
