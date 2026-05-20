mod minqueue;
pub use minqueue::*;

mod pgraph;
pub use pgraph::*;

mod tests;

use symbol_table::GlobalSymbol as Symbol;
use std::collections::HashMap;

type Id = usize;
type Subst = Box<[Id]>;
type Score = usize;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node(Symbol, Box<[Id]>);

struct EGraph<'a> {
    pgraph: &'a PGraph,
    matches: HashMap<(Id, PId), Vec<Subst>>,
    uf: Vec</*Id -> */Id>,
    hashcons: HashMap<Node, Id>,
    queue: MinPrioQueue<Score, (PId, RhsId, Subst)>,
}

impl<'a> EGraph<'a> {
    fn new(pgraph: &'a PGraph) -> Self {
        Self {
            pgraph,
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

        self.match_node(i, &n);

        self.hashcons.insert(n, i);

        i
    }

    // returns true, if "matches" was changed.
    fn match_node(&mut self, i: Id, Node(f, args): &Node) -> bool {
        let mut changed = false;
        for (pid, c) in self.pgraph.pmap.iter().enumerate() {
            let PatNode(pf, pargs) = &c.node;
            if pf != f { continue }

            let init_subst = vec![Id::MAX; c.arity].into_boxed_slice();
            let mut substs = vec![init_subst];
            for i in 0..args.len() {
                let (child_pid, child_pargs): &(PId, Box<[PVar]>) = &pargs[i];
                for subst in std::mem::take(&mut substs) {
                    'l: for child_subst in &self.matches[&(args[i], *child_pid)] {
                        let mut subst = subst.clone();
                        for (a, v) in child_pargs.iter().zip(child_subst) {
                            if subst[*a] == *v || subst[*a] == Id::MAX {
                                subst[*a] = *v;
                            } else {
                                continue 'l;
                            }
                        }
                        substs.push(subst);
                    }
                }
            }

            let entry: &mut Vec<Subst> = self.matches.entry((i, pid)).or_insert(Vec::new());
            for subst in substs {
                if !entry.contains(&subst) {
                    changed = true;
                    entry.push(subst.clone());
                    for rhs_idx in 0..c.rhss.len() {
                        let score = 42;
                        self.queue.push(score, (pid, rhs_idx, subst.clone()));
                    }
                }
            }
        }
        changed
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
    // TODO should call "match_node" or something?
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
        if pid == 0 { return subst[0] }

        let PatNode(f, args) = &self.pgraph.pmap[pid].node;
        let args = args.iter().map(|(pid2, args2)| {
                let subst: &[Id] = subst;
                let args: &[PVar] = args2;
                // Example:
                // We have subst = [id0, id1, id2]
                // pid(0, 1, 2) = f(pid3(0, 1), pid2(2, 1))
                //                                   ^
                // Then args2 = [2, 1]               |  up here.
                // Then we want args3 = [id2, id1]
                let args3 = args2.iter().map(|x| subst[*x]).collect();
                self.instantiate_pid(*pid2, &args3)
            }).collect();
        self.add(Node(*f, args))
    }

    fn tick(&mut self) {
        let Some((_, (pid, rhs_id, subst))) = self.queue.pop() else { return };
        let rhs = &self.pgraph.pmap[pid].rhss[rhs_id];
        let instantiated_lhs = self.instantiate_pid(pid, &subst);
        let instantiated_rhs = self.instantiate_pattern(&rhs, &subst);
        self.union(instantiated_lhs, instantiated_rhs);
        self.rebuild();
        self.rematch();
    }

    // TODO make more efficient, like rebuilding.
    fn rematch(&mut self) {
        loop {
            let mut changed = false;
            for (n, i) in self.hashcons.clone() {
                if self.match_node(i, &n) { changed = true; }
            }
            if !changed { break }
        }
    }
}

fn main() {
}
