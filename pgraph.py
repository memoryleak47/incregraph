from dataclasses import dataclass
from btypes import *

# This effectively is a slotted hashcons.

type Shape = PNode # a pnode where all variables are renamed in order 0..n

class PClass:
    def __init__(self, sh: Shape):
        self.sh = sh
        self.reactors = [] # : list[PTerm], applicable rules from here

    def __repr__(self):
        return f"PClass(sh={self.sh}, reactors={self.reactors})"

class PGraph:
    def __init__(self, rules: list[Rule]):
        self.pclasses = {} # PId -> PClass
        self.hashcons = {} # PNode -> PId

        for r in rules:
            self.add_rule(r)

    def add_pnode(self, pnode: PNode) -> AppliedPId:
        (sh, args) = shape(pnode)
        if sh not in self.hashcons:
            pid = PId(len(self.pclasses))
            self.pclasses[pid] = PClass(sh)
            self.hashcons[sh] = pid
        return AppliedPId(self.hashcons[sh], args)
        
    def add_pterm(self, t: PTerm) -> AppliedPId:
        if isinstance(t, App):
            args = tuple(map(self.add_pterm, t.args))
            t = App(t.f, args)
        return self.add_pnode(t)

    def add_rule(self, rule: Rule):
        app_pid = self.add_pterm(rule.lhs)
        f = lambda x: PVar(app_pid.args.index(x))
        renamed = rename_pterm(rule.rhs, f)
        self.pclasses[app_pid.pid].reactors.append(renamed)

    def dump(self):
        for (lhs, rhs) in self.hashcons.items():
            c = self.pclasses[rhs]
            print(f"{lhs} -> {rhs}\n\tshape={c.sh}\n\treactors={c.reactors}")
            

def rename_pterm(x: PTerm, f: Fn[PVar, PVar]):
    if isinstance(x, PVar):
        return f(x)
    if isinstance(x, App):
        args = tuple(map(lambda x: rename_pterm(x, f), x.args))
        return App(x.f, args)
    assert(False)

def shape(pnode: PNode) -> (Shape, tuple[PVar]):
    if isinstance(pnode, PVar):
        return (PVar(0), (pnode,))

    d = dict() # PVar "input" -> PVar "shape"
    def zip_d(t):
        out = []
        for v in t:
            assert(isinstance(v, PVar))
            if v not in d: d[v] = PVar(len(d))
            out.append(d[v])
        return tuple(out)

    new_args = []
    for a in pnode.args:
        if isinstance(a, PVar):
            new_args.append(zip_d((a,))[0])
        else:
            assert(isinstance(a, AppliedPId))
            new_args.append(
                AppliedPId(a.pid, zip_d(a.args))
            )
    shape = App(pnode.f, tuple(new_args))

    l = sorted(d.items(), key=lambda x: x[1].var_i)
    args = tuple(x[0] for x in l)
    
    return shape, args
