# Advent of Code 2023

Attempting this year with rust

## Log

⭐️ Spoilers lurk below ⭐️

**--- Day 1: Trebuchet?! ---**

Read through the first day of 'Comprehensive Rust' and jumped straight into sliding windows of strings... so a much harder opener than other years - punishing several naive approaches that would normally slide for a few days.

Final code is not pretty, with some gross labelled loop...break blocks.

**--- Day 2: Cube Conundrum ---**

Used today's puzzles to try out some of the functional features of iterators and am fairly happy with the result. The top-level functions are nice and readable. The inner workings of the input parsing: less so.

**--- Day 3: Gear Ratios ---**

Enjoyed day 3, but didn't get to it until the 4th. Solution is fairly clean, but didn't get a huge amount of reuse between parts 1 and 2.

**--- Day 4: Scratchcards ---**

Lots of parsing again today - tackled it in one functional chain which wasn't very pretty. Might reach for regexp next time there's something similar. Part 2 sounded like it was drifting towards some nasty recursion, but with the given limits was fairly straightforward.

**--- Day 5: If You Give A Seed A Fertilizer ---**

Once I stopped trying to store billions of ints in a HashMap, part 1 suddenly became very doable... funny that. The rust compiler is doing all of the heavy lifting for me in part 2. My solution is just a brute force through all seeds. Unoptimised builds wouldn't even churn through the 1st range, but optimised did the whole thing in ~2 minutes. I think I figured out the nicer approach to take, but won't refactor it unless I catch back up with the other days.

**--- Day 6: Wait For It ---**

Got up a bit earlier for today. Could have been a lot faster by not bothering to parse the input, but did so anyway in the interests of keeping the inputs secret and being a bit too purist...

Realised while doing part 2 that the twist was 64-bit ints and enjoyed barrelling and just changing some u32s to u64s. I imagine there's a more elegant maths solution as well.

**--- Day 7: Camel Cards ---**

A lot of upfront logic in building the Hand struct and determining HandType derivation. Learnt a bit about `#[derive ... ]` for the enum and writing a custom `PartialOrd` function for the ranking. I vey much shoehorned in part 2 by passing a flag all the way through to the core logic but it worked well.

**--- Day 8: Haunted Wasteland ---**

Nice and easy first part with HashMap. Then let an optimised brute force for part 2 run for fun while trying to prove that the sequences looped nicely and implementing LCM (via GCD with the Euclidean algorithm which I definitely did not remember). It got ~0.02% there.

**--- Day 9: Mirage Maintenance ---**

Pretty simple one for day 09. Repeatedly called a small function to reduce the sequences to zeroes, and picked up on summing the final elements for part 1 quickly. I don't have an intuitive understanding for why alternately adding and subtracting works for part 2 - found it by quick trial and error.

**--- Day 10: Pipe Maze ---**

Could have done part 1 a fair bit faster, but ended up fighting a bunch of spaghetti code that couldn't decide if it was using positional offsets or directional enums - made more complicated by a dodgy application of a Tile enum. Came back to part 2 in the afternoon with a fairly good idea of what the heuristic for an enclosed tile would be. Spent a long time debugging why the new example wouldn't even get through the part 1 solver before realising the input wasn't square...

**--- Day 11: Cosmic Expansion ---**

Thought today was going to be based on cellular automata at first; still enjoyed both parts. The part 2 solution evaded me due to a calculation error with how I was expanding the space. Tidied up and DRYed out before pushing.

**--- Day 12: Hot Springs ---**

Only done part 1 so far. Solution is well organised and fairly readable, but unoptimised both in terms of generating permutations that are trivially invalid and by doing a lot of Vec cloning where I imagine proper knowledge of Rust would point to a better approach.
Will have to think on how to reduce part 2, given that for some of the more unknown records, the number of permutations is just absurdly large (~2^79).

**--- Day 13: Point of Incidence ---**

This solution isn't very pretty, lots of duplication and a fairly gross bit of logic in part 2 to determine whether the lines of reflection are new or old, instead of just having the initial results to compare against. Learnt a bit about Result<T, Err>. Also the reflection finding is pretty naive - likely a better way to do this.

I had mostly caught up by the 13th, but quickly fell even further behind. Time to try and catch up a bit...

**--- Day 14: Parabolic Reflector Dish ---**

Enjoyed this one, not too difficult a concept, so just had to go through the motions of being able to 'roll' in each direction. Kept track of seen states for the loop detection needed for part 2 using strings of the entire state which is not going to be space efficient, but hey.
