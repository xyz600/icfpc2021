# -*- coding: utf-8 -*-

import matplotlib.pyplot as plt
import json


def load_data(filepath):
    with open(filepath) as fin:
        tris = []
        for line in fin.readlines():
            vs = list(map(float, line.strip().split(" ")))
            tris.append(((vs[0], vs[1]), (vs[2], vs[3]), (vs[4], vs[5])))

        return tris


class Problem:
    def __init__(self, filepath):
        with open(filepath, 'r') as fin:
            js = json.load(fin)
            self.hole_vertices = js['hole']
            self.figure_edges = js['figure']['edges']
            self.figure_vertices = js['figure']['vertices']

    def plot(self, plt):

        # hole
        xs = [item[0]
              for item in self.hole_vertices] + [self.hole_vertices[0][0]]
        ys = [item[1]
              for item in self.hole_vertices] + [self.hole_vertices[0][1]]
        plt.plot(xs, ys, '-', color='lightgray')


def plot(data):

    for tri in data:
        xs = [tri[0][0], tri[1][0], tri[2][0], tri[0][0]]
        ys = [tri[0][1], tri[1][1], tri[2][1], tri[0][1]]
        plt.plot(xs, ys, 'b-')


if __name__ == "__main__":

    for i in range(1, 79):
        filepath = f"../data/debug/hole_{i}.txt"
        data = load_data(filepath)

        problem = Problem(f"../data/in/{i}.json")
        problem.plot(plt)
        plot(data)
        plt.savefig(f"../data/debug/img_{i}.png")
        print(f"{i} finished")
        plt.cla()
