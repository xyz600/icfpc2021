# -*- coding: utf-8 -*-

import matplotlib.pyplot as plt
import json
import sys


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
        plt.plot(xs, ys, 'b-')

        # previous figure
        for e in self.figure_edges:
            xs = [self.figure_vertices[e[0]][0], self.figure_vertices[e[1]][0]]
            ys = [self.figure_vertices[e[0]][1], self.figure_vertices[e[1]][1]]
            plt.plot(xs, ys, '-', color='lightgray')


class Pose:
    def __init__(self, filepath):
        with open(filepath, 'r') as fin:
            js = json.load(fin)
            self.vertices = js['vertices']

    def plot(self, plt, problem):
        # previous figure
        for e in problem.figure_edges:
            xs = [self.vertices[e[0]][0], self.vertices[e[1]][0]]
            ys = [self.vertices[e[0]][1], self.vertices[e[1]][1]]
            plt.plot(xs, ys, '-', color='black')


if __name__ == "__main__":

    problem_id = sys.argv[1]
    problem_path = f"../data/in/{problem_id}.json"
    pose_path = f"{problem_id}.json"

    problem = Problem(problem_path)
    pose = Pose(pose_path)

    problem.plot(plt)
    pose.plot(plt, problem)

    plt.show()
