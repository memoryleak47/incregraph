use crate::*;

pub type PId = usize;
pub type PVar = usize;
pub type RhsId = usize;

// PId 0 always means PVar. See args[0] for the var.
pub type AppliedPId = (PId, /*args*/ Box<[PVar]>);

pub struct PatNode(pub Symbol, pub Box<[AppliedPId]>);

pub fn varcount(args: &[AppliedPId]) -> PVar {
    let mut vc = 0;
    for (_, vs) in args {
        for v in vs {
            vc = vc.max(*v);
        }
    }
    vc
}

#[derive(Clone)]
pub enum Pattern {
    PVar(PVar),
    Node(Symbol, Box<[Pattern]>),
}

pub struct PGraph {
    // never index with 0!
    pub pmap: Vec</*PId -> */(PatNode, /*rhss: */Vec<Pattern>)>,
}

impl PGraph {
    pub fn new() -> Self {
        Self {
            pmap: Vec::new(), // TODO initial thing?
        }
    }

    pub fn add_rule(&mut self, lhs: Pattern, rhs: Pattern) {
        let (pid, pargs) = self.add_pattern(lhs);
        self.pmap[pid].1.push(rhs);
    }

    pub fn add_pattern(&mut self, pat: Pattern) -> AppliedPId {
        todo!()
    }
}
