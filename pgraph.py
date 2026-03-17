from dataclasses import dataclass
from types import *

class PClass:
    def __init__(self, node: PNode):
        self.arity = arity
        self.node = node

class PGraph:
    def __init__(self):
        self.classes = {} # PId -> PClass
