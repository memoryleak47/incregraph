from dataclasses import dataclass
from pgraph import *

class Class:
    def __init__(self):
        pass

class EGraph:
    def __init__(self, rules: list[Rule]):
        self.pgraph = PGraph(rules)

        self.classes = {} # Id -> Class
        self.hashcons = {} # Node -> Id
        self.uf = {} # Id -> Id

    def add_node(self, n: Node):
        n = Node(n.f, tuple(map(self.find, n.args)))
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
        ...
