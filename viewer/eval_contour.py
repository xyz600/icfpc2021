# -*- coding: utf-8 -*-

import json
import matplotlib.pyplot as plt
import sys
import numpy as np
from matplotlib import ticker, cm


class Problem:
    def __init__(self, filepath):
        with open(filepath, 'r') as fin:
            js = json.load(fin)
            self.hole_vertices = js['hole']
            self.figure_edges = js['figure']['edges']
            self.figure_vertices = js['figure']['vertices']

    def get_range(self):
        xs = [1e100, -1e100]
        ys = [1e100, -1e100]

        for v in self.hole_vertices:
            x = v[0]
            y = v[1]
            xs[0] = min(xs[0], x)
            xs[1] = max(xs[1], x)
            ys[0] = min(ys[0], y)
            ys[1] = max(ys[1], y)

        for v in self.figure_vertices:
            x = v[0]
            y = v[1]
            xs[0] = min(xs[0], x)
            xs[1] = max(xs[1], x)
            ys[0] = min(ys[0], y)
            ys[1] = max(ys[1], y)

        return xs, ys

    def plot(self, plt):

        # hole
        xs = [item[0]
              for item in self.hole_vertices] + [self.hole_vertices[0][0]]
        ys = [item[1]
              for item in self.hole_vertices] + [self.hole_vertices[0][1]]
        plt.plot(xs, ys, 'b-')

        # # previous figure
        # for e in self.figure_edges:
        #     xs = [self.figure_vertices[e[0]][0], self.figure_vertices[e[1]][0]]
        #     ys = [self.figure_vertices[e[0]][1], self.figure_vertices[e[1]][1]]
        #     plt.plot(xs, ys, '-', color='black')


def load_table(filepath):

    table = []
    with open(filepath, 'r') as fin:
        for line in fin.readlines():
            row = list(map(float, line.strip().split(" ")))
            table.append(row)
        return table


if __name__ == "__main__":

    for prob_id in range(1, 79):
        table = load_table(f"../data/debug/penalty_map_{prob_id}.txt")
        problem = Problem(f"../data/in/{prob_id}.json")

        n = len(table)

        xrange, yrange = problem.get_range()
        xs = np.linspace(xrange[0], xrange[1], n)
        ys = np.linspace(yrange[0], yrange[1], n)

        target_idx = 1

        cntr = plt.contour(xs, ys, table)
        plt.clabel(cntr)
        problem.plot(plt)
        plt.plot([problem.figure_vertices[target_idx][0]], [
            problem.figure_vertices[target_idx][1]], 'o', color='red')
        plt.savefig(f"../data/debug/penalty_contour_{prob_id}.png")
        plt.cla()
