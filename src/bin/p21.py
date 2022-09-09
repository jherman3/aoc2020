import collections
from typing import Tuple

# // each turn:
# // t = 3 d3
# // pos = (pos + t) % 10
# // score += pos + 1
# 3d3
rolls = collections.defaultdict(int)
for i in range(1,4):
    for j in range(1,4):
        for k in range(1,4):
            rolls[i+j+k] += 1

SCORE_LIMIT = 21
cache = {}
def f(p1s: int, p2s: int, p1t: int, p2t: int) -> Tuple[int, int]:
    """num ways of p1 and p2 winning when it's p1's turn"""
    if (p1s, p2s, p1t, p2t) in cache:
        return cache[(p1s, p2s, p1t, p2t)]
    if p1s >= SCORE_LIMIT:
        return (1, 0)
    if p2s >= SCORE_LIMIT:
        return (0, 1)
    p1ways = 0
    p2ways = 0
    for roll, freq in rolls.items():
        new_t = (p1t + roll) % 10
        new_s = p1s + new_t + 1
        p2, p1 = f(p2s, new_s, p2t, new_t)
        p1ways += p1 * freq
        p2ways += p2 * freq
    cache[(p1s, p2s, p1t, p2t)] = (p1ways, p2ways)
    return (p1ways, p2ways)


# test problem: p1 is at 4 (3)
# p2 is at 8 (7)
a, b = f(0, 0, 3, 7)
print(max(a, b))

# real problem
# Player 1 starting position: 2
# Player 2 starting position: 5
a, b = f(0, 0, 1, 4)
print(max(a, b))
