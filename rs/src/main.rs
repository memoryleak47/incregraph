mod minqueue;
pub use minqueue::*;

use symbol_table::GlobalSymbol as Symbol;
use std::collections::HashMap;

type Id = usize;
type PId = usize;
type Subst = Box<[Id]>;
type PVar = usize;
type Score = usize;
type RhsId = usize;

#[derive(PartialEq, Eq, Hash)]
struct Node(Symbol, Box<[Id]>);

// PId 0 always means PVar.
type AppliedPId = (PId, Box<[PVar]>);

struct PatNode(Symbol, Box<[AppliedPId]>);

#[derive(Clone)]
enum Pattern {
    PVar(PVar),
    Node(Symbol, Box<[Pattern]>),
}

struct EGraph {
    pmap: Vec</*PId -> */(PatNode, /*rhss: */Box<[Pattern]>)>,
    matches: HashMap<(Id, PId), Vec<Subst>>,
    uf: Vec</*Id -> */Id>,
    hashcons: HashMap<Node, Id>,
    queue: MinPrioQueue<Score, (PId, RhsId, Subst)>,
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

    fn canon(&self, Node(f, args): Node) -> Node {
        let args = args.into_iter().map(|x| self.find(x)).collect();
        Node(f, args)
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

    fn instantiate_pattern(&mut self, pat: &Pattern, subst: &Subst) -> Id {
        match pat {
            Pattern::PVar(v) => subst[*v],
            Pattern::Node(f, args) => {
                let args = args.iter().map(|x| self.instantiate_pattern(x, subst)).collect();
                self.add(Node(*f, args))
            },
        }
    }

    fn instantiate_pid(&mut self, pid: PId, subst: &Subst) -> Id {
        let (node, _) = &self.pmap[pid];
        todo!()
    }

    fn tick(&mut self) {
        let Some((_, (pid, rhs_id, subst))) = self.queue.pop() else { return };
        let (_, rhss) = &self.pmap[pid];
        let rhs = rhss[rhs_id].clone(); // TODO fix useless clone.
        let instantiated_lhs = self.instantiate_pid(pid, &subst);
        let instantiated_rhs = self.instantiate_pattern(&rhs, &subst);
        self.union(instantiated_lhs, instantiated_rhs);
        self.rebuild();
    }
}

fn main() {
}
