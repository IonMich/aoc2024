import numpy as np

a = np.loadtxt('inputs/input.txt', dtype=int)
a1, a2 = a[:, 0], a[:, 1]
print(np.sum(np.abs(np.sort(a2) - np.sort(a1))))