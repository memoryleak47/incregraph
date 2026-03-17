from dataclasses import dataclass
from pgraph import *

class Class:
    def __init__(self):
        pass

class EGraph:
    def __init__(self, pgraph: PGraph):
        self.classes = {} # Id -> Class
        self.pgraph = pgraph
