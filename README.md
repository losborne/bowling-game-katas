# bowling-game-katas
Practice bowling game katas in various languages.

## The Bowling Game Kata

The bowling game kata is an exercise created by Robert Martin that uses Test
Driven Development (TDD). It is an exercise meant to be memorized and practiced
until one achieves muscle memory.

By practicing the exercise in different languages, you solidify the skills of
writing tests, writing code itself, and refactoring efficiently. It also serves
as a chance to hone your development environment to optimize the
red-green-refactor cycle of TDD for a particular language.

## The Rules of Bowling
A bowling game has 10 frames.

```
+-----+-+-----+-+-----+-+-----+-+----+-+-----+-+-----+-+-----+-+----+-+-------+
| 1 |4| | 4 |5| | 6 |/| | 5 |/| |  |X| | 0 |1| | 7 |/| | 6 |/| |  |X| | 2 |/|6|
|   --+ |   --+ |   --+ |   --+ |  --+ |   --+ |   --+ |   --+ |  --+ |   ----+
|  5  | |  14 | |  29 | |  49 | | 60 | |  61 | |  77 | |  97 | | 117| |  133  |
+-----+-+-----+-+-----+-+-----+-+----+-+-----+-+-----+-+-----+-+----+-+-------+
```

In each frame, the player has two changes to knock down 10 pins. The score for
the frame is the total number of pins knocked down, plus a bonus if the frame
has a strike or a spare.

A spare is when the player knocks down all pins in one frame using two balls.
The bonus for a spare is to add the number of pins knocked down in the next roll
to the frame.

A strike is when a player knocks down all 10 pins in the first roll of the
frame. The bonus for a strike is to add the number of pins knocked down in the
next two rolls to the frame.

Bonuses from strikes or spares perpetuate all the way to the end of the game in
the case of chains of spares or strikes.

The tenth frame is slightly different. If a player gets a spare in the first two
balls of the tenth frame, they can roll one more time.  If the player gets a
strike in the first roll, they can roll two more times.

## The Bowling Game Problem

Write a class that calculates the score of a bowling game that has two methods:
    * roll(pins) is called each time the player rolls a ball. The argument is the
      number of pins knocked down.
    * score() is called at the very end of the game and returns the total score
      for that game.

### References
    * <http://butunclebob.com/ArticleS.UncleBob.TheBowlingGameKata>
    * <http://butunclebob.com/files/downloads/Bowling%20Game%20Kata.ppt>
