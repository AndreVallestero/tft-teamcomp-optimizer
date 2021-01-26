# tft-teamcomp-optimizer

This project generates the most efficient team comps (synergy-wise) within a given amount of units available. It is adaptable different sets and will work between patches with minor changes. It takes ~24 hours to run as it has to generate and calculate almost 60 billion combinations, so be ready to leave your computer on over night.

It does not take into account in-game effects such as `chosen one` from TFT Set 4 or spatula items.

Here's some sample data from TFT Set 4.0:

<details>
<summary>With Tier 5</summary>

<details>
<summary>### Most synergies for 9 units</summary>
</details>

<details>
<summary>### Most synergies for 8 units</summary>
</details>

<details>
<summary>### Most synergies for 7 units</summary>
</details>

<details>
<summary>### Most synergies for 6 units</summary>
</details>

<details>
<summary>### Most synergies for 5 units</summary>
</details>

<details>
<summary>### Most synergies for 4 units</summary>
</details>

<details>
<summary>### Most synergies for 3 units</summary>
</details>

</details>

<details>
<summary>Without Tier 5</summary>

<details>
<summary>### Most synergies for 9 units</summary>
</details>

<details>
<summary>### Most synergies for 8 units</summary>
- 18: Cho Gath, Irelia, Janna, Kayle, Maokai, Rakan, Shen, Xayah
- 18: Cho Gath, Irelia, Janna, Kayle, Nunu, Rakan, Shen, Xayah
- 18: Elise, Irelia, Janna, Kayle, Maokai, Nunu, Shen, Xayah
- 18: Irelia, Janna, Jarvan, Kayle, Maokai, Nunu, Shen, Xayah
- 18: Irelia, Janna, Kayle, Maokai, Nunu, Rakan, Shen, Xayah
- 18: Irelia, Janna, Kayle, Maokai, Rakan, Shen, Shyvana, Xayah
- 18: Irelia, Janna, Kayle, Maokai, Rakan, Shen, Tahm Kench, Xayah
- 18: Irelia, Janna, Kayle, Maokai, Rakan, Shen, Vi, Xayah
- 18: Irelia, Janna, Kayle, Nunu, Rakan, Shen, Shyvana, Xayah
- 18: Irelia, Janna, Kayle, Nunu, Rakan, Shen, Tahm Kench, Xayah
- 18: Irelia, Janna, Kayle, Nunu, Rakan, Shen, Vi, Xayah
- 17: Aatrox, Darius, Kalista, Katarina, Pyke, Sejuani, Yasuo, Zed
- 17: Aatrox, Elise, Irelia, Janna, Kalista, Shen, Wukong, Yasuo
- 17: Aatrox, Elise, Jarvan, Katarina, Pyke, Sejuani, Tahm Kench, Vi
- 17: Aatrox, Elise, Kayle, Lulu, Twisted Fate, Veigar, Wukong, Xayah
- 17: Aatrox, Irelia, Janna, Kalista, Nasus, Shen, Vladimir, Yasuo
- 17: Aatrox, Irelia, Janna, Kalista, Pyke, Shen, Wukong, Yasuo
- 17: Aatrox, Irelia, Janna, Kalista, Shen, Sivir, Wukong, Yasuo
- 17: Aatrox, Irelia, Janna, Kalista, Shen, Twisted Fate, Wukong, Yasuo
- 17: Aatrox, Irelia, Janna, Kalista, Shen, Vladimir, Wukong, Yasuo
- 17: Aatrox, Irelia, Kalista, Morgana, Shen, Vladimir, Wukong, Yasuo
- 17: Aatrox, Irelia, Kalista, Pyke, Shen, Talon, Wukong, Yasuo
- 17: Aatrox, Kalista, Katarina, Pyke, Sejuani, Tahm Kench, Tryndamere, Vi
- 17: Aatrox, Katarina, Nidalee, Pyke, Sejuani, Sivir, Tahm Kench, Vi
- 17: Akali, Jax, Katarina, Sejuani, Tahm Kench, Tryndamere, Vi, Wukong
- 17: Annie, Aurelion Sol, Brand, Braum, Irelia, Janna, Shen, Wukong
- 17: Annie, Irelia, Janna, Kayle, Lulu, Shen, Veigar, Xayah
</details>

<details>
<summary>### Most synergies for 7 units</summary>
- 16: Diana, Irelia, Jax, Shen, Talon, Yasuo, Yuumi
- 16: Fiora, Irelia, Janna, Morgana, Nasus, Shen, Yasuo
- 16: Fiora, Irelia, Kayle, Kindred, Shen, Yasuo, Yuumi
- 16: Irelia, Janna, Kayle, Lulu, Rakan, Shen, Xayah
- 16: Irelia, Janna, Kayle, Maokai, Nunu, Shen, Xayah
- 16: Irelia, Janna, Kayle, Maokai, Rakan, Shen, Xayah
- 16: Irelia, Janna, Kayle, Nunu, Rakan, Shen, Xayah
- 16: Irelia, Janna, Kayle, Rakan, Shen, Veigar, Xayah
- 15: Aatrox, Diana, Irelia, Shen, Talon, Wukong, Yuumi
- 15: Braum, Diana, Irelia, Shen, Talon, Wukong, Yuumi
- 15: Diana, Fiora, Irelia, Janna, Jax, Shen, Talon
- 15: Diana, Fiora, Irelia, Jax, Katarina, Shen, Yuumi
- 15: Diana, Fiora, Irelia, Jax, Pyke, Shen, Yuumi
- 15: Diana, Fiora, Irelia, Jax, Shen, Talon, Yuumi
- 15: Diana, Garen, Irelia, Shen, Talon, Wukong, Yuumi
- 15: Diana, Irelia, Janna, Katarina, Kayle, Kindred, Shen
- 15: Diana, Irelia, Janna, Kayle, Kindred, Pyke, Shen
- 15: Diana, Irelia, Janna, Kayle, Kindred, Shen, Talon
- 15: Diana, Irelia, Janna, Morgana, Nasus, Shen, Talon
- 15: Diana, Irelia, Jax, Kalista, Shen, Talon, Yuumi
- 15: Diana, Irelia, Jax, Shen, Talon, Tryndamere, Yuumi
- 15: Diana, Irelia, Katarina, Morgana, Nasus, Shen, Yuumi
- 15: Diana, Irelia, Kayle, Kindred, Neeko, Shen, Talon
- 15: Diana, Irelia, Kayle, Kindred, Shen, Talon, Yuumi
- 15: Diana, Irelia, Kayle, Shen, Talon, Xayah, Yuumi
- 15: Diana, Irelia, Morgana, Nasus, Pyke, Shen, Yuumi
- 15: Diana, Irelia, Morgana, Nasus, Shen, Talon, Yuumi
</details>

<details>
<summary>### Most synergies for 6 units</summary>
- 13: Akali, Garen, Katarina, Sejuani, Tahm Kench, Vi
- 13: Diana, Fiora, Irelia, Jax, Shen, Yuumi
- 13: Diana, Irelia, Janna, Kayle, Kindred, Shen
- 13: Diana, Irelia, Jax, Shen, Talon, Yuumi
- 13: Diana, Irelia, Kayle, Kindred, Shen, Talon
- 13: Diana, Irelia, Kayle, Shen, Talon, Yuumi
- 13: Diana, Irelia, Morgana, Nasus, Shen, Yuumi
- 13: Diana, Irelia, Nasus, Shen, Talon, Yuumi
- 13: Diana, Irelia, Shen, Talon, Wukong, Yuumi
- 13: Elise, Irelia, Janna, Kayle, Shen, Xayah
- 13: Fiora, Irelia, Janna, Jax, Morgana, Shen
- 13: Fiora, Irelia, Janna, Jax, Shen, Talon
- 13: Fiora, Irelia, Janna, Morgana, Nasus, Shen
- 13: Fiora, Irelia, Jax, Kindred, Shen, Yuumi
- 13: Fiora, Irelia, Jax, Shen, Teemo, Yuumi
- 13: Fiora, Irelia, Kayle, Kindred, Shen, Yuumi
- 13: Irelia, Janna, Jarvan, Kayle, Shen, Xayah
- 13: Irelia, Janna, Kayle, Kindred, Shen, Teemo
- 13: Irelia, Janna, Kayle, Kindred, Shen, Yuumi
- 13: Irelia, Janna, Kayle, Rakan, Shen, Xayah
- 13: Irelia, Janna, Morgana, Nasus, Shen, Talon
- 13: Irelia, Kayle, Kindred, Morgana, Shen, Yuumi
- 13: Irelia, Kayle, Kindred, Shen, Talon, Yuumi
- 13: Irelia, Kindred, Morgana, Nasus, Shen, Yuumi
- 13: Irelia, Morgana, Nasus, Shen, Teemo, Yuumi
- 12: Aatrox, Akali, Jax, Kalista, Pyke, Wukong
- 12: Aatrox, Akali, Nasus, Pyke, Vladimir, Wukong
</details>

<details>
<summary>### Most synergies for 5 units</summary>
- 12: Irelia, Janna, Jax, Shen, Yasuo
- 11: Aatrox, Irelia, Janna, Shen, Wukong
- 11: Braum, Irelia, Janna, Shen, Wukong
- 11: Diana, Irelia, Shen, Talon, Yuumi
- 11: Fiora, Irelia, Janna, Jax, Shen
- 11: Fiora, Irelia, Jax, Neeko, Shen
- 11: Fiora, Irelia, Jax, Shen, Yuumi
- 11: Garen, Irelia, Janna, Shen, Wukong
- 11: Irelia, Janna, Jax, Kalista, Shen
- 11: Irelia, Janna, Jax, Shen, Tryndamere
- 11: Irelia, Janna, Kayle, Kindred, Shen
- 11: Irelia, Janna, Kayle, Shen, Xayah
- 11: Irelia, Janna, Morgana, Nasus, Shen
- 11: Irelia, Janna, Nasus, Shen, Vladimir
- 11: Irelia, Janna, Nautilus, Shen, Wukong
- 11: Irelia, Janna, Sejuani, Shen, Wukong
- 11: Irelia, Kayle, Kindred, Shen, Yuumi
- 11: Irelia, Morgana, Nasus, Neeko, Shen
- 11: Irelia, Morgana, Nasus, Shen, Yuumi
- 10: Akali, Irelia, Jax, Talon, Yasuo
- 10: Diana, Irelia, Kayle, Kindred, Talon
- 10: Fiora, Irelia, Janna, Shen, Yasuo
- 10: Fiora, Irelia, Jax, Shen, Yasuo
- 10: Fiora, Irelia, Kayle, Shen, Yasuo
- 10: Fiora, Irelia, Nasus, Shen, Yasuo
- 10: Fiora, Irelia, Neeko, Shen, Yasuo
- 10: Fiora, Irelia, Shen, Wukong, Yasuo
</details>

<details>
<summary>### Most synergies for 4 units</summary>
- 9: Fiora, Irelia, Jax, Shen
- 9: Irelia, Janna, Jax, Shen
- 9: Irelia, Janna, Kayle, Shen
- 9: Irelia, Janna, Nasus, Shen
- 9: Irelia, Janna, Shen, Wukong
- 9: Irelia, Morgana, Nasus, Shen
- 8: Akali, Fiora, Talon, Yasuo
- 8: Diana, Janna, Talon, Yuumi
- 8: Fiora, Irelia, Shen, Yasuo
- 8: Fiora, Janna, Shen, Yasuo
- 8: Fiora, Jax, Morgana, Nasus
- 8: Irelia, Janna, Shen, Yasuo
- 8: Irelia, Jax, Shen, Yasuo
- 8: Kennen, Maokai, Nunu, Rakan
- 8: Kennen, Maokai, Nunu, Xayah
- 7: Aatrox, Irelia, Janna, Shen
- 7: Aatrox, Irelia, Shen, Wukong
- 7: Aatrox, Jax, Wukong, Yasuo
- 7: Akali, Diana, Janna, Yuumi
- 7: Akali, Diana, Kayle, Kindred
- 7: Akali, Diana, Kindred, Xayah
- 7: Akali, Diana, Neeko, Yuumi
- 7: Akali, Diana, Nidalee, Teemo
- 7: Akali, Diana, Sivir, Teemo
- 7: Akali, Diana, Teemo, Tristana
- 7: Akali, Fiora, Irelia, Jax
- 7: Akali, Fiora, Jax, Talon
</details>

<details>
<summary>### Most synergies for 3 units</summary>
- 7: Irelia, Janna, Shen
- 6: Fiora, Irelia, Jax
- 6: Irelia, Morgana, Nasus
- 5: Akali, Diana, Kindred
- 5: Akali, Diana, Teemo
- 5: Akali, Diana, Yuumi
- 5: Akali, Fiora, Talon
- 5: Akali, Irelia, Talon
- 5: Akali, Janna, Talon
- 5: Akali, Morgana, Talon
- 5: Diana, Shen, Yuumi
- 5: Fiora, Irelia, Shen
- 5: Fiora, Irelia, Yasuo
- 5: Fiora, Janna, Shen
- 5: Fiora, Janna, Yasuo
- 5: Fiora, Morgana, Yasuo
- 5: Fiora, Talon, Yasuo
- 5: Irelia, Jax, Shen
- 5: Irelia, Jax, Yasuo
- 5: Irelia, Kayle, Shen
- 5: Irelia, Morgana, Shen
- 5: Irelia, Nasus, Shen
- 5: Irelia, Neeko, Shen
- 5: Irelia, Shen, Talon
- 5: Irelia, Shen, Wukong
- 5: Irelia, Shen, Yuumi
- 5: Janna, Morgana, Shen
</details>

</details>