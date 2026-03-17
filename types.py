from dataclasses import dataclass

type Id = int # corresponds to e-graph e-classes
type PId = int # corresponds to patterns

@dataclass(frozen=True)
class App: # generic over T
    f: str
    args: tuple[T]

type Node = App # which recurses via Id
type Term = App # which recurses via Term

type PNode = App # which recurses via PId
type PTerm = App # which recurses via PTerm (PTerm = pattern)

@dataclass(frozen=True)
class Rule:
    lhs: Pattern
    rhs: Pattern
