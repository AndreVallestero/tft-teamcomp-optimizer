# Number of combinations
# 5      4187106
# 6     36288252
# 7    264385836
# 8   1652411475
# 9   8996462475
# 10 43183019880

# Use in combination with winrate info for best results
# https://app.powerbi.com/view?r=eyJrIjoiMTg3MWYwNTctYjc4Ni00ODM4LWIzZWItZmYzYmI0MzJjZGUxIiwidCI6ImY1MGNmY2Y1LTRjZGUtNDcxYS05NmQxLWJjYzAxY2FkNmFmMiIsImMiOjF9&pageName=ReportSectionad853af7c37708d627b0

import itertools
import heapq
import math

UNIT_COMBS = 32

CULTIST = 0
DIVINE = 1
DUSK = 2
ELDERWOOD = 3
ENLIGHTENED = 4
EXILE = 5
FORTUNE = 6
MOONLIGHT = 7
NINJA = 8
SPIRIT = 9
BOSS = 10
TORMENTED = 11
WARLORD = 12
ADEPT = 13
ASSASSIN = 14
BRAWLER = 15
DAZZLER = 16
DUELIST = 17
EMPEROR = 18
HUNTER = 19
KEEPER = 20
MAGE = 21
MYSTIC = 22
SHADE = 23
SHARPSHOOTER = 24
VANGUARD = 25

champ_dict = {
    # Comment out 5 costs for more reliable comps
    'Azir': (WARLORD, KEEPER, EMPEROR),
    'Ezreal': (ELDERWOOD, DAZZLER),
    'Kayn': (TORMENTED, SHADE),
    'Lee Sin': (DIVINE, DUELIST),
    'Lillia': (DUSK, MAGE),
    'Sett': (BOSS, BRAWLER),
    'Yone': (EXILE, ADEPT),
    'Zilean': (CULTIST, MYSTIC),

    'Aatrox': (CULTIST, VANGUARD),
    'Ahri': (SPIRIT, MAGE),
    'Akali': (NINJA, ASSASSIN),
    'Annie': (FORTUNE, MAGE),
    'Aphelios': (MOONLIGHT, HUNTER),
    'Ashe': (ELDERWOOD, HUNTER),
    'Cassiopeia': (DUSK, MYSTIC),
    'Diana': (MOONLIGHT, ASSASSIN),
    'Elise': (CULTIST, KEEPER),
    'Evelynn': (CULTIST, SHADE),
    'Fiora': (ENLIGHTENED, DUELIST),
    'Garen': (WARLORD, VANGUARD),
    'Hecarim': (ELDERWOOD, VANGUARD),
    'Irelia': (ENLIGHTENED, DIVINE, ADEPT),
    'Janna': (ENLIGHTENED, MYSTIC),
    'Jarvan': (WARLORD, KEEPER),
    'Jax': (DIVINE, DUELIST),
    'Jhin': (CULTIST, SHARPSHOOTER),
    'Jinx': (FORTUNE, SHARPSHOOTER),
    'Kalista': (CULTIST, DUELIST),
    'Katarina': (WARLORD, FORTUNE, ASSASSIN),
    'Kennen': (NINJA, KEEPER),
    'Kindred': (SPIRIT, HUNTER),
    'Lissandra': (MOONLIGHT, DAZZLER),
    'Lulu': (ELDERWOOD, MAGE),
    'Lux': (DIVINE, DAZZLER),
    'Maokai': (ELDERWOOD, BRAWLER),
    'Morgana': (ENLIGHTENED, DAZZLER),
    'Nami': (ENLIGHTENED, MAGE),
    'Nidalee': (WARLORD, SHARPSHOOTER),
    'Nunu': (ELDERWOOD, BRAWLER),
    'Pyke': (CULTIST, ASSASSIN),
    'Riven': (DUSK, KEEPER),
    'Sejuani': (FORTUNE, VANGUARD),
    'Shen': (NINJA, ADEPT, MYSTIC),
    'Sylas': (MOONLIGHT, BRAWLER),
    'Tahm Kench': (FORTUNE, BRAWLER),
    'Talon': (ENLIGHTENED, ASSASSIN),
    'Thresh': (DUSK, VANGUARD),
    'Twisted Fate': (CULTIST, MAGE),
    'Vayne': (DUSK, SHARPSHOOTER),
    'Veigar': (ELDERWOOD, MAGE),
    'Vi': (WARLORD, BRAWLER),
    'Warwick': (DIVINE, BRAWLER, HUNTER),
    'Wukong': (DIVINE, VANGUARD),
    'Xin Zhao': (WARLORD, DUELIST),
    'Yasuo': (EXILE, DUELIST),
    'Yuumi': (SPIRIT, MYSTIC),
    'Zed': (NINJA, SHADE)
}
CHAMP_COUNT = len(champ_dict)
PROGRESS_SEGS = 50
CURR_COMB = MILESTONE = None

def count_synergies(combination):
    global CURR_COMB, MILESTONE

    synergy_sum = 0
    synergy_tally = [0] * 26
    for champ_name in combination:
        for synergy in champ_dict[champ_name]:
            synergy_tally[synergy] += 1

    # Force mage
    #if synergy_tally[MAGE] < 3:
    #    return 0

    synergy_sum += synergy_tally[CULTIST] // 3 * 3
    synergy_sum += synergy_tally[DIVINE] // 2 * 2
    synergy_sum += synergy_tally[DUSK] // 2 * 2
    synergy_sum += synergy_tally[ELDERWOOD] // 3 * 3
    synergy_sum += synergy_tally[ENLIGHTENED] // 2 * 2
    synergy_sum += synergy_tally[EXILE]
    synergy_sum += synergy_tally[FORTUNE] // 3 * 3
    synergy_sum += synergy_tally[MOONLIGHT] // 3 * 3
    synergy_sum += synergy_tally[NINJA] if synergy_tally[NINJA] in (1, 4) else 0
    synergy_sum += synergy_tally[SPIRIT] // 2 * 2
    synergy_sum += synergy_tally[BOSS]
    synergy_sum += synergy_tally[TORMENTED]
    synergy_sum += synergy_tally[WARLORD] // 3 * 3

    synergy_sum += synergy_tally[ADEPT] if synergy_tally[ADEPT] > 1 else 0
    synergy_sum += synergy_tally[ASSASSIN] // 2 * 2
    synergy_sum += synergy_tally[BRAWLER] // 2 * 2
    synergy_sum += synergy_tally[DAZZLER] // 2 * 2
    synergy_sum += synergy_tally[DUELIST] // 2 * 2
    synergy_sum += synergy_tally[EMPEROR]
    synergy_sum += synergy_tally[HUNTER] if synergy_tally[HUNTER] > 1 else 0
    synergy_sum += synergy_tally[KEEPER] // 2 * 2
    synergy_sum += synergy_tally[MAGE] // 3 * 3
    synergy_sum += synergy_tally[MYSTIC] // 2 * 2
    synergy_sum += synergy_tally[SHADE] if synergy_tally[SHADE] > 1 else 0
    synergy_sum += synergy_tally[SHARPSHOOTER] // 2 * 2
    synergy_sum += synergy_tally[VANGUARD] // 2 * 2

    CURR_COMB += 1
    if CURR_COMB >= MILESTONE:
        MILESTONE += milestone_step
        print('=', end='', flush=True)

    return synergy_sum

for i in range(7, 11):
    milestone_step = math.factorial(CHAMP_COUNT) // (math.factorial(i) * math.factorial(CHAMP_COUNT - i) * PROGRESS_SEGS)
    MILESTONE = milestone_step
    CURR_COMB = 0

    print(f'\nCalculating synergies for {i} units\n+{"-"*PROGRESS_SEGS}+\n|', end='')
    best = heapq.nlargest(
        UNIT_COMBS,
        itertools.combinations(champ_dict.keys(), i),
        key=count_synergies
    )
    print('|')
 
    print(f'Most synergies for {i} units:')
    for comp in best:
        comp_str = str(comp)[1:-1].replace("'", "")
        print(f'\t- {count_synergies(comp)}: {comp_str }')
