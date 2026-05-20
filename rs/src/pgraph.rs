use crate::*;

pub type PId = usize;
pub type PVar = usize;
pub type RhsId = usize;

// PId 0 always means PVar. See args[0] for the var.
pub type AppliedPId = (PId, /*args*/ Box<[PVar]>);

#[derive(PartialEq, Debug)]
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

#[derive(Clone, Debug)]
pub enum Pattern {
    PVar(PVar),
    Node(Symbol, Box<[Pattern]>),
}

#[derive(Debug)]
pub struct PClass {
    pub node: PatNode,
    pub rhss: Vec<Pattern>,
}

#[derive(Debug)]
pub struct PGraph {
    // never index with 0!
    pub pmap: Vec</*PId -> */ PClass>,
}

impl PGraph {
    pub fn new() -> Self {
        let var_pnode = PatNode(Symbol::new("var--never-use"), Box::new([]));
        let var_pclass = PClass {
            node: var_pnode,
            rhss: Vec::new(),
        };
        Self {
            pmap: vec![var_pclass],
        }
    }

    pub fn add_rule(&mut self, lhs: Pattern, rhs: Pattern) {
        let (pid, pargs) = self.add_pattern(lhs);
        self.pmap[pid].rhss.push(rhs.rename_rev(&pargs));
    }

    pub fn add_pattern(&mut self, pat: Pattern) -> AppliedPId {
        match pat {
            Pattern::PVar(v) => (0, Box::new([v])),
            Pattern::Node(f, args) => {
                let pnode = PatNode(f, args.into_iter().map(|x| self.add_pattern(x)).collect());
                let (pnode, m) = canon_node(pnode);

                if let Some(i) = self.pmap.iter().position(|c| c.node == pnode) {
                    return (i, todo!())
                } else {
                    let i = self.pmap.len();
                    self.pmap.push(PClass {
                        node: pnode,
                        rhss: Vec::new()
                    });
                    (i, m)
                }
            },
        }
    }
}

fn canon_node(PatNode(f, args): PatNode) -> (PatNode, Box<[PVar]>) {
    let mut ret = Vec::new();
    let mut m = HashMap::new();
    let mut outargs = Vec::new();
    for (pid, pvars) in args {
        let mut outpvars = Vec::new();
        for v in pvars {
            if !m.contains_key(&v) {
                m.insert(v, m.len());
                ret.push(v);
            }
            outpvars.push(m[&v]);
        }
        outargs.push((pid, outpvars.into()));
    }
    let n = PatNode(f, outargs.into());
    (n, ret.into())
}

impl Pattern {
    pub fn rename_rev(&self, m: &[PVar]) -> Pattern {
        match self {
            Pattern::PVar(v) => Pattern::PVar(m.iter().position(|x| *x == *v).unwrap()),
            Pattern::Node(f, args) => Pattern::Node(*f, args.iter().map(|x| x.rename_rev(m)).collect())
        }
    }
}
