|    |        Cube 3x3          |          All possible twists      |           no L after L         |   no L after L no L+R after R   |
|---:|-------------------------:|----------------------------------:|-------------------------------:|--------------------------------:|
|  0 |                        1 |                         1 ( 1,0x) |                       1 (1,0x) |                       1 (1,00x) |
|  1 |                       18 |                        18 ( 1,0x) |                      18 (1,0x) |                      18 (1,00x) |
|  2 |                      243 |                       324 ( 1,3x) |                     270 (1,1x) |                     243 (1,00x) |
|  3 |                    3'240 |                     5'832 ( 1,8x) |                   4'050 (1,3x) |                   3.281 (1,01x) |
|  4 |                   43'239 |                   104'976 ( 2,4x) |                  60'750 (1,4x) |                  44.287 (1,02x) |
|  5 |                  574'908 |                 1'889'568 ( 3,3x) |                 911'250 (1,6x) |                 597.871 (1,04x) |
|  6 |                7'618'438 |                34'012'224 ( 4,5x) |              13'668'750 (1,8x) |               8.071.260 (1,06x) |
|  7 |              100'803'036 |               612'220'032 ( 6,1x) |             205'031'250 (2,0x) |             108.962.013 (1,08x) |
|  8 |            1'332'343'288 |            11'019'960'576 ( 8,3x) |           3'075'468'750 (2,3x) |           1.470.987.169 (1,10x) |
|  9 |           17'596'479'795 |           198'359'290'368 (11,3x) |          46'132'031'250 (2,6x) |          19.858.326.784 (1,13x) |
| 10 |          232'248'063'316 |         3'570'467'226'624 (15,4x) |         691'980'468'750 (3,0x) |         268.087.411.582 (1,15x) |
| 11 |        3'063'288'809'012 |        64'268'410'079'232 (21,0x) |      10'379'707'031'250 (3,4x) |       3.619.180.056.351 (1,18x) |
| 12 |       40'374'425'656'248 |     1'156'831'381'426'180 (28,7x) |     155'695'605'468'750 (3,9x) |      48.858.930.760.742 (1,21x) |
| 13 |      531'653'418'284'628 |    20'822'964'865'671'200 (39,2x) |   2'335'434'082'031'250 (4,4x) |     659.595.565.270.016 (1,24x) |
| 14 |    6'989'320'578'825'358 |   374'813'367'582'081'000 (53,6x) |  35'031'511'230'468'800 (5,0x) |   8.904.540.131.145.210 (1,27x) |
| 15 |   91'365'146'187'124'313 | 6'746'640'616'477'460'000 (73,8x) | 525'472'668'457'031'000 (5,8x) | 120.211.291.770.460.000 (1,32x) |

Cube 3x3 from https://oeis.org/A080601


|     |        Cube 3x3          |  Cube 2x2  |      Subset    |      Coset    |
|----:|-------------------------:|-----------:|---------------:|--------------:|
|   0 |                        1 |          1 |              1 |             1 |
|   1 |                       18 |         18 |             10 |             4 |
|   2 |                      243 |        243 |             67 |            50 |
|   3 |                    3'240 |      2'874 |            456 |           592 |
|   4 |                   43'239 |     28'000 |          3'079 |         7'156 |
|   5 |                  574'908 |    205'416 |         19'948 |        87'236 |
|   6 |                7'618'438 |  1'168'516 |        123'074 |     1'043'817 |
|   7 |              100'803'036 |  5'402'628 |        736'850 |    12'070'278 |
|   8 |            1'332'343'288 | 20'776'176 |      4'185'118 |   124'946'368 |
|   9 |           17'596'479'795 | 45'391'616 |     22'630'733 |   821'605'960 |
|  10 |          232'248'063'316 | 15'139'616 |    116'767'872 | 1'199'128'738 |
|  11 |        3'063'288'809'012 |     64'736 |    552'538'680 |    58'202'444 |
|  12 |       40'374'425'656'248 |            |  2'176'344'160 |           476 |
|  13 |      531'653'418'284'628 |            |  5'627'785'188 |               |
|  14 |    6'989'320'578'825'358 |            |  7'172'925'794 |               |
|  15 |   91'365'146'187'124'313 |            |  3'608'731'814 |               |
|  16 |                        ? |            |    224'058'996 |               |
|  17 |                        ? |            |      1'575'608 |               |
|  18 |                        ? |            |          1'352 |               |
|  19 |                        ? |            |                |               |
|  20 |                        ? |            |                |               |
| Sum |                          | 88'179'840 | 19'508'428'800 | 2'217'093'120 |


With "wsl ./twophase -s 20 -q < test_pos_big.txt", cube20src can solve test_pos_big.txt in 3.75 seconds. That's 375 us per pos on average.

# Answered research questions

## Q1: With "unique_twists_after", what's the shortest distance of a subset config to an other subset config?
A: 5 (e.g. L1 R1 U2 L1 R1).

## Q2: Is the two phase solver equally fast, if the Twist::None is removed and the logic replaced by branching?
A: Yes.

# Open research questions

## Q1: Which of the 4x6 colour rotations are independent?
Task 1: Create all 4x6 colour rotations for a given cube.
Task 2: Take 1 million randomly twisted cubes and calculate the coset distance of each colour rotation.
Task 3: Create a correlation plot for this data to see how the different colour rotations correlate.
Task 4: Use the uncorrelated colour rotations to speed up the two phase solver.

## Q2: When does the corner table cut in the two phase solver?
Task 1: Log at which depth in the search the corner tables prunes.
Task 2: If it's only relevant to know that a corner configuration is above a given depth, compactify the corner table. Maybe this reduces RAM pressure and leads to better cache usage.

## Q3: Are there subset configurations in the subset table that have a shorter distance to the solution when all possible twists are considered?
Task 1: Create a brute force solver.
Task 2: Randomly pick configurations from the subset table and check if the brute force solver finds quicker solutions.
Task 3: If it does, use this information to improve the subset table.


