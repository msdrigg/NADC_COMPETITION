fun main(args : Array<String>) {
    val n = readLine()!!.toInt()
    val twoToN = 1 shl n;
    val count = 1 shl n shl n;
    val trominoMax = (count - 1) / 3;
    val trominoMapping = IntArray(trominoMax + 1);
    var trominoFoundCount = 0;
    
    var matrix = IntArray(count)

    for (row in 0 until twoToN) {
        val intRow = readLine()!!.split(' ').map{it.toInt()};
        for ((col, gridPoint) in intRow.withIndex()) {
            matrix[row * twoToN  + col] = gridPoint;
        }
    }

    for (row in 0 until twoToN) {
        for (col in 0 until twoToN) {
            val trominoType = matrix[row * twoToN + col];
            if (trominoType == 0) {
                if (trominoMapping[0] == 0) {
                    trominoMapping[0] = 1
                    trominoFoundCount += 1;
                } else {
                    println("0")
                    return
                }
            } else if (((col > 0 && matrix[row * twoToN + col - 1] == trominoType) || (col < twoToN - 1 && matrix[row*twoToN + col + 1] == trominoType)) && 
                ((row > 0 && matrix[(row - 1) * twoToN + col] == trominoType) || (row < twoToN - 1 && matrix[(row + 1)*twoToN + col] == trominoType))) {

                    if (trominoMapping[trominoType] == 0) {
                        trominoMapping[trominoType] = 1
                        trominoFoundCount += 1;
                    } else {
                        println("0")
                        return
                    }
                }
        }
    }

    if (trominoFoundCount < trominoMax + 1) {
        println("0")
    } else {
        println("1")
    }
}