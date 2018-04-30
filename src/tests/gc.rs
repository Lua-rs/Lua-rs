use gc::{GcContext, GcObject};

#[test]
fn test_simple_allocate() {
    struct Simple(i64);
    unsafe impl GcObject for Simple {}

    let gc_context = GcContext::default();
    let r = gc_context.allocate_root(Simple(1));
    assert_eq!(r.read().0, 1);
    r.write().0 = 42;
    assert_eq!(r.read().0, 42);
}