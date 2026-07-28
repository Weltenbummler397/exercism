class CollatzCalculator {

    int computeStepCount(int start) {
        if (start <= 0) {
            throw new IllegalArgumentException("Only positive integers are allowed");
        }
        int digit = start;
        int count = 0;
        while(digit > 1) {
            count++;
            if (digit % 2 == 0) {
                digit /= 2;
            } else {
                digit *= 3;
                digit++;
            }
        }
        return count;
    }

}
