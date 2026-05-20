from btypes import *
from pgraph import *

f = lambda x, y: App("f", (x, y))
pv = PVar

rws = [
# commutativity!
    Rule(
        # lhs
        f(pv(12), pv(13)),

        # rhs
        f(pv(13), pv(12))
    )
]

pg = PGraph(rws)
pg.dump()
