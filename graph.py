import matplotlib.pyplot as plt
import numpy as np

networks = ("ALARM", "CHILD", "MUNIN")
ci_tests = {
    'PC':            (137, 230, 201),
    'Shortcut':      ( 84, 243, 240),
    'PC Dual':       (149, 240, 199),
    'Shortcut Dual': (127, 308, 250),
}

ms_ellapsed = {
    'PC':            (9, 230, 1239),
    'Shortcut':      (5, 259, 1046),
    'PC Dual':       (9, 408,  920),
    'Shortcut Dual': (8, 323, 1115),
}

errors = {
    'PC':            (4, 13, 6),
    'Shortcut':      (5, 11, 7),
    'PC Dual':       (5, 15, 6),
    'Shortcut Dual': (6,  9, 9),
}


x = np.arange(len(networks))  # the label locations
width = 0.15  # the width of the bars
multiplier = 0

fig, ax = plt.subplots(layout='constrained')

for attribute, measurement in ms_ellapsed.items():
    offset = width * multiplier
    rects = ax.bar(x + offset, measurement, width, label=attribute)
    ax.bar_label(rects, padding=3)
    multiplier += 1

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Milliseconds')
ax.set_title('Time Taken')
ax.set_xticks(x + width, networks)
ax.legend(loc='upper left', ncols=2)
ax.set_ylim(0, 1300)

plt.savefig('ms.png')
