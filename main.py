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

ABOMINATION = 0
COVEN = 1
DAWNBRINGER = 2
DRACONIC = 3
DRAGONSLAYER = 4
ETERNAL = 5
FORGOTTEN = 6
HELLION = 7
IRONCLAD = 8
NIGHTBRINGER = 9
REDEEMED = 10
REVENANT = 11
VERDANT = 12

ASSASSIN = 13
BRAWLER = 14
CARETAKER = 15
CAVALIER = 16
CRUEL = 17
GODKING = 18
INVOKER = 19
KNIGHT = 20
LEGIONNAIRE = 21
MYSTIC = 22
RANGER = 23
RENEWER = 24
SKIRMISHER = 25
SPELLWEAVER = 26

UNIT_COMBS = 27

champ_dict = {
    # Comment out teir 5 for more reliable comps
    #'Darius': (NIGHTBRINGER, KNIGHT, GODKING),
    #'Garen': (DAWNBRINGER, KNIGHT, GODKING),
    #'Heimerdinger': (DRACONIC, CARETAKER, RENEWER),
    #'Kayle': (REDEEMED, VERDANT, LEGIONNAIRE),
    #'Kindred': (ETERNAL, MYSTIC, RANGER),
    #'Teemo': (HELLION, CRUEL, INVOKER),
    #'Viego': (FORGOTTEN, ASSASSIN, SKIRMISHER),
    #'Volibear': (REVENANT, BRAWLER),

    'Aatrox': (REDEEMED, LEGIONNAIRE),
    'Aphelios': (NIGHTBRINGER, RANGER),
    'Ashe': (VERDANT, DRACONIC, RANGER),
    'Brand': (ABOMINATION, SPELLWEAVER),
    'Diana': (NIGHTBRINGER, DRAGONSLAYER, ASSASSIN),
    'Draven': (FORGOTTEN, LEGIONNAIRE),
    'Gragas': (DAWNBRINGER, BRAWLER),
    'Hecarim': (FORGOTTEN, CAVALIER),
    'Ivern': (REVENANT, INVOKER, RENEWER),
    'Jax': (IRONCLAD, SKIRMISHER),
    'Kalista': (ABOMINATION, LEGIONNAIRE),
    'Karma': (DAWNBRINGER, INVOKER),
    'Katarina': (FORGOTTEN, ASSASSIN),
    'Kennen': (HELLION, SKIRMISHER),
    'Khazix': (DAWNBRINGER, ASSASSIN),
    'Kled': (HELLION, CAVALIER),
    'Leblanc': (COVEN, ASSASSIN),
    'LeeSin': (NIGHTBRINGER, SKIRMISHER),
    'Leona': (REDEEMED, KNIGHT),
    'Lissandra': (COVEN, RENEWER),
    'Lulu': (HELLION, MYSTIC),
    'Lux': (REDEEMED, MYSTIC),
    'Mordekaiser': (DRAGONSLAYER, LEGIONNAIRE),
    'Morgana': (COVEN, NIGHTBRINGER, MYSTIC),
    'Nautilus': (IRONCLAD, KNIGHT),
    'Nidalee': (DAWNBRINGER, SKIRMISHER),
    'Nocturne': (REVENANT, ASSASSIN),
    'Nunu': (ABOMINATION, BRAWLER),
    'Pantheon': (DRAGONSLAYER, SKIRMISHER),
    'Poppy': (HELLION, KNIGHT),
    'Rell': (REDEEMED, IRONCLAD, CAVALIER),
    'Riven': (DAWNBRINGER, LEGIONNAIRE),
    'Ryze': (ABOMINATION, FORGOTTEN, MYSTIC),
    'Sejuani': (NIGHTBRINGER, CAVALIER),
    'Sett': (DRACONIC, BRAWLER),
    'Soraka': (DAWNBRINGER, RENEWER),
    'Syndra': (REDEEMED, INVOKER),
    'Taric': (VERDANT, KNIGHT),
    'Thresh': (FORGOTTEN, KNIGHT),
    'Trundle': (DRAGONSLAYER, SKIRMISHER),
    'Udyr': (DRACONIC, SKIRMISHER),
    'Varus': (REDEEMED, RANGER),
    'Vayne': (FORGOTTEN, RANGER),
    'Velkoz': (REDEEMED, SPELLWEAVER),
    'Viktor': (FORGOTTEN, SPELLWEAVER),
    'Vladimir': (NIGHTBRINGER, RENEWER),
    'Warwick': (FORGOTTEN, BRAWLER),
    'Yasuo': (NIGHTBRINGER, LEGIONNAIRE),
    'Ziggs': (HELLION, SPELLWEAVER),
    'Zyra': (DRACONIC, SPELLWEAVER)
}
CHAMP_COUNT = len(champ_dict)
PROGRESS_SEGS = 50
CURR_COMB = MILESTONE = None

def count_synergies(combination):
    global CURR_COMB, MILESTONE
    CURR_COMB += 1
    if CURR_COMB >= MILESTONE:
        MILESTONE += milestone_step
        print('=', end='', flush=True)

    # Force Kalista Sivir
    #if 'Kalista' not in combination or 'Sivir' not in combination:
    #    return 0
    
    synergy_tally = [0] * UNIT_COMBS
    for champ_name in combination:
        for synergy in champ_dict[champ_name]:
            synergy_tally[synergy] += 1

    # Force assassin 4
    #if synergy_tally[ASSASSIN] != 4:
    #    return 0

    # Branchless calculations
    return \
        synergy_tally[ABOMINATION] * (synergy_tally[ABOMINATION] >= 3) + \
        3 * (synergy_tally[COVEN] >= 3) + \
        synergy_tally[DAWNBRINGER] // 2 * 2 + \
        ((synergy_tally[DRACONIC] - 1) // 2 * 2 + 1) * (synergy_tally[DRACONIC] >= 3) + \
        synergy_tally[DRAGONSLAYER] // 2 * 2 + \
        synergy_tally[FORGOTTEN] // 3 * 3 + \
        ((synergy_tally[HELLION] - 1) // 2 * 2 + 1) * (synergy_tally[HELLION] >= 3) + \
        synergy_tally[IRONCLAD] * (synergy_tally[IRONCLAD] >= 2) + \
        synergy_tally[NIGHTBRINGER] // 2 * 2 + \
        synergy_tally[REDEEMED] // 3 * 3 + \
        synergy_tally[REVENANT] * (synergy_tally[REVENANT] >= 2) + \
        synergy_tally[VERDANT] * (synergy_tally[VERDANT] >= 2) + \
        synergy_tally[ASSASSIN] // 2 * 2 + \
        synergy_tally[BRAWLER] // 2 * 2 + \
        synergy_tally[CAVALIER] * (synergy_tally[CAVALIER] >= 2) + \
        synergy_tally[INVOKER] // 2 * 2 + \
        synergy_tally[KNIGHT] // 2 * 2 + \
        synergy_tally[LEGIONNAIRE] // 2 * 2 + \
        synergy_tally[MYSTIC] // 2 * 2 + \
        synergy_tally[RANGER] // 2 * 2 + \
        synergy_tally[RENEWER] // 2 * 2 + \
        synergy_tally[SKIRMISHER] // 3 * 3 + \
        synergy_tally[SPELLWEAVER] // 2 * 2        
        #synergy_tally[ETERNAL] + \ 
        #synergy_tally[CARETAKER] + \
        #synergy_tally[CRUEL] + \
        #synergy_tally[GODKING] >= 1 + \
        # Comment out unique properties for more accurate synergy count

for i in range(3, 9):
    milestone_step = math.factorial(CHAMP_COUNT) // (math.factorial(i) * math.factorial(CHAMP_COUNT - i) * PROGRESS_SEGS)
    MILESTONE = milestone_step
    CURR_COMB = 0

    print(f'\nCalculating synergies for {i} units\n+{"-"*PROGRESS_SEGS}+\n|', end='')
    best = heapq.nlargest(
        32,
        itertools.combinations(champ_dict.keys(), i),
        key=count_synergies
    )
    print('|')
    print(f'Most synergies for {i} units:')
    for comp in best:
        comp_str = str(comp)[1:-1].replace("'", "")
        print(f'\t- {count_synergies(comp)}: {comp_str }')

input('\nComps generated, press enter to exit')
