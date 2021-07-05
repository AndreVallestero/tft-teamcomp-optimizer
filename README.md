# tft-teamcomp-optimizer

![unknown](https://user-images.githubusercontent.com/39736205/124393917-e0536b80-dcca-11eb-8aed-ada95a06e3e3.png)


This project generates the most efficient team comps (synergy-wise) within a given amount of units available. It is adaptable for different sets and will work between patches with minor changes.

Here's some sample data from TFT Set 5:

<details>
<summary>With Tier 5</summary>

TODO

</details>

<details>
<summary>Without Tier 5</summary>

<details>
<summary>Most synergies for 8 units</summary>

- 18: Brand, Diana, Kalista, Katarina, Mordekaiser, Morgana, Ryze, Viktor
- 18: Diana, Kalista, Katarina, Mordekaiser, Morgana, Nunu, Ryze, Warwick
- 17: Aatrox, Aphelios, Diana, Ivern, Mordekaiser, Nocturne, Syndra, Varus
- 17: Aatrox, Aphelios, Diana, Karma, Khazix, Mordekaiser, Syndra, Varus
- 17: Aatrox, Ashe, Nautilus, Rell, Sejuani, Taric, Varus, Yasuo
- 17: Aatrox, Brand, Draven, Lux, Nunu, Ryze, Velkoz, Warwick
- 17: Aatrox, Brand, Hecarim, Kalista, Lux, Rell, Ryze, Viktor
- 17: Aatrox, Brand, Kalista, Leona, Lux, Ryze, Thresh, Viktor
- 17: Aatrox, Brand, Kalista, Lux, Ryze, Varus, Vayne, Viktor
- 17: Aatrox, Diana, Ivern, Leona, Mordekaiser, Nocturne, Syndra, Vladimir
- 17: Aatrox, Diana, Ivern, Lux, Mordekaiser, Morgana, Nocturne, Syndra
- 17: Aatrox, Diana, Ivern, Lux, Mordekaiser, Nocturne, Syndra, Vladimir
- 17: Aatrox, Diana, Ivern, Mordekaiser, Nocturne, Rell, Sejuani, Syndra
- 17: Aatrox, Diana, Ivern, Mordekaiser, Nocturne, Rell, Syndra, Vladimir
- 17: Aatrox, Diana, Ivern, Mordekaiser, Nocturne, Syndra, Varus, Vladimir
- 17: Aatrox, Diana, Ivern, Mordekaiser, Nocturne, Syndra, Velkoz, Vladimir
- 17: Aatrox, Diana, Karma, Khazix, Lux, Mordekaiser, Morgana, Syndra
- 17: Aatrox, Diana, Karma, Khazix, Mordekaiser, Rell, Sejuani, Syndra
- 17: Aatrox, Diana, Katarina, Leona, Mordekaiser, Nautilus, Rell, Sejuani
- 17: Aatrox, Diana, Khazix, Leona, Mordekaiser, Nautilus, Rell, Sejuani
- 17: Aatrox, Diana, Khazix, Leona, Nautilus, Rell, Riven, Sejuani
- 17: Aatrox, Diana, Leblanc, Leona, Mordekaiser, Nautilus, Rell, Sejuani
- 17: Aatrox, Diana, Leona, Mordekaiser, Nautilus, Nocturne, Rell, Sejuani
- 17: Aatrox, Hecarim, Kalista, Lux, Nunu, Rell, Ryze, Warwick
- 17: Aatrox, Kalista, Leona, Lux, Nunu, Ryze, Thresh, Warwick
- 17: Aatrox, Kalista, Lux, Nunu, Ryze, Varus, Vayne, Warwick
- 17: Aatrox, Kalista, Lux, Nunu, Ryze, Velkoz, Viktor, Warwick
- 17: Aatrox, Leona, Nautilus, Rell, Riven, Sejuani, Soraka, Vladimir
- 17: Aphelios, Ashe, Diana, Draven, Katarina, Mordekaiser, Taric, Thresh
- 17: Aphelios, Diana, Draven, Gragas, Khazix, Mordekaiser, Vayne, Warwick
- 17: Aphelios, Diana, Gragas, Katarina, Mordekaiser, Riven, Vayne, Warwick
- 17: Aphelios, Diana, Hecarim, Katarina, Mordekaiser, Sejuani, Vayne, Yasuo
</details>

<details>
<summary>Most synergies for 7 units</summary>

- 16: Diana, Ivern, Karma, Mordekaiser, Nocturne, Riven, Vladimir
- 16: Diana, Ivern, Karma, Mordekaiser, Nocturne, Soraka, Yasuo
- 15: Aatrox, Diana, Leona, Mordekaiser, Nautilus, Rell, Sejuani
- 15: Diana, Ivern, Karma, LeeSin, Nidalee, Nocturne, Pantheon
- 15: Diana, Ivern, Karma, LeeSin, Nidalee, Nocturne, Trundle
- 15: Diana, Ivern, LeeSin, Nidalee, Nocturne, Pantheon, Soraka
- 15: Diana, Ivern, LeeSin, Nidalee, Nocturne, Soraka, Trundle
- 15: Diana, Jax, Khazix, Nidalee, Pantheon, Rell, Sejuani
- 15: Diana, Jax, Khazix, Nidalee, Rell, Sejuani, Trundle
- 15: Diana, Khazix, LeeSin, Mordekaiser, Pantheon, Riven, Trundle
- 15: Diana, Khazix, Mordekaiser, Nidalee, Pantheon, Trundle, Yasuo
- 15: Diana, Leblanc, Lissandra, Mordekaiser, Morgana, Riven, Soraka
- 15: Diana, Leblanc, Lissandra, Mordekaiser, Morgana, Vladimir, Yasuo
- 15: Ivern, Leona, Nautilus, Rell, Sejuani, Syndra, Vladimir
- 14: Aatrox, Diana, Ivern, Karma, Khazix, Mordekaiser, Vladimir
- 14: Aatrox, Diana, Ivern, Karma, Mordekaiser, Nocturne, Soraka
- 14: Aatrox, Diana, Ivern, Karma, Mordekaiser, Nocturne, Vladimir
- 14: Aatrox, Diana, Ivern, Karma, Nocturne, Riven, Vladimir
- 14: Aatrox, Diana, Ivern, Karma, Nocturne, Soraka, Yasuo
- 14: Aatrox, Diana, Ivern, Mordekaiser, Nocturne, Syndra, Vladimir
- 14: Aatrox, Draven, Hecarim, Jax, Leona, Rell, Thresh
- 14: Aatrox, Draven, Hecarim, Jax, Lux, Rell, Ryze
- 14: Aatrox, Draven, Hecarim, Jax, Rell, Varus, Vayne
- 14: Aatrox, Draven, Hecarim, Jax, Rell, Velkoz, Viktor
- 14: Aatrox, Draven, Hecarim, Katarina, Leona, Nautilus, Rell
- 14: Aatrox, Draven, Hecarim, Leona, Nautilus, Rell, Ryze
- 14: Aatrox, Draven, Hecarim, Leona, Nautilus, Rell, Thresh
- 14: Aatrox, Draven, Hecarim, Leona, Nautilus, Rell, Vayne
- 14: Aatrox, Draven, Hecarim, Leona, Nautilus, Rell, Viktor
- 14: Aatrox, Draven, Hecarim, Leona, Nautilus, Rell, Warwick
- 14: Aatrox, Draven, Hecarim, Lux, Nautilus, Rell, Ryze
- 14: Aatrox, Draven, Hecarim, Lux, Nautilus, Rell, Thresh
</details>


</details>
