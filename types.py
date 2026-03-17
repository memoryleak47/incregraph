from dataclasses import dataclass

@dataclass(frozen=True)
class App: # generic over T
    f: str
    args: tuple[T]

@dataclass(frozen=True)
class Id: # corresponds to e-graph e-classes
    id_i: int

type Node = App # which recurses via Id
type Term = App # which recurses via Term

# -- pattern things --

@dataclass(frozen=True)
class PId # corresponds to patterns
    pid_i: int

@dataclass(frozen=True)
class AppliedPId:
    pid: PId
    args: tuple[PVar]

@dataclass(frozen=True)
class PVar: # aka 'Slot'
    var_i: int

type PNode = App # which recurses via AppliedPId|PVar
type PTerm = App # which recurses via PTerm (PTerm = pattern)

@dataclass(frozen=True)
class Rule:
    lhs: Pattern
    rhs: Pattern
