import matplotlib.pyplot as plt

a = [
    (0, 0, 1, 0),
    (0, 1, 1, 1),
    (0, 2, 2, 2),
    (0, 0, 0, 4),
    (1, -1, 1, 0),
    (2, -2, 2, 2),
    (1, 1, 2, 2),
    (1, 2, 2, 3),
    (3, 0, 2, 1),
    (1, 1, 0, 2),
    (3, 1, 2, 2),
    (1, 3, 0, 2),
]


fig  = plt.figure()
ax = fig.add_subplot(111)

for line in a:
    point = list(map(lambda a: float(a) - 1, line ))

    ax.axline(point[:2], point[2:])
ax.set_aspect('equal')
plt.show()