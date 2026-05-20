from dataclasses import dataclass
from pgraph import *

# every PVar is a number from 0..n, so we can express a subst via a list.
type Subst = tuple[Id]

class Class:
    def __init__(self):
        self.matches = {} # PId -> list[Subst]

class EGraph:
    def __init__(self, rules: list[Rule]):
        self.pgraph = PGraph(rules)

        self.classes = {} # Id -> Class
        self.hashcons = {} # Node -> Id
        self.uf = {} # Id -> Id

    def canon_node(self, n: Node) -> Node:
        return Node(n.f, tuple(map(self.find, n.args)))

    def add_node(self, n: Node):
        n = self.canon_node(n)
        if n not in self.hashcons:
            i = Id(len(self.classes))
            self.classes[i] = Class()
            self.uf[i] = i
            self.hashcons[n] = i
        return self.hashcons[n]

    def find(self, x: Id) -> Id:
        while True:
            y = self.uf[x]
            if x == y: return x
            x = y

    def union(self, x: Id, y: Id):
        x = self.find(x)
        y = self.find(y)
        if x == y: return
        self.uf[x] = y
        self.rebuild()

    def rebuild(self):
        dirty = True
        while dirty:
            hashcons = {}
            dirty = False
            for (n, x) in self.hashcons.items():
                n = self.canon_node(n)
                x = self.find(x)
                if n in hashcons:
                    self.union(hashcons[n], x)
                    dirty = True
                else:
                    hashcons[n] = x
            self.hashcons = hashcons
