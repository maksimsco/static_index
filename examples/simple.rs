use static_index::{codegen::static_index, codegen::StaticIndex, Indexable};

struct A;
struct B;
struct C;

#[derive(StaticIndex)]
struct D;

struct E;

static_index!(A, B, C);

static_index!(E);

fn main() {
    assert_eq!(1, <A as Indexable>::INDEX);
    assert_eq!(2, <B as Indexable>::INDEX);
    assert_eq!(3, <C as Indexable>::INDEX);
    assert_eq!(0, <D as Indexable>::INDEX);
    assert_eq!(4, <E as Indexable>::INDEX);
}
