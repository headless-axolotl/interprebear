# bear (esolang)

Welcome to the bear esoteric language!

In this two-dimensional language you do not really write programs
but obstacle courses for unlucky bears which are coerced to do stuff
when they step on certain things.

Although there is support for multiple bears, currently
only programs with one bear have been tested.

## state

In this language bears hold state in their baskets.
They have a dynamic array of cells which are integer values.
In addition, each bear "holds" a singular value in its mouth
and another "swap" value in its paw.

There is one selected value in the bear's basket. Values
can be inserted and popped at the end of the dynamic array.
The dynamic array's size cannot become less than 1. The
initial value of the first cell of the dynamic array is
0, the bear holds 0 and the swap of the bear is also 0.

## runtime

Bears start at predetermined spots marked by the bear character
(in "askii_bear" the bear character is '#'). Afterwards they start
following the closest food they can reach and they haven't
eaten immediately before. There is replenishable (or infinite)
food and perishable (one-time) food. If there isn't any
reachable food the bear gives up and the program terminates
(in the case of multiple bears the program should terminate
when all bears have given up).

There is a special symbol makes a bear enter "collect" mode.
In this mode some of the functions of the symbols change.

Symbols which do not have a description in collect mode do the
same thing as in normal mode. The symbols that do an arithmetic or bitwise operation
store the result of the operation to the integer the bear holds in its mouth.

|  Symbol   | Normal mode | Collect mode |
|-          |-            |-             |
| `.` | An empty cell.  | |
| `#`       | Marks the starting position of a bear. Does nothing afterwards. | |
| `>`       | One way door. Allows the bear to pass only from west to east. | |
| `_`       | One way door. Allows the bear to pass only from north to south. | |
| `<`       | One way door. Allows the bear to pass only from east to west. | |
| `^`       | One way door. Allows the bear to pass only from south to north. | |
| `:`       | A gate that allows the bear to step on this cell if and only if the integer the bear holds in its mouth is the same as the selected integer in is basket. When searching for the next closest food the bear considers all gates open and the check only happens when the bear attempts to move to the cell with the gate. | |
| `\|`      | A "tree". Blocks a cell. | |
| `~`       | Toggles collect mode. | |
| `'`       | Perishable food. | Left shifts the integer in the bear's mouth. |
| `@`       | Replenishable food. | Left shifts the integer in the bear's mouth and adds 1 to it. |
| `?`       | Inputs a string (from stdin). The characters of the string are converted to integers and are appended to the end of the bear's basket. | Inputs a number. Appends it to the end of the bear's basket. |
| `!`       | Interprets the selected integer in the bear's basket as a character. If it is a valid UTF-8 character, outputs (to stdout). | Outputs the selected integer in the bear's basket as a number. |
| `"`       | Changes the selected integer to the next cell. Loops back around if at the end. | Changes the selected integer to the previous cell in the dynamic array. Loops back around if at the beginning. |
| `+`       | Adds the selected integer to the one the bear holds in its mouth. | Subtracts the selected integer from the one the bear holds in its mouth. |
| `*`       | Multiplies the integer the bear holds in its mouth by the selected one. | Divides the integer the bear holds in its mouth by the selected one. (Currently not checking for division by 0.) |
| `&`       | Bitwise AND the selected integer and the one the bear holds in its mouth. | Bitwise OR the selected integer and the one the bear holds in its mouth. |
| `-`       | Flip the bits of the integer the bear holds in its mouth. | |
| `%`       | Swap the integer the bear holds in its mouth with the one in its paw. | |
| `=`       | Set the integer the bear holds in its mouth to the selected integer in its basket. | Set the selected integer in the bear's basket to the one the bear holds in its mouth. |
| `;`       | Append the integer the bear holds in its mouth to the end of the dynamic array. | Pop the last integer in the dynamic array if the length of the array is at least 2. |

Any other character is considered "None" and removed from the input.
If the lines of the file to be interpreted are "jagged" empty cells
should be appended to each line so that the area the bear explores
is rectangular in nature.

After a better error handling is written for the interpreter the stop conditions will be described better.
