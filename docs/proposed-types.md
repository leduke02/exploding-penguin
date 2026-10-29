Part 2: Caleb Kellum
Turnchoice:
—-----------------------------------------------------------------
	1: Stop_at_Fort: 
What it does: signifies that player has stopped at a fort
Result: lose 45 miles, run fort_shop function
	2: Hunt:
		What it does: signifies that a player has gone hunting
		Result: lose 45 miles(pre movement), run hunt function
	3: Continue:
		What it does: signifies that the player has chosen to move onto the next cycle
		Result: ends the current menu cycle and lets the code move to the other factors
—--------------------------------------------------------------------------
Is shared because all other parts rely on it to loop properly.


Part 3: Ryker Saenz
EatingLevel:
—-----------------------------------------------------------------
	1: Poor:
		What it does: lowest eating level for the turn
		Result: cheapest food cost, illness more likely
	2: Moderate:
		What it does: middle eating level for the turn
		Result: medium food cost, medium illness chance
	3: Well:
		What it does: highest eating level for the turn
		Result: highest food cost, illness less likely
—--------------------------------------------------------------------------
Is shared because: stored as last_eating_level in GameState. Part 4 reads it for illness probability and the final arrival food calculation, so it can't be local to Part 3.


Part 4: 
EventKind:
---------------------------------------------------------------
1. WagonBreakdown
   What it does: Wagon breaks down; you lose time and supplies
                 fixing it.
   Final result: Mileage drops about 15-20, misc supplies drop
                 about 8.

2. OxInjury
   What it does: An ox hurts its leg, slowing you for the rest
                 of the trip.
   Final result: Mileage drops 25, oxen_spend drops 20.

3. DaughterBrokenArm
   What it does: You stop and use supplies to make a sling.
   Final result: Small mileage loss (about 5-9), misc supplies
                 drop about 2-5.

4. OxWandersOff
   What it does: You spend time searching for the ox.
   Final result: Mileage drops 17.

5. SonLost
   What it does: You spend half a day finding your son.
   Final result: Mileage drops 10.

6. UnsafeWater
   What it does: You lose time looking for a clean spring.
   Final result: Mileage drops about 2-12.

7. BanditAttack
   What it does: Shooting challenge scaled by skill_level.
   Final result: Slow shot: lose cash and supplies, possible
                 injury. Not enough bullets: game over.

8. WagonFire
   What it does: Fire damages the wagon.
   Final result: Food drops 40, bullets drop 400, misc supplies
                 drop about 3-11, mileage drops 15.

9. Fog
   What it does: You lose your way in heavy fog.
   Final result: Mileage drops about 10-15.

10. Snakebite
    What it does: You kill a snake after it bites you.
    Final result: Bullets drop 10, misc supplies drop 5. If misc
                  supplies go negative, game over.

11. RiverFording
    What it does: Wagon gets swamped crossing a river.
    Final result: Food drops 30, misc supplies drop 20, mileage
                  drops about 20-40.

12. WildAnimalAttack
    What it does: Shooting challenge.
    Final result: Bullets used. Too few bullets or too slow: lose
                  food/supplies or game over.

13. HailStorm
    What it does: Hail damages supplies.
    Final result: Mileage drops about 5-15, bullets drop 200,
                  misc supplies drop about 4-7.




14. HeavyRainOrColdWeather   (special case)
   What it does: One table entry that branches on mileage inside
                 apply_event. Early in the trip it's heavy rain;
                 past the mountains it's cold weather.
   Final result: Rain: lose time, food, bullets and supplies.
                 Cold: clothing decides whether you get ill
                 (illness_pending).
 
15. EatingDependentIllness   (special case)
   What it does: Silent illness roll with no message of its own,
                 using last_eating_level.
   Final result: Poor eating makes illness likely, so
                 illness_pending may be set. Well eating makes
                 it unlikely.
 
16. FriendlyIndians   (special case)
   What it does: Fallback event where Indians help you.
   Final result: Food increases by about 14.
---------------------------------------------------------------
Is internal to Part 4 because only roll_random_event returns it and only apply_event matches on it; no other part needs to construct or match on it. 
