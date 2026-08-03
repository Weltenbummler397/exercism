import java.util.*;

class SumOfMultiples {
    int sum;
    SumOfMultiples(int number, int[] set) {
        HashSet<Integer> mySet = new HashSet<>();
        for (int digit : set) {
            if (digit == 0) {
                continue;
            }
        int count = 1;
        int result;
        while ((result = count * digit) < number) {
            mySet.add(result);
            count++;
        }
        }
        int sum = 0;
        for (int digit : mySet) {
            sum += digit;
        }
        this.sum = sum;
    }

    int getSum() {
        return sum;
    }

}
