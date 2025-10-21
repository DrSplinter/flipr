use proptest::prelude::Strategy;
use proptest::strategy::ValueTree;
use proptest::test_runner::TestRunner;

pub fn sampler<T>(strategy: impl Strategy<Value = T>) -> impl Iterator<Item = T> {
    let mut runner = TestRunner::default();
    std::iter::from_fn(move || strategy.new_tree(&mut runner).ok().map(|v| v.current()))
}
