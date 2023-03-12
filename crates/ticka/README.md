
# Ticka

ticka is a multihrreaded tick based action processor for games with simultanous action commitment and conflict resolving strategies with a goal of scalability,
optimized for 64 bit architecture.

## Tick based
Tick based as used within this projects means that several actions can happen within the same game tick.
Those actions can even cause conflicts - like 2 NPC or Players trying to move to the same field.

## Units
Game Objects that can do an actions are named "Units" within this project.

## Planning
Units in the first phase generate a plan what they want to do.
This does NOT change any data of the current situation on the field.
Therefore only read access is required, and this can be highly parallelised.
Each thread delivers a collection of Plans on the heap.
therefore: The planning phase can be highly multithreaded, because each unit

## Conflicts
If a conflict in the planning occurs, there is a 2 phased conflict resolution strategy: gentle and hard.
Each conflict is represented by a struct, that holds informations about the conflicting members. 
The information is passed around 2 times:


### Conflict resolvement: Gentle Replanning of planing conflicts.
In the first round, conflict partners can willingly put back their decisions and make another decision.
Example: Take another screwdriver.
Since informations about "What other's try to do" is already there,
Units should decide for an action that probably does not resolve into a further conflict.

Example: 
There are 2 empty position close by, each  surrounded by 4 allied Units that plan to enter this position. (8 Units in total).
This results in 2 Conflicts, and in each conflict, and the gentle replanner will make it possible for 3 units in each group,
to do something else, like moving to another field.

So the Gentle Replaner receives 2 times a Conflict Struct with the 4 Units that are conflicting.
Furthermore it has access to all other commited plans, so it can choose a plan for each of the 3 other units that
won't result into another conflict.
but there is no garantee, that the Unit's won't end up into another conflict.
In this scenario, 1 Unit from each conflict could plan to enter the same position,
that would result into another conflict.

Therefore we reduced the Conflicts from 2 conflicts with 4 conflicting parties to 1 conflict with 2 conflicting parties in this example

This "could" now be recursive, but that could end up with an infinite chain of conflicts.
Therefore we have to limit the number of conflict resolve runs here.

But it is not required to solve all planning conflicts,
because existing planning conflicts will result in "execution conflicts"

### Conflict resolvement: Hard execution conflicts.

While Gentle Replaning of planing Conflicts aim's to reduce the conflict numbers between allied unit's,
hard execution conflicts are designed to the resolved in any case. 
There is no replanning, the ongoing action will get executed.
For the example of the 2 allied units that try to move to the same position, 
it means that only 1 unit will manage to move there, and the second one does nothing (or falls back into an alternative action). 
Hostile Units can be contested, who is the stronger or faster one to make the move.


# Non-Conflicting Actions.
There are several actions that can not lead to a conflict.
Example: Observe  

# Plan, Tactic, Strategy AI

## Plan
A Plan is processed with each tick, and goes into the conflict resolution pipeline

## Tactical AI
A Tactic is a lightweigt shortterm goal.
Example: Move to the city south east.
A Tactic AI delivers a Plan in each Tick, and does the potential replaning on a gentle conflict,
or deliver non-conflicting action alternatives. 
Tactic AIs need to deliver fast, and are executed on the server (or client in standalone)

## Strategic AI
A Strategic AI has the big picture and delivers tactical AI's.
The do changes very infrequently,
and are possible to run on a Node attached to the server,
allowing Latency.
They are in their own responsibility to switch out the Tactical AI,
Either on completing the task, or on seeing another Goal.

Example: 
During the travel to the city south east, the unit, a skilled hunter, observes a group of wild boars.
The Strategic AI exchanges the "move to city south east" tactical AI to "hunt boars".
