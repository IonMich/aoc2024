from collections import Counter
import numpy as np

a = np.loadtxt('inputs/input.txt', dtype=int)
a1, a2 = a[:, 0], Counter(a[:, 1])
print(sum(i * a2[i] for i in set(a1)))