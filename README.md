# cop4520_hw2

## Problem 1: Minotaur’s Birthday Party
The solution I have come up with for this problem is for the guests to choose a leader. This leader is the only one who is allowed to request a cupcake. When any guests enters the labrynth, if they have not eaten a cupcake and a cupcake is available, they must eat the cupcake. Each time the leader enters the labrynth they must take note of whether or not the cupcake is there. If it is not there, that means a new person ate it and the leader requests a new one. If the cupcake is there that means no one new entered and must leave it there. Each time a cupcake is not there, the leader must add one to their counter. Once the counter is the same as the number of guests, the leader will then let the minatour know that all guests have entered the labrynth.  

The minatour then tells all guests to leave the party and the party is over.

Problem 1 can be simulated by typing:
```
cargo run
1
```

## Problem 2: Minotaur’s Crystal Vase
There are three strategies for the guests to choose for viewing the labrynth.

### Strategy 1
Pros: There is very minimal coordination required between the guests.

Cons: The guests must spend their time waiting for the vase rather than doing other things. Also it is not gauranteed whether a guest will be able to enter as it may always be busy when they check.

### Strategy 2
Pros: The guests can spend their time doing other things. Multiple guests can check the sign at the same time.

Cons: Each guests needs to follow the rules and make sure they flip the sign. It is not gauranteed whether a guest will be able to enter as it may always be busy when they check.

### Strategy 3
Pros: Every guest will have the chance to enter.

Cons: The guests must spend their time waiting for the vase rather than doing other things. Requires coordination between guests.

### Conclusion
I believe strategy 2 is the best strategy as the rules are simple and it allows each guest to do other things and is therefore the one that I implemented.

It can be simulated by typing:
```
cargo run
2
```
