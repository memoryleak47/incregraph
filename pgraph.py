from dataclasses import dataclass
from types import *

# This effectively is a slotted hashcons.

type Shape = PNode # a pnode where all variables are renamed in order 0..n

class PClass:
    def __init__(self, sh: Shape):
        self.sh = sh
        self.reactors = [] # : list[PTerm], applicable rules from here

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
            self.pclasses[pid] = Class(sh)
            self.hashcons[sh] = pid
        return AppliedPId(self.hashcons[sh], args)
        
    def add_term(self, t: PTerm) -> AppliedPId:
        pass

    def add_rule(self, rule: Rule):
        app_pid = self.add_term(self.lhs)
        renamed = rename(rule.rhs, app_pid.args)
        self.classes[app_pid.pid].reactors.append(renamed)

def rename(x, f: Fn[PVar, PVar]):
    if isinstance(x, PVar):
        return f(x)
    if isinstance(x, AppliedPId):
        args = tuple(map(lambda x: rename(x, f), x.args))
        return AppliedPId(x.pid, args)
    if isinstance(x, App):
        args = tuple(map(lambda x: rename(x, f), x.args))
        return App(x.f, args)
    assert(False)

def shape(pnode: PNode) -> (Shape, tuple[PVar]):
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
    shape = PNode(pnode.f, new_args)

    args = []
    for (v, pos) in d.items():
        args[pos] = v
    args = tuple(args)
    
    return shape, args
