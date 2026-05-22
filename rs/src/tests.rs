use crate::*;

fn plus(l: Pattern, r: Pattern) -> Pattern { Pattern::Node(Symbol::new("plus"), Box::new([l, r])) }
fn mul(l: Pattern, r: Pattern) -> Pattern { Pattern::Node(Symbol::new("mul"), Box::new([l, r])) }
fn neg(l: Pattern) -> Pattern { Pattern::Node(Symbol::new("neg"), Box::new([l])) }
fn zero() -> Pattern { Pattern::Node(Symbol::new("zero"), Box::new([])) }
fn one() -> Pattern { Pattern::Node(Symbol::new("one"), Box::new([])) }
fn x() -> Pattern { Pattern::PVar(17) }
fn y() -> Pattern { Pattern::PVar(42) }
fn a() -> Pattern { Pattern::Node(Symbol::new("a"), Box::new([])) }

#[test]
fn test1() {
    let mut pgraph = PGraph::new();

    pgraph.add_rule(
        plus(x(), neg(x())),
        zero()
    );

    pgraph.add_rule(
        plus(x(), y()),
        plus(y(), x()),
    );

    pgraph.add_rule(
        neg(zero()),
        zero()
    );

    pgraph.add_rule(
        plus(x(), zero()),
        x(),
    );

    let mut eg = EGraph::new(&pgraph);

    let l = eg.add_term(plus(a(), neg(a())));
    let r = eg.add_term(zero());

    for _ in 0..10 {
        eg.tick();
    }
    assert_eq!(eg.find(l), eg.find(r));
}


#[test]
fn test2() {
    let mut pgraph = PGraph::new();

    pgraph.add_rule(
        mul(x(), zero()),
        zero()
    );

    pgraph.add_rule(
        mul(x(), one()),
        x()
    );

    pgraph.add_rule(
        mul(x(), y()),
        mul(y(), x())
    );

    pgraph.add_rule(
        plus(x(), zero()),
        x()
    );

    pgraph.add_rule(
        plus(x(), y()),
        plus(y(), x()),
    );

    let mut eg = EGraph::new(&pgraph);

    // Proving that: (a * 0) + (1 * a) == a
    let l = eg.add_term(plus(mul(a(), zero()), mul(one(), a())));
    let r = eg.add_term(a());

    for _ in 0..20 {
        eg.tick();
    }
    assert_eq!(eg.find(l), eg.find(r));
}
