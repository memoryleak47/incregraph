mod minqueue;
pub use minqueue::*;

use symbol_table::GlobalSymbol as Symbol;
use std::collections::HashMap;

type Id = usize;
type PId = usize;
type AppliedPId = (PId, Box<[PVar]>);
type Subst = Box<[Id]>;
type PVar = usize;
type Score = usize;
type RuleId = usize;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    f: Symbol,
    args: Box<[Id]>,
}

enum PatNode {
    PVar(PVar),
    Node(Symbol, Box<[AppliedPId]>),
}

struct EGraph {
    pmap: Vec</*PId -> */PatNode>,
    matches: HashMap<(Id, PId), Vec<Subst>>,
    uf: Vec</*Id -> */Id>,
    hashcons: HashMap<Node, Id>,
    queue: MinPrioQueue<Score, (RuleId, Subst)>,
}

impl EGraph {
    fn new() -> Self {
        Self {
            pmap: Default::default(),
            matches: Default::default(),
            uf: Default::default(),
            hashcons: Default::default(),
            queue: MinPrioQueue::new(),
        }
    }

    fn add(&mut self, mut n: Node) -> Id {
        let n = self.canon(n);
        if let Some(i) = self.hashcons.get(&n) { return *i }

        let i = self.uf.len();
        self.uf.push(i);
        self.hashcons.insert(n, i);
        i

        // TODO do matches here!
    }

    // You have to manually call rebuild after this!
    fn union(&mut self, x: Id, y: Id) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y { return }
        self.uf[x] = y;
    }

    fn canon(&self, mut n: Node) -> Node {
        n.args = n.args.into_iter().map(|x| self.find(x)).collect();
        n
    }

    // This rebuild isn't good for incremental stuff! Its too big.
    // We need parent pointers.
    fn rebuild(&mut self) {
        for (n, i) in std::mem::take(&mut self.hashcons) {
            let n = self.canon(n);
            let i = self.find(i);
            if let Some(j) = self.hashcons.get(&n) {
                self.union(i, *j);
            } else {
                self.hashcons.insert(n, i);
            }
        }
    }

    fn find(&self, mut x: Id) -> Id {
        loop {
            let y = self.uf[x];
            if x == y { return x }
            x = y;
        }
    }

    fn tick(&mut self) {
        let Some((_, (rule_id, subst))) = self.queue.pop() else { return };
    }
}

fn main() {
}
