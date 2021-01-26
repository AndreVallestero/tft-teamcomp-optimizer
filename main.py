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



BOSS = 0
CULTIST = 1
DAREDEVIL = 2
DIVINE = 3
DRAGONSOUL = 4
ELDERWOOD = 5
ENLIGHTENED = 6
EXILE = 7
FABLED = 8
FORTUNE = 9
NINJA = 10
SPIRIT = 11
WARLORD = 12

ADEPT = 13
ASSASSIN = 14
BLACKSMITH = 15
BRAWLER = 16
DUELIST = 17
EMPEROR = 18
EXECUTIONER = 19
KEEPER = 20
MAGE = 21
MYSTIC = 22
SHARPSHOOTER = 23
SLAYER = 24
SYPHONER = 25
VANGUARD = 26

UNIT_COMBS = 27

champ_dict = {
    # Comment out 5 costs for more reliable comps
    #'Azir': (WARLORD, KEEPER, EMPEROR),
    #'Lee Sin': (DIVINE, DUELIST),
    #'Samira': (DAREDEVIL, SHARPSHOOTER, SLAYER)
    #'Sett': (BOSS, BRAWLER),
    #'Swain': (DRAGONSOUL, SYPHONER),
    #'Ornn': (BLACKSMITH, ELDERWOOD),
    #'Yone': (EXILE, ADEPT),
    #'Zilean': (CULTIST, MYSTIC),

    'Aatrox': (CULTIST, VANGUARD),
    'Akali': (NINJA, ASSASSIN),
    'Annie': (FORTUNE, MAGE),
    'Aurelion Sol': (DRAGONSOUL, MAGE),
    'Brand': (DRAGONSOUL, MAGE),
    'Braum': (DRAGONSOUL, VANGUARD),
    'Cho Gath': (BRAWLER, FABLED),
    'Darius': (FORTUNE, SLAYER),
    'Diana': (SPIRIT, ASSASSIN),
    'Elise': (CULTIST, KEEPER),
    'Fiora': (ENLIGHTENED, DUELIST),
    'Garen': (WARLORD, VANGUARD),
    'Irelia': (ENLIGHTENED, DIVINE, ADEPT),
    'Janna': (ENLIGHTENED, MYSTIC),
    'Jarvan': (WARLORD, KEEPER),
    'Jax': (DIVINE, DUELIST),
    'Kalista': (CULTIST, DUELIST),
    'Katarina': (WARLORD, FORTUNE, ASSASSIN),
    'Kayle': (DIVINE, EXECUTIONER),
    'Kennen': (NINJA, KEEPER),
    'Kindred': (SPIRIT, EXECUTIONER),
    'Lulu': (ELDERWOOD, MAGE),
    'Maokai': (ELDERWOOD, BRAWLER),
    'Morgana': (ENLIGHTENED, SYPHONER),
    'Nasus': (DIVINE, SYPHONER),
    'Nautilus': (FABLED, VANGUARD),
    'Neeko': (FABLED, MYSTIC),
    'Nidalee': (WARLORD, SHARPSHOOTER),
    'Nunu': (ELDERWOOD, BRAWLER),
    'Olaf': (DRAGONSOUL, SLAYER),
    'Pyke': (CULTIST, ASSASSIN, SLAYER),
    'Rakan': (ELDERWOOD, KEEPER),
    'Sejuani': (FORTUNE, VANGUARD),
    'Shen': (NINJA, ADEPT, MYSTIC),
    'Shyvana': (BRAWLER, DRAGONSOUL),
    'Sivir': (CULTIST, SHARPSHOOTER),
    'Tahm Kench': (FORTUNE, BRAWLER),
    'Talon': (ENLIGHTENED, ASSASSIN),
    'Teemo': (SPIRIT, SHARPSHOOTER),
    'Tristana': (DRAGONSOUL, SHARPSHOOTER),
    'Tryndamere': (DUELIST, WARLORD),
    'Twisted Fate': (CULTIST, MAGE),
    'Veigar': (ELDERWOOD, MAGE),
    'Vi': (WARLORD, BRAWLER),
    'Vladimir': (CULTIST, SYPHONER),
    'Wukong': (DIVINE, VANGUARD),
    'Xayah': (ELDERWOOD, EXECUTIONER, KEEPER),
    'Yasuo': (EXILE, DUELIST),
    'Yuumi': (SPIRIT, MYSTIC),
    'Zed': (NINJA, SLAYER)
}
CHAMP_COUNT = len(champ_dict)
PROGRESS_SEGS = 50
CURR_COMB = MILESTONE = None

def count_synergies(combination):
    global CURR_COMB, MILESTONE

    synergy_sum = 0
    synergy_tally = [0] * UNIT_COMBS
    for champ_name in combination:
        for synergy in champ_dict[champ_name]:
            synergy_tally[synergy] += 1

    CURR_COMB += 1
    if CURR_COMB >= MILESTONE:
        MILESTONE += milestone_step
        print('=', end='', flush=True)

    # Force Teemo
    #if synergy_tally[SHARPSHOOTER] < 4 or synergy_tally[SPIRIT] < 2:
    #    return 0

    # Comment out unique properties for more accurate synergy count
    #synergy_sum += synergy_tally[BOSS]
    #synergy_sum += synergy_tally[DAREDEVIL]
    
    #synergy_sum += synergy_tally[BLACKSMITH]
    #synergy_sum += synergy_tally[EMPEROR]

    synergy_sum += synergy_tally[CULTIST] // 3 * 3
    synergy_sum += synergy_tally[DIVINE] // 2 * 2
    synergy_sum += synergy_tally[DRAGONSOUL] // 3 * 3
    synergy_sum += synergy_tally[ELDERWOOD] // 3 * 3
    synergy_sum += synergy_tally[ENLIGHTENED] // 2 * 2
    synergy_sum += synergy_tally[EXILE]
    synergy_sum += synergy_tally[FABLED] if synergy_tally[FABLED] == 3 else 0
    synergy_sum += synergy_tally[FORTUNE] // 3 * 3
    synergy_sum += synergy_tally[NINJA] if synergy_tally[NINJA] in (1, 4) else 0
    synergy_sum += synergy_tally[SPIRIT] // 2 * 2
    synergy_sum += synergy_tally[WARLORD] // 3 * 3

    synergy_sum += synergy_tally[ADEPT] if synergy_tally[ADEPT] > 1 else 0
    synergy_sum += synergy_tally[ASSASSIN] // 2 * 2
    synergy_sum += synergy_tally[BRAWLER] // 2 * 2
    synergy_sum += synergy_tally[DUELIST] // 2 * 2
    synergy_sum += synergy_tally[EXECUTIONER] if synergy_tally[EXECUTIONER] > 1 else 0
    synergy_sum += synergy_tally[KEEPER] // 2 * 2
    synergy_sum += (synergy_tally[MAGE] - 3) // 2 * 2 + 3 if synergy_tally[MAGE] > 2 else 0
    synergy_sum += synergy_tally[MYSTIC] // 2 * 2
    synergy_sum += synergy_tally[SHARPSHOOTER] // 2 * 2
    synergy_sum += synergy_tally[SLAYER] // 3 * 3
    synergy_sum += synergy_tally[SYPHONER] // 2 * 2
    synergy_sum += synergy_tally[VANGUARD] // 2 * 2

    return synergy_sum

for i in range(3, 10):
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

input('\nComps generated, press enter to exit')
