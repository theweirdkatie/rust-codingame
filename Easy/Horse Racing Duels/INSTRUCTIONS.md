# Horse-Racing Duels
Casablanca’s hippodrome is organizing a new type of horse racing: duals. During a dual, only two horses will participate in the race. In order for the race to be interesting, it is necessary to try to select two horses with similar strength.

Write a program which, using a given number of strengths, identifies the two closest strengths and shows their difference with an integer (≥ 0).

>#### Input
>
>**Line 1:** Number `N` of horses
>**The `N` following lines:** the strength `P_i` of each horse. `P_i` is an integer.
>
>#### Output
>The difference `D` between the two closest strengths. `D` is an integer greater than or equal to 0.
>
>#### Constraints
>1 < `N`  < 100000
>0 < `P_i` ≤ 10000000

>##### Example
>|Input|Output|
>|-----|------|
>|3<br>5<br>8<br>9|1|