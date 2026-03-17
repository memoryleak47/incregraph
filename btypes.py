from dataclasses import dataclass

@dataclass(frozen=True)
class App: # generic over T
    f: str
    args: tuple[T]

    def __post_init__(self):
        assert(isinstance(self.args, tuple))

    def __repr__(self):
        if len(self.args) > 0:
            return self.f + "(" + ", ".join(map(str, self.args)) + ")"
        else:
            return self.f

@dataclass(frozen=True)
class Id: # corresponds to e-graph e-classes
    id_i: int

    def __repr__(self):
        return f"id{self.id_i}"

type Node = App # which recurses via Id
type Term = App # which recurses via Term

# -- pattern things --

@dataclass(frozen=True)
class PId: # corresponds to patterns
    pid_i: int

    def __repr__(self):
        return f"pid{self.pid_i}"

@dataclass(frozen=True)
class AppliedPId:
    pid: PId
    args: tuple[PVar]

    def __repr__(self):
        if len(self.args) > 0:
            return str(self.pid) + "(" + ", ".join(map(str, self.args)) + ")"
        else:
            return str(self.pid)


@dataclass(frozen=True)
class PVar: # aka 'Slot'
    var_i: int

    def __repr__(self):
        return f"V{self.var_i}"

type PNode = PVar|App # which recurses via AppliedPId
type PTerm = PVar|App # which recurses via PTerm
# PTerm is typically called "Pattern"

@dataclass(frozen=True)
class Rule:
    lhs: PTerm
    rhs: PTerm
