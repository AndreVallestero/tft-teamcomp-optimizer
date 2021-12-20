# tft-teamcomp-optimizer

![unknown](https://user-images.githubusercontent.com/39736205/124393917-e0536b80-dcca-11eb-8aed-ada95a06e3e3.png)

This project generates the most efficient team comps (synergy-wise) within a given amount of units available. It is adaptable for different sets and will work between patches with minor changes.

## Benchmarks

Generate 8 unit comps, 100 run average, using Rust unless specified

- 2h python
- 5.10s original rust implementation
- 4.91s no empty synergy / variable length champ synergies
- 5.73s refactor using enum, no empty synergy

## TODO

- Optimize using adjacency matrix (2d or 3d. champ, class, origin) with max energy n (number of units in a comp) via breadth first search. Once trait / champ weight + bias is implemented, use that in BFS via Dijkstra.
  - Also check out optimizations mentioned [here](https://redd.it/oams7w)
- Change "enable 5+" to "limit 5+" for more granular control
- Add disable traits