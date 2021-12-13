// THIS IS EVIL PREMATURE OPTIMIZATION

// 2 approaches
// - 1 vertical bit count by using unique bits for each u32
// - 2 array of u32s. Each u32 is a u32. Once 0 it hit, entry is done.

const ABOMINATION_ID: u32 = 0;
const ABOMINATION: u32 = 1 << ABOMINATION_ID;
const COVEN_ID: u32 = 1;
const COVEN: u32 = 1 << COVEN_ID;
const DAWNBRINGER_ID: u32 = 2;
const DAWNBRINGER: u32 = 1 << DAWNBRINGER_ID;
const DRACONIC_ID: u32 = 3;
const DRACONIC: u32 = 1 << DRACONIC_ID;
const DRAGONSLAYER_ID: u32 = 4;
const DRAGONSLAYER: u32 = 1 << DRAGONSLAYER_ID;
const ETERNAL_ID: u32 = 5;
const ETERNAL: u32 = 1 << ETERNAL_ID;
const FORGOTTEN_ID: u32 = 6;
const FORGOTTEN: u32 = 1 << FORGOTTEN_ID;
const HELLION_ID: u32 = 7;
const HELLION: u32 = 1 << HELLION_ID;
const IRONCLAD_ID: u32 = 8;
const IRONCLAD: u32 = 1 << IRONCLAD_ID;
const NIGHTBRINGER_ID: u32 = 9;
const NIGHTBRINGER: u32 = 1 << NIGHTBRINGER_ID;
const REDEEMED_ID: u32 = 10;
const REDEEMED: u32 = 1 << REDEEMED_ID;
const REVENANT_ID: u32 = 11;
const REVENANT: u32 = 1 << REVENANT_ID;
const VERDANT_ID: u32 = 12;
const VERDANT: u32 = 1 << VERDANT_ID;

const ASSASSIN_ID: u32 = 13;
const ASSASSIN: u32 = 1 << ASSASSIN_ID;
const BRAWLER_ID: u32 = 14;
const BRAWLER: u32 = 1 << BRAWLER_ID;
const CARETAKER_ID: u32 = 15;
const CARETAKER: u32 = 1 << CARETAKER_ID;
const CAVALIER_ID: u32 = 16;
const CAVALIER: u32 = 1 << CAVALIER_ID;
const CRUEL_ID: u32 = 17;
const CRUEL: u32 = 1 << CRUEL_ID;
const GODKING_ID: u32 = 18;
const GODKING: u32 = 1 << GODKING_ID;
const INVOKER_ID: u32 = 19;
const INVOKER: u32 = 1 << INVOKER_ID;
const KNIGHT_ID: u32 = 20;
const KNIGHT: u32 = 1 << KNIGHT_ID;
const LEGIONNAIRE_ID: u32 = 21;
const LEGIONNAIRE: u32 = 1 << LEGIONNAIRE_ID;
const MYSTIC_ID: u32 = 22;
const MYSTIC: u32 = 1 << MYSTIC_ID;
const RANGER_ID: u32 = 23;
const RANGER: u32 = 1 << RANGER_ID;
const RENEWER_ID: u32 = 24;
const RENEWER: u32 = 1 << RENEWER_ID;
const SKIRMISHER_ID: u32 = 25;
const SKIRMISHER: u32 = 1 << SKIRMISHER_ID;
const SPELLWEAVER_ID: u32 = 26;
const SPELLWEAVER: u32 = 1 << SPELLWEAVER_ID;

const SYNERGIES: u32 = 27; // 1 bit for each unit type

const AATROX: u32 = REDEEMED | LEGIONNAIRE;
fn main() {
    println!("Hello, world!");
}


/*
#include <stdio.h>
#include <stdlib.h>

#define MAX_BIT 27
#define DATA_SIZE 12345

int *countBits(int *dataPtr, int *dataEndPtr);

int main()
{
   int data[DATA_SIZE];
   int *countResults;
   int i;
   
   /* dummy data for testing */
   for(i = 0; i < DATA_SIZE; i++)
   {
      data[i] = i;
   }
   
   countResults = countBits(data, data + DATA_SIZE);
   
   for(i = 1; i <= MAX_BIT; i++)
   {
      printf("Bit %2d: %d\n", i, countResults[i]);
   }
   
   return 0;
}

/* replacement for the processor instruction, to test in C
   assuming x != 0 */
int trailingZeros(unsigned int x)
{
   int count = 0;
   x = ~x;
   while(x & 1)
   {
      x >>= 1;
      count++;
   }
   return count;
}

int *countBits(int *dataPtr, int *dataEndPtr)
{
   int *bitCounts = (int *) calloc(MAX_BIT + 1, sizeof(int)); /* 0th int unused, to skip a -1 in loop */
   int bitPosition;
   int shift;
   
   unsigned int c1 = 0, c2; /* bit accumulators */
   
   while(dataPtr < dataEndPtr)
   {
      c2 = c1 & *dataPtr;
      c1 ^= *dataPtr++;
      
      bitPosition = 0;
      while(c2 != 0)
      {
         bitPosition += shift = trailingZeros(c2) + 1;
         c2 >>= shift;
         bitCounts[bitPosition] += 2;
      }
   }
   
   /* end of data, c1 still holds some uncounted bits */
   bitPosition = 0;
   while(c1 != 0)
   {
      bitPosition += shift = __builtin_ffs(c2);// bitPosition += shift = trailingZeros(c1) + 1; 
      c1 >>= shift;
      bitCounts[bitPosition]++;
   }
   
   return bitCounts;
}
*/