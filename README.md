# tft-teamcomp-optimizer

This project generates the most efficient team comps (synergy-wise) within a given amount of units available. It is adaptable different sets and will work between patches with minor changes. It takes ~24 hours to run as it has to generate and calculate almost 60 billion combinations, so be ready to leave your computer on over night.

It does not take into account in-game effects such as `chosen one` from TFT Set 4 or spatula items.

Here's some sample data from TFT Set 4.0:

### Most synergies for 9 units

- 21: Azir, Lee Sin, Sett, Irelia, Janna, Jarvan, Shen, Vi, Yasuo
- 21: Azir, Lee Sin, Cassiopeia, Fiora, Garen, Irelia, Jarvan, Shen, Thresh
- 21: Azir, Lee Sin, Cassiopeia, Fiora, Irelia, Jarvan, Nidalee, Shen, Vayne
- 21: Azir, Lee Sin, Cassiopeia, Irelia, Katarina, Riven, Shen, Talon, Xin Zhao
- 21: Azir, Lee Sin, Fiora, Irelia, Janna, Jarvan, Katarina, Shen, Talon
- 21: Azir, Lee Sin, Garen, Irelia, Janna, Jarvan, Jax, Shen, Wukong
- 21: Azir, Lee Sin, Garen, Irelia, Janna, Riven, Shen, Thresh, Xin Zhao
- 21: Azir, Lee Sin, Irelia, Janna, Jarvan, Jax, Shen, Vi, Warwick
- 21: Azir, Lee Sin, Irelia, Janna, Nidalee, Riven, Shen, Vayne, Xin Zhao
- 21: Azir, Sett, Irelia, Janna, Jarvan, Jax, Shen, Vi, Yasuo
- 21: Azir, Sett, Irelia, Janna, Jarvan, Shen, Warwick, Xin Zhao, Yasuo
- 21: Azir, Yone, Aphelios, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 21: Azir, Yone, Ashe, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 21: Azir, Yone, Irelia, Janna, Jarvan, Kindred, Shen, Vi, Warwick
- 21: Azir, Yone, Irelia, Janna, Jarvan, Kindred, Vi, Warwick, Yuumi
- 21: Azir, Yone, Irelia, Jarvan, Kindred, Shen, Vi, Warwick, Yuumi
- 21: Azir, Ahari, Irelia, Janna, Jarvan, Kindred, Shen, Vi, Warwick
- 21: Azir, Cassiopeia, Fiora, Garen, Irelia, Jarvan, Jax, Shen, Thresh
- 21: Azir, Cassiopeia, Fiora, Garen, Irelia, Riven, Shen, Wukong, Xin Zhao
- 21: Azir, Cassiopeia, Fiora, Irelia, Jarvan, Jax, Nidalee, Shen, Vayne
- 21: Azir, Cassiopeia, Fiora, Irelia, Jarvan, Shen, Thresh, Wukong, Xin Zhao
- 21: Azir, Cassiopeia, Fiora, Irelia, Riven, Shen, Vi, Warwick, Xin Zhao
- 21: Azir, Cassiopeia, Garen, Irelia, Jarvan, Lux, Morgana, Shen, Thresh
- 21: Azir, Cassiopeia, Garen, Irelia, Katarina, Riven, Shen, Talon, Wukong
- 21: Azir, Cassiopeia, Irelia, Jarvan, Katarina, Shen, Talon, Thresh, Wukong
- 21: Azir, Cassiopeia, Irelia, Jarvan, Lux, Morgana, Nidalee, Shen, Vayne
- 21: Azir, Cassiopeia, Irelia, Jax, Katarina, Riven, Shen, Talon, Xin Zhao
- 21: Azir, Cassiopeia, Irelia, Katarina, Riven, Shen, Talon, Vi, Warwick
- 21: Azir, Fiora, Irelia, Janna, Jarvan, Jax, Katarina, Shen, Talon
- 21: Azir, Fiora, Irelia, Janna, Jarvan, Lux, Morgana, Shen, Xin Zhao
- 21: Azir, Fiora, Irelia, Jarvan, Kindred, Shen, Vi, Warwick, Yuumi
- 21: Azir, Fiora, Irelia, Jarvan, Kindred, Shen, Warwick, Xin Zhao, Yuumi 

Without 5 costs:

- 21: Ahri, Ashe, Irelia, Lulu, Maokai, Nami, Shen, Warwick, Yuumi
- 21: Ahri, Ashe, Irelia, Lulu, Nami, Nunu, Shen, Warwick, Yuumi
- 21: Ahri, Ashe, Irelia, Maokai, Nami, Shen, Veigar, Warwick, Yuumi
- 21: Ahri, Ashe, Irelia, Nami, Nunu, Shen, Veigar, Warwick, Yuumi
- 21: Ahri, Irelia, Janna, Kindred, Lulu, Maokai, Shen, Veigar, Warwick
- 21: Ahri, Irelia, Janna, Kindred, Lulu, Nunu, Shen, Veigar, Warwick
- 21: Aphelios, Diana, Irelia, Kindred, Shen, Sylas, Talon, Warwick, Yuumi
- 21: Aphelios, Irelia, Kindred, Lissandra, Morgana, Shen, Sylas, Warwick, Yuumi
- 21: Irelia, Kindred, Lulu, Maokai, Nami, Shen, Veigar, Warwick, Yuumi
- 21: Irelia, Kindred, Lulu, Nami, Nunu, Shen, Veigar, Warwick, Yuumi
- 20: Aatrox, Cassiopeia, Elise, Fiora, Irelia, Kalista, Riven, Shen, Wukong
- 20: Aatrox, Cassiopeia, Elise, Irelia, Pyke, Riven, Shen, Talon, Wukong
- 20: Aatrox, Cassiopeia, Fiora, Irelia, Jhin, Kalista, Shen, Vayne, Wukong
- 20: Aatrox, Cassiopeia, Irelia, Jax, Kalista, Pyke, Shen, Talon, Thresh
- 20: Aatrox, Cassiopeia, Irelia, Jhin, Pyke, Shen, Talon, Vayne, Wukong
- 20: Aatrox, Elise, Irelia, Janna, Jax, Kalista, Riven, Shen, Thresh
- 20: Aatrox, Elise, Irelia, Janna, Jhin, Riven, Shen, Vayne, Wukong
- 20: Aatrox, Evelynn, Irelia, Janna, Kalista, Kayn, Shen, Wukong, Yasuo
- 20: Aatrox, Fiora, Irelia, Janna, Kalista, Pyke, Shen, Talon, Wukong
- 20: Aatrox, Irelia, Janna, Jax, Jhin, Kalista, Shen, Thresh, Vayne
- 20: Ahri, Aphelios, Diana, Irelia, Shen, Sylas, Talon, Warwick, Yuumi
- 20: Ahri, Aphelios, Irelia, Lissandra, Morgana, Shen, Sylas, Warwick, Yuumi
- 20: Ahri, Ashe, Irelia, Janna, Kindred, Lulu, Shen, Veigar, Warwick
- 20: Ahri, Ashe, Irelia, Kindred, Lulu, Maokai, Nami, Shen, Warwick
- 20: Ahri, Ashe, Irelia, Kindred, Lulu, Nami, Nunu, Shen, Warwick
- 20: Ahri, Ashe, Irelia, Kindred, Maokai, Nami, Shen, Veigar, Warwick
- 20: Ahri, Ashe, Irelia, Kindred, Nami, Nunu, Shen, Veigar, Warwick
- 20: Annie, Irelia, Katarina, Kindred, Shen, Tahm Kench, Talon, Warwick, Yuumi
- 20: Aphelios, Cassiopeia, Diana, Irelia, Riven, Shen, Sylas, Talon, Warwick
- 20: Aphelios, Cassiopeia, Diana, Irelia, Shen, Sylas, Talon, Thresh, Warwick
- 20: Aphelios, Cassiopeia, Diana, Irelia, Shen, Sylas, Talon, Vayne, Warwick
- 20: Aphelios, Cassiopeia, Irelia, Lissandra, Morgana, Riven, Shen, Sylas, Warwick 

### Most synergies for 8 units

- 19: Azir, Lee Sin, Yone, Irelia, Janna, Jarvan, Shen, Xin Zhao
- 19: Azir, Yone, Garen, Irelia, Janna, Jarvan, Shen, Wukong
- 19: Azir, Yone, Irelia, Janna, Jarvan, Jax, Shen, Xin Zhao
- 19: Azir, Yone, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 19: Azir, Aphelios, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 19: Azir, Ashe, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 19: Azir, Irelia, Janna, Jarvan, Kindred, Shen, Vi, Warwick
- 19: Azir, Irelia, Jarvan, Kindred, Shen, Vi, Warwick, Yuumi
- 19: Sett, Ashe, Irelia, Janna, Maokai, Nunu, Shen, Warwick
- 19: Sett, Fiora, Irelia, Kindred, Shen, Warwick, Yasuo, Yuumi
- 18: Azir, Lee Sin, Sett, Yone, Fiora, Irelia, Jarvan, Vi
- 18: Azir, Lee Sin, Sett, Fiora, Irelia, Jarvan, Shen, Vi
- 18: Azir, Lee Sin, Sett, Irelia, Janna, Jarvan, Shen, Vi
- 18: Azir, Lee Sin, Sett, Irelia, Janna, Jarvan, Shen, Xin Zhao
- 18: Azir, Lee Sin, Sett, Irelia, Janna, Shen, Vi, Xin Zhao
- 18: Azir, Lee Sin, Yone, Akali, Fiora, Irelia, Jarvan, Katarina
- 18: Azir, Lee Sin, Yone, Akali, Irelia, Jarvan, Talon, Xin Zhao
- 18: Azir, Lee Sin, Yone, Cassiopeia, Fiora, Irelia, Riven, Shen
- 18: Azir, Lee Sin, Yone, Fiora, Irelia, Jarvan, Xin Zhao, Yasuo
- 18: Azir, Lee Sin, Yone, Irelia, Jarvan, Katarina, Talon, Yasuo
- 18: Azir, Lee Sin, Yone, Irelia, Katarina, Kennen, Talon, Xin Zhao
- 18: Azir, Lee Sin, Fiora, Irelia, Jarvan, Shen, Xin Zhao, Yasuo
- 18: Azir, Lee Sin, Garen, Irelia, Janna, Jarvan, Shen, Yasuo
- 18: Azir, Lee Sin, Irelia, Janna, Jarvan, Katarina, Shen, Yasuo
- 18: Azir, Lee Sin, Irelia, Janna, Jarvan, Kayn, Shen, Xin Zhao
- 18: Azir, Lee Sin, Irelia, Janna, Jarvan, Nidalee, Shen, Yasuo
- 18: Azir, Lee Sin, Irelia, Janna, Jarvan, Shen, Vi, Yasuo
- 18: Azir, Lee Sin, Irelia, Janna, Jarvan, Shen, Xin Zhao, Yasuo
- 18: Azir, Lee Sin, Irelia, Jarvan, Katarina, Shen, Talon, Yasuo
- 18: Azir, Sett, Yone, Fiora, Irelia, Jarvan, Jax, Vi
- 18: Azir, Sett, Yone, Fiora, Irelia, Jarvan, Warwick, Xin Zhao
- 18: Azir, Sett, Yone, Irelia, Janna, Jarvan, Shen, Vi

Without 5 costs:

- 18: Aatrox, Ashe, Hecarim, Irelia, Janna, Maokai, Shen, Warwick
- 18: Aatrox, Ashe, Hecarim, Irelia, Janna, Nunu, Shen, Warwick
- 18: Aphelios, Cassiopeia, Diana, Irelia, Shen, Sylas, Talon, Warwick
- 18: Aphelios, Cassiopeia, Irelia, Lissandra, Morgana, Shen, Sylas, Warwick
- 18: Aphelios, Diana, Irelia, Janna, Katarina, Shen, Sylas, Warwick
- 18: Aphelios, Diana, Irelia, Janna, Pyke, Shen, Sylas, Warwick
- 18: Aphelios, Diana, Irelia, Janna, Shen, Sylas, Talon, Warwick
- 18: Aphelios, Diana, Irelia, Shen, Sylas, Talon, Warwick, Yuumi
- 18: Aphelios, Irelia, Janna, Lissandra, Lux, Shen, Sylas, Warwick
- 18: Aphelios, Irelia, Janna, Lissandra, Morgana, Shen, Sylas, Warwick
- 18: Aphelios, Irelia, Lissandra, Morgana, Shen, Sylas, Warwick, Yuumi
- 18: Ashe, Cassiopeia, Hecarim, Irelia, Maokai, Shen, Thresh, Warwick
- 18: Ashe, Cassiopeia, Hecarim, Irelia, Nunu, Shen, Thresh, Warwick
- 18: Ashe, Garen, Hecarim, Irelia, Janna, Maokai, Shen, Warwick
- 18: Ashe, Garen, Hecarim, Irelia, Janna, Nunu, Shen, Warwick
- 18: Ashe, Hecarim, Irelia, Janna, Maokai, Sejuani, Shen, Warwick
- 18: Ashe, Hecarim, Irelia, Janna, Maokai, Shen, Thresh, Warwick
- 18: Ashe, Hecarim, Irelia, Janna, Maokai, Shen, Warwick, Wukong
- 18: Ashe, Hecarim, Irelia, Janna, Nunu, Sejuani, Shen, Warwick
- 18: Ashe, Hecarim, Irelia, Janna, Nunu, Shen, Thresh, Warwick
- 18: Ashe, Hecarim, Irelia, Janna, Nunu, Shen, Warwick, Wukong
- 18: Ashe, Irelia, Janna, Maokai, Nunu, Shen, Sylas, Warwick
- 18: Ashe, Irelia, Janna, Maokai, Nunu, Shen, Tahm Kench, Warwick
- 18: Ashe, Irelia, Janna, Maokai, Nunu, Shen, Vi, Warwick
- 18: Fiora, Irelia, Kindred, Maokai, Shen, Warwick, Yasuo, Yuumi
- 18: Fiora, Irelia, Kindred, Nunu, Shen, Warwick, Yasuo, Yuumi
- 18: Fiora, Irelia, Kindred, Shen, Sylas, Warwick, Yasuo, Yuumi
- 18: Fiora, Irelia, Kindred, Shen, Tahm Kench, Warwick, Yasuo, Yuumi
- 18: Fiora, Irelia, Kindred, Shen, Vi, Warwick, Yasuo, Yuumi
- 17: Aatrox, Elise, Evelynn, Irelia, Janna, Kayn, Shen, Wukong
- 17: Aatrox, Elise, Irelia, Janna, Kalista, Shen, Wukong, Yasuo
- 17: Aatrox, Elise, Jarvan, Jinx, Katarina, Nidalee, Pyke, Sejuani    

### Most synergies for 7 units

- 17: Azir, Lee Sin, Irelia, Janna, Jarvan, Shen, Xin Zhao
- 17: Azir, Garen, Irelia, Janna, Jarvan, Shen, Wukong
- 17: Azir, Irelia, Janna, Jarvan, Jax, Shen, Xin Zhao
- 17: Azir, Irelia, Janna, Jarvan, Shen, Vi, Warwick
- 16: Azir, Lee Sin, Cassiopeia, Fiora, Irelia, Riven, Shen
- 16: Azir, Sett, Irelia, Janna, Jarvan, Shen, Vi
- 16: Azir, Yone, Cassiopeia, Garen, Jarvan, Shen, Thresh
- 16: Azir, Yone, Cassiopeia, Jarvan, Nidalee, Shen, Vayne
- 16: Azir, Yone, Fiora, Janna, Jarvan, Shen, Xin Zhao
- 16: Azir, Yone, Janna, Jarvan, Katarina, Shen, Talon
- 16: Azir, Cassiopeia, Fiora, Irelia, Jax, Riven, Shen
- 16: Azir, Cassiopeia, Irelia, Lux, Morgana, Riven, Shen
- 16: Azir, Irelia, Janna, Jarvan, Shen, Xin Zhao, Yasuo
- 16: Azir, Irelia, Janna, Riven, Shen, Thresh, Wukong
- 16: Ezreal, Yone, Ashe, Irelia, Maokai, Morgana, Warwick
- 16: Ezreal, Yone, Ashe, Irelia, Morgana, Nunu, Warwick
- 16: Ezreal, Ashe, Irelia, Janna, Maokai, Shen, Warwick
- 16: Ezreal, Ashe, Irelia, Janna, Nunu, Shen, Warwick
- 16: Ezreal, Ashe, Irelia, Maokai, Morgana, Shen, Warwick
- 16: Ezreal, Ashe, Irelia, Morgana, Nunu, Shen, Warwick
- 16: Ezreal, Irelia, Janna, Lux, Maokai, Nunu, Shen
- 16: Lee Sin, Sett, Irelia, Janna, Jax, Shen, Warwick
- 16: Lee Sin, Zilean, Irelia, Kalista, Pyke, Shen, Talon
- 16: Sett, Yone, Aphelios, Irelia, Janna, Shen, Warwick
- 16: Sett, Yone, Ashe, Irelia, Janna, Shen, Warwick
- 16: Sett, Yone, Irelia, Janna, Kindred, Shen, Warwick
- 16: Sett, Yone, Irelia, Janna, Kindred, Warwick, Yuumi
- 16: Sett, Yone, Irelia, Kindred, Shen, Warwick, Yuumi
- 16: Sett, Ahri, Irelia, Janna, Kindred, Shen, Warwick
- 16: Sett, Fiora, Irelia, Kindred, Shen, Warwick, Yuumi
- 16: Sett, Irelia, Janna, Kindred, Shen, Warwick, Yuumi
- 16: Sett, Irelia, Kindred, Morgana, Shen, Warwick, Yuumi

Without 5 costs:

- 16: Aphelios, Diana, Irelia, Janna, Shen, Sylas, Warwick
- 16: Aphelios, Diana, Irelia, Shen, Sylas, Talon, Warwick
- 16: Aphelios, Irelia, Janna, Lissandra, Shen, Sylas, Warwick
- 16: Aphelios, Irelia, Lissandra, Morgana, Shen, Sylas, Warwick
- 16: Ashe, Hecarim, Irelia, Janna, Maokai, Shen, Warwick
- 16: Ashe, Hecarim, Irelia, Janna, Nunu, Shen, Warwick
- 16: Ashe, Irelia, Janna, Lulu, Maokai, Shen, Warwick
- 16: Ashe, Irelia, Janna, Lulu, Nunu, Shen, Warwick
- 16: Ashe, Irelia, Janna, Maokai, Nunu, Shen, Warwick
- 16: Ashe, Irelia, Janna, Maokai, Shen, Veigar, Warwick
- 16: Ashe, Irelia, Janna, Nunu, Shen, Veigar, Warwick
- 16: Cassiopeia, Fiora, Irelia, Shen, Thresh, Wukong, Yasuo
- 16: Fiora, Irelia, Janna, Lux, Morgana, Shen, Yasuo
- 16: Fiora, Irelia, Kindred, Shen, Warwick, Yasuo, Yuumi
- 16: Hecarim, Irelia, Janna, Maokai, Nunu, Shen, Wukong
- 15: Aatrox, Cassiopeia, Fiora, Irelia, Jax, Shen, Thresh
- 15: Aatrox, Cassiopeia, Irelia, Lux, Morgana, Shen, Thresh
- 15: Ahri, Irelia, Janna, Kindred, Maokai, Shen, Warwick
- 15: Ahri, Irelia, Janna, Kindred, Nunu, Shen, Warwick
- 15: Ahri, Irelia, Janna, Kindred, Shen, Sylas, Warwick
- 15: Ahri, Irelia, Janna, Kindred, Shen, Tahm Kench, Warwick
- 15: Ahri, Irelia, Janna, Kindred, Shen, Vi, Warwick
- 15: Akali, Aphelios, Diana, Jax, Sylas, Warwick, Yasuo
- 15: Aphelios, Diana, Fiora, Jax, Sylas, Talon, Warwick
- 15: Aphelios, Diana, Lux, Morgana, Sylas, Talon, Warwick
- 15: Aphelios, Fiora, Irelia, Jax, Kindred, Shen, Yuumi
- 15: Aphelios, Fiora, Jax, Lissandra, Morgana, Sylas, Warwick
- 15: Aphelios, Irelia, Kindred, Lux, Morgana, Shen, Yuumi
- 15: Aphelios, Kayn, Lissandra, Lux, Sylas, Warwick, Zed
- 15: Ashe, Fiora, Irelia, Jax, Kindred, Shen, Yuumi
- 15: Ashe, Hecarim, Kayn, Maokai, Warwick, Wukong, Zed
- 15: Ashe, Hecarim, Kayn, Nunu, Warwick, Wukong, Zed

### Most synergies for 6 units 

- 14: Lee Sin, Yone, Irelia, Janna, Shen, Yasuo
- 14: Sett, Yone, Irelia, Janna, Shen, Warwick
- 14: Sett, Aphelios, Irelia, Janna, Shen, Warwick
- 14: Sett, Ashe, Irelia, Janna, Shen, Warwick
- 14: Sett, Irelia, Janna, Kindred, Shen, Warwick
- 14: Sett, Irelia, Kindred, Shen, Warwick, Yuumi
- 14: Yone, Irelia, Janna, Jax, Shen, Yasuo
- 13: Azir, Lee Sin, Sett, Jarvan, Jax, Vi
- 13: Azir, Lee Sin, Sett, Jarvan, Warwick, Xin Zhao
- 13: Azir, Lee Sin, Yone, Fiora, Irelia, Kennen
- 13: Azir, Lee Sin, Yone, Irelia, Jarvan, Xin Zhao
- 13: Azir, Lee Sin, Akali, Jarvan, Jax, Katarina
- 13: Azir, Lee Sin, Garen, Jarvan, Wukong, Yasuo
- 13: Azir, Lee Sin, Garen, Kennen, Wukong, Xin Zhao
- 13: Azir, Lee Sin, Irelia, Janna, Shen, Yasuo
- 13: Azir, Lee Sin, Irelia, Jarvan, Shen, Xin Zhao
- 13: Azir, Lee Sin, Jarvan, Jax, Xin Zhao, Yasuo
- 13: Azir, Lee Sin, Jarvan, Vi, Warwick, Yasuo
- 13: Azir, Lee Sin, Kennen, Vi, Warwick, Xin Zhao
- 13: Azir, Sett, Yone, Jarvan, Shen, Vi
- 13: Azir, Sett, Garen, Jarvan, Warwick, Wukong
- 13: Azir, Sett, Garen, Riven, Thresh, Vi
- 13: Azir, Sett, Irelia, Janna, Shen, Warwick
- 13: Azir, Sett, Jarvan, Jax, Warwick, Xin Zhao
- 13: Azir, Sett, Jarvan, Kayn, Vi, Zed
- 13: Azir, Sett, Kennen, Vi, Xin Zhao, Yasuo
- 13: Azir, Sett, Nidalee, Riven, Vayne, Vi
- 13: Azir, Yone, Fiora, Irelia, Jarvan, Xin Zhao
- 13: Azir, Yone, Fiora, Irelia, Jax, Kennen
- 13: Azir, Yone, Garen, Irelia, Jarvan, Wukong
- 13: Azir, Yone, Irelia, Jarvan, Jax, Xin Zhao
- 13: Azir, Yone, Irelia, Jarvan, Katarina, Talon

Without 5 costs:

- 13: Ahri, Fiora, Irelia, Jax, Shen, Yuumi
- 13: Ahri, Irelia, Janna, Kindred, Shen, Warwick
- 13: Ahri, Irelia, Lux, Morgana, Shen, Yuumi
- 13: Akali, Garen, Jinx, Katarina, Nidalee, Sejuani
- 13: Akali, Garen, Katarina, Sejuani, Tahm Kench, Vi
- 13: Akali, Jinx, Katarina, Nidalee, Tahm Kench, Vi
- 13: Aphelios, Diana, Irelia, Sylas, Talon, Warwick
- 13: Aphelios, Irelia, Janna, Maokai, Shen, Warwick
- 13: Aphelios, Irelia, Janna, Nunu, Shen, Warwick
- 13: Aphelios, Irelia, Janna, Shen, Sylas, Warwick
- 13: Aphelios, Irelia, Janna, Shen, Tahm Kench, Warwick
- 13: Aphelios, Irelia, Janna, Shen, Vi, Warwick
- 13: Aphelios, Irelia, Lissandra, Morgana, Sylas, Warwick
- 13: Ashe, Irelia, Janna, Maokai, Shen, Warwick
- 13: Ashe, Irelia, Janna, Nunu, Shen, Warwick
- 13: Ashe, Irelia, Janna, Shen, Sylas, Warwick
- 13: Ashe, Irelia, Janna, Shen, Tahm Kench, Warwick
- 13: Ashe, Irelia, Janna, Shen, Vi, Warwick
- 13: Cassiopeia, Fiora, Irelia, Jax, Riven, Shen
- 13: Cassiopeia, Fiora, Irelia, Jax, Shen, Thresh
- 13: Cassiopeia, Fiora, Irelia, Jax, Shen, Vayne
- 13: Cassiopeia, Fiora, Irelia, Shen, Thresh, Wukong
- 13: Cassiopeia, Irelia, Janna, Shen, Thresh, Wukong
- 13: Cassiopeia, Irelia, Lux, Morgana, Riven, Shen
- 13: Cassiopeia, Irelia, Lux, Morgana, Shen, Thresh
- 13: Cassiopeia, Irelia, Lux, Morgana, Shen, Vayne
- 13: Cassiopeia, Irelia, Morgana, Shen, Thresh, Wukong
- 13: Cassiopeia, Irelia, Nami, Shen, Thresh, Wukong
- 13: Cassiopeia, Irelia, Shen, Talon, Thresh, Wukong
- 13: Fiora, Irelia, Janna, Jax, Morgana, Shen
- 13: Fiora, Irelia, Janna, Jax, Nami, Shen
- 13: Fiora, Irelia, Janna, Jax, Shen, Talon

### Most synergies for 5 units

- 12: Lee Sin, Irelia, Janna, Shen, Yasuo
- 12: Sett, Irelia, Janna, Shen, Warwick
- 12: Irelia, Janna, Jax, Shen, Yasuo
- 11: Azir, Yone, Cassiopeia, Riven, Shen
- 11: Ezreal, Ashe, Lux, Maokai, Warwick
- 11: Ezreal, Ashe, Lux, Nunu, Warwick
- 11: Ezreal, Irelia, Janna, Lux, Shen
- 11: Lee Sin, Yone, Fiora, Irelia, Shen
- 11: Lee Sin, Yone, Irelia, Janna, Shen
- 11: Lee Sin, Zilean, Fiora, Irelia, Shen
- 11: Lee Sin, Cassiopeia, Fiora, Irelia, Shen
- 11: Lee Sin, Fiora, Irelia, Janna, Shen
- 11: Lee Sin, Fiora, Irelia, Shen, Yuumi
- 11: Lee Sin, Irelia, Janna, Jax, Shen
- 11: Lee Sin, Irelia, Janna, Kalista, Shen
- 11: Lee Sin, Irelia, Janna, Shen, Xin Zhao
- 11: Yone, Fiora, Irelia, Jax, Shen
- 11: Yone, Fiora, Janna, Shen, Yasuo
- 11: Yone, Irelia, Janna, Jax, Shen
- 11: Yone, Irelia, Janna, Lux, Shen
- 11: Yone, Irelia, Janna, Shen, Warwick
- 11: Yone, Irelia, Janna, Shen, Wukong
- 11: Yone, Irelia, Lux, Morgana, Shen
- 11: Zilean, Fiora, Irelia, Jax, Shen
- 11: Zilean, Irelia, Lux, Morgana, Shen
- 11: Aatrox, Irelia, Janna, Shen, Wukong
- 11: Aphelios, Irelia, Janna, Shen, Warwick
- 11: Aphelios, Lissandra, Lux, Sylas, Warwick
- 11: Ashe, Hecarim, Maokai, Warwick, Wukong
- 11: Ashe, Hecarim, Nunu, Warwick, Wukong
- 11: Ashe, Irelia, Janna, Shen, Warwick
- 11: Cassiopeia, Fiora, Irelia, Jax, Shen 

Without 5 costs:

- 12: Irelia, Janna, Jax, Shen, Yasuo
- 11: Aatrox, Irelia, Janna, Shen, Wukong
- 11: Aphelios, Irelia, Janna, Shen, Warwick
- 11: Aphelios, Lissandra, Lux, Sylas, Warwick
- 11: Ashe, Hecarim, Maokai, Warwick, Wukong
- 11: Ashe, Hecarim, Nunu, Warwick, Wukong
- 11: Ashe, Irelia, Janna, Shen, Warwick
- 11: Cassiopeia, Fiora, Irelia, Jax, Shen
- 11: Cassiopeia, Irelia, Lux, Morgana, Shen
- 11: Cassiopeia, Irelia, Shen, Thresh, Wukong
- 11: Fiora, Irelia, Janna, Jax, Shen
- 11: Fiora, Irelia, Jax, Shen, Yuumi
- 11: Garen, Irelia, Janna, Shen, Wukong
- 11: Hecarim, Irelia, Janna, Shen, Wukong
- 11: Irelia, Janna, Jax, Kalista, Shen
- 11: Irelia, Janna, Jax, Shen, Xin Zhao
- 11: Irelia, Janna, Kindred, Shen, Warwick
- 11: Irelia, Janna, Lissandra, Lux, Shen
- 11: Irelia, Janna, Lux, Morgana, Shen
- 11: Irelia, Janna, Maokai, Shen, Warwick
- 11: Irelia, Janna, Nunu, Shen, Warwick
- 11: Irelia, Janna, Sejuani, Shen, Wukong
- 11: Irelia, Janna, Shen, Sylas, Warwick
- 11: Irelia, Janna, Shen, Tahm Kench, Warwick
- 11: Irelia, Janna, Shen, Thresh, Wukong
- 11: Irelia, Janna, Shen, Vi, Warwick
- 11: Irelia, Kindred, Shen, Warwick, Yuumi
- 11: Irelia, Lux, Morgana, Shen, Yuumi
- 10: Ahri, Ashe, Kindred, Lulu, Veigar
- 10: Akali, Aphelios, Diana, Sylas, Warwick
- 10: Akali, Irelia, Jax, Talon, Yasuo
- 10: Cassiopeia, Fiora, Irelia, Shen, Yasuo