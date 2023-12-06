# Advent of Code 2023

Attempting this year with rust

## Log

**--- Day 1: Trebuchet?! ---**

Read through the first day of 'Comprehensive Rust' and jumped straight into sliding windows of strings... so a much harder opener than other years - punishing several naive approaches that would normally slide for a few days.

Final code is not pretty, with some gross labelled loop...break blocks.

**--- Day 2: Cube Conundrum ---**

Used today's puzzles to try out some of the functional features of iterators and am fairly happy with the result. The top-level funcitons are nice and readable. The inner workings of the input parsing: less so.

**--- Day 3: Gear Ratios ---**

Enjoyed day 3, but didn't get to it until the 4th. Solution is fairly clean, but didn't get a huge amount of reuse between parts 1 and 2.

**--- Day 4: Scratchcards ---**

Lots of parsing again today - tackled it in one functional chain which wasn't very pretty. Might reach for regexp next time there's something similatr. Part 2 sounded like it was drifting towards some nasty recursion, but with the given limits was fairly straightworward.

**--- Day 5: If You Give A Seed A Fertilizer ---**

Started yesterday but haven't finished either part yet.

**--- Day 6: Wait For It ---**

Got up a bit earlier for today. Could have been a lot faster by not bothering to parse the input, but did so anyway in the interests of keeping the inputs secret and being a bit too purist...

Realised while doing part 2 that the twist was 64-bit ints and enjoyed barrelling and just changing some u32s to u64s. I imagine there's a more elegant maths solution as well.
