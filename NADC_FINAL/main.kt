import java.io.*
import java.util.StringTokenizer
import java.util.PriorityQueue


/// Floodfill the grid recursively dropping down into a lower sized robot if necessary
fun floodfill(grid: IntArray, startingRobotSize: Int, rows: Int, cols: Int, startingRow: Int, startingCol: Int) {

    // Looped, breadth first search 
    // Action: (robotSize, idx, row, col, direction)
    // Direction: (rowDirection, colDirection)
    val operationQueue: PriorityQueue<IntArray> = PriorityQueue{a, b -> 
        val p0 = b[0] - a[0];
        if (p0 != 0) p0 else b[1] - a[1]
    }

    val firstOperation = IntArray(6)
    firstOperation[0] = startingRobotSize
    firstOperation[1] = 0
    firstOperation[2] = startingRow
    firstOperation[3] = startingCol
    firstOperation[4] = 0
    firstOperation[5] = 0

    for (row_adj in 0 until startingRobotSize) {
        for (col_adj in 0 until startingRobotSize) {
            grid[startingCol + col_adj + (startingRow + row_adj) * cols] = startingRobotSize
        }
    }

    operationQueue.add(firstOperation)

    var indexCounter = 1
    while (true) {

        // println(operationQueue.joinToString{
        //     "{ ${it.joinToString()} }"
        // })

        // println("[")
        // for (print_row in 0 until rows) {
        //     print("\t[ ")
        //     for (print_col in 0 until cols) {
        //         print("${grid[print_col + cols * print_row]} ")
        //     }
        //     println("]")
        // }
        // println("]")

        val nextOperation = operationQueue.poll() ?: return;
        val currentRobotSize = nextOperation[0]
        val currentRow = nextOperation[2]
        val currentCol = nextOperation[3]

        // Fill in the proper direction
        if (nextOperation[4] != 0) {
            // Operation is up/down
            val vertShift = if (nextOperation[4] == -1) {
                0
            } else {
                currentRobotSize - 1
            }
            for (col_adj in 0 until currentRobotSize) {
                grid[currentCol + col_adj + (currentRow + vertShift) * cols] = currentRobotSize.coerceAtLeast(grid[currentCol + col_adj + (currentRow + vertShift) * cols])
            }
        } else if (nextOperation[5] != 0) {
            // Operation is left/right
            val horizShift = if (nextOperation[5] == -1) {
                0
            } else {
                currentRobotSize - 1
            }
            for (row_adj in 0 until currentRobotSize) {
                grid[currentCol + horizShift + (currentRow + row_adj) * cols] = currentRobotSize.coerceAtLeast(grid[currentCol + horizShift + (currentRow + row_adj) * cols])
            }
        }

        // Check robot size and flood fill row down
        if (currentRow > 0) {
            var previous_valid_index = 0
            var minimum_in_range = Int.MAX_VALUE

            for (col_adjust in 0 until currentRobotSize) {
                if (grid[currentCol + col_adjust + (currentRow - 1) * cols] != Int.MAX_VALUE) {
                    minimum_in_range = grid[currentCol + col_adjust + (currentRow - 1) * cols].coerceAtMost(minimum_in_range)
                } else {
                    if (col_adjust - previous_valid_index > minimum_in_range) {
                        val newOperation = IntArray(6)
                        newOperation[0] = col_adjust - previous_valid_index
                        newOperation[1] = indexCounter
                        newOperation[2] = currentRow - 1
                        newOperation[3] = previous_valid_index + currentCol
                        newOperation[4] = -1
                        newOperation[5] = 0

                        indexCounter++
                        operationQueue.add(newOperation)
                    }
                    previous_valid_index = col_adjust + 1
                    minimum_in_range = Int.MAX_VALUE
                }
            }

            if (currentRobotSize - previous_valid_index > minimum_in_range) {
                val newOperation = IntArray(6)
                newOperation[0] = currentRobotSize - previous_valid_index
                newOperation[1] = indexCounter
                newOperation[2] = currentRow - 1
                newOperation[3] = previous_valid_index + currentCol
                newOperation[4] = -1
                newOperation[5] = 0

                indexCounter++
                operationQueue.add(newOperation)
            }
        }

        // Check robot size and flood fill row down
        if (currentRow + currentRobotSize < rows) {
            var previous_valid_index = 0
            var minimum_in_range = Int.MAX_VALUE

            for (col_adjust in 0 until currentRobotSize) {
                if (grid[currentCol + col_adjust + (currentRow + currentRobotSize) * cols] != Int.MAX_VALUE) {
                    minimum_in_range = grid[currentCol + col_adjust + (currentRow + currentRobotSize) * cols].coerceAtMost(minimum_in_range)
                } else {
                    val newRobotSize = col_adjust - previous_valid_index
                    if (newRobotSize > minimum_in_range) {
                        val newOperation = IntArray(6)
                        newOperation[0] = newRobotSize
                        newOperation[1] = indexCounter
                        newOperation[2] = currentRow + 1 + (currentRobotSize - newRobotSize)
                        newOperation[3] = previous_valid_index + currentCol
                        newOperation[4] = 1
                        newOperation[5] = 0

                        indexCounter++
                        operationQueue.add(newOperation)
                    }
                    previous_valid_index = col_adjust + 1
                    minimum_in_range = Int.MAX_VALUE
                }
            }

            val newRobotSize = currentRobotSize - previous_valid_index
            if (newRobotSize > minimum_in_range) {
                val newOperation = IntArray(6)
                newOperation[0] = newRobotSize
                newOperation[1] = indexCounter
                newOperation[2] = currentRow + 1 + (currentRobotSize - newRobotSize)
                newOperation[3] = previous_valid_index + currentCol
                newOperation[4] = +1
                newOperation[5] = 0

                indexCounter++
                operationQueue.add(newOperation)
            }
        }

        // Check robot size and flood fill currentRow down
        if (currentCol > 0) {
            var previous_valid_index = 0
            var minimum_in_range = Int.MAX_VALUE

            for (row_adjust in 0 until currentRobotSize) {
                if (grid[currentCol - 1 + (currentRow + row_adjust) * cols] != Int.MAX_VALUE) {
                    minimum_in_range = grid[currentCol - 1 + (currentRow + row_adjust) * cols].coerceAtMost(minimum_in_range)
                } else {
                    if (row_adjust - previous_valid_index > minimum_in_range) {
                        val newOperation = IntArray(6)
                        newOperation[0] = row_adjust - previous_valid_index
                        newOperation[1] = indexCounter
                        newOperation[2] = previous_valid_index + currentRow
                        newOperation[3] = currentCol - 1
                        newOperation[4] = 0
                        newOperation[5] = -1

                        indexCounter++
                        operationQueue.add(newOperation)
                    }
                    previous_valid_index = row_adjust + 1
                    minimum_in_range = Int.MAX_VALUE
                }
            }

            if (currentRobotSize - previous_valid_index > minimum_in_range) {
                val newOperation = IntArray(6)
                newOperation[0] = currentRobotSize - previous_valid_index
                newOperation[1] = indexCounter
                newOperation[2] = previous_valid_index + currentRow
                newOperation[3] = currentCol - 1
                newOperation[4] = 0
                newOperation[5] = -1

                indexCounter++
                operationQueue.add(newOperation)
            }
        }

        // Check robot size and flood fill currentRow down
        if (currentCol + currentRobotSize < cols) {
            var previous_valid_index = 0
            var minimum_in_range = Int.MAX_VALUE

            for (row_adjust in 0 until currentRobotSize) {
                if (grid[currentCol + currentRobotSize + (currentRow + row_adjust) * cols] != Int.MAX_VALUE) {
                    minimum_in_range = grid[currentCol + currentRobotSize + (currentRow + row_adjust) * cols].coerceAtMost(minimum_in_range)
                } else {
                    val newRobotSize = row_adjust - previous_valid_index
                    if (newRobotSize > minimum_in_range) {
                        val newOperation = IntArray(6)
                        newOperation[0] = newRobotSize
                        newOperation[1] = indexCounter
                        newOperation[2] = previous_valid_index + currentRow
                        newOperation[3] = currentCol + 1 + (currentRobotSize - newRobotSize)
                        newOperation[4] = 0
                        newOperation[5] = 1

                        indexCounter++
                        operationQueue.add(newOperation)
                    }
                    previous_valid_index = row_adjust + 1
                    minimum_in_range = Int.MAX_VALUE
                }
            }

                val newRobotSize = currentRobotSize - previous_valid_index
            if (newRobotSize > minimum_in_range) {
                val newOperation = IntArray(6)
                newOperation[0] = newRobotSize
                newOperation[1] = indexCounter
                newOperation[2] = previous_valid_index + currentRow
                newOperation[3] = currentCol + 1 + (currentRobotSize - newRobotSize)
                newOperation[4] = 0
                newOperation[5] = 1

                indexCounter++
                operationQueue.add(newOperation)
            }
        }

    }
}

/// Assume starting at 0, 0, but find real starting point along the smallest row and col 
/// that are not filled
fun findLargestStartingRobot(grid: IntArray, rows: Int, cols: Int): Triple<Int, Int, Int> {
    var startingCol = 0
    var startingRow = 0
    while (grid[startingCol + startingRow * cols] == Int.MAX_VALUE) {
        startingCol += 1
        if (startingCol == cols) {
            if (startingRow == rows - 1) {
                return Triple(-1, 0, 0)
            }
            startingCol = 0
            startingRow += 1
        }
    }

    val maxSize = (rows - startingRow).coerceAtMost(cols - startingCol)
    var trackedSize = 0


    while (trackedSize < maxSize) {
        for (rowAdj in 0 until trackedSize) {
            if (grid[startingCol + trackedSize + cols * (startingRow + rowAdj)] == Int.MAX_VALUE) {
                return Triple(trackedSize, startingRow, startingCol)
            }
        }

        for (colAdj in 0..trackedSize) {
            if (grid[startingCol + colAdj + cols * (startingRow + trackedSize)] == Int.MAX_VALUE) {
                return Triple(trackedSize, startingRow, startingCol)
            }
        }

        trackedSize += 1
    }

    return Triple(trackedSize, startingRow, startingCol)
}

fun main() {
    val line1 = readLine()!!.split(" ").map{it.toInt()}
    val rows = line1[0];
    val cols = line1[1];
    val k = line1[2];

    val count = rows * cols;
    // 0 = Untouched
    // Int.MAX_MAX_VALUE = Blocked
    // n = Cleaned by robot of n size
    val grid = IntArray(count);

    val reader = BufferedReader(InputStreamReader(System.`in`));
    for (i in 0 until k) {
        val line = StringTokenizer(reader.readLine()!!);
        val row = line.nextToken()!!.toInt();
        val col = line.nextToken()!!.toInt();
        grid[(row - 1) * cols + col - 1] = Int.MAX_VALUE
    }


    val startingConfig = findLargestStartingRobot(grid, rows, cols)
    val startingRobotSize = startingConfig.first;
    val startingRow = startingConfig.second;
    val startingCol = startingConfig.third;
    if (startingRobotSize == -1) {
        println("-1")
        return
    }

    // print("[\n")
    // for (print_row in 0 until rows) {
    //     print("\t[ ")
    //     for (print_col in 0 until cols) {
    //         print("${grid[print_col + cols * print_row]} ")
    //     }
    //     print("]\n")
    // }
    // println("]")

    floodfill(grid, startingRobotSize, rows, cols, startingRow, startingCol)

    // print("[\n")
    // for (print_row in 0 until rows) {
    //     print("\t[ ")
    //     for (print_col in 0 until cols) {
    //         print("${grid[print_col + cols * print_row]} ")
    //     }
    //     print("]\n")
    // }
    // println("]")

    var min = startingRobotSize
    for (row in 0 until rows) {
        for (col in 0 until cols) {
            // If grid is blocked, ignore it.
            if (grid[row * cols + col] != Int.MAX_VALUE) {
                if (min == 0) {
                    println(-1)
                    return
                }
                min = grid[row * cols + col].coerceAtMost(min)
            }
        }
    }

    println(min)
}