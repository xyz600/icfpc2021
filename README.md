# icfpc2021
for ICFPC2021

My solution is simulated annealing.
The neighborhood of SA is as follows.

1. swap position of neighbor points in the figure
2. move position of vertex to (-1, 0), (1, 0), (0, 1), (0, -1)

this project contains following tools.

* solver
  * solver described above
* submitter
  * upload solutions created locally
* problem-crawler
  * download problems
* hole_print, eval-contour-dump
  * for debug