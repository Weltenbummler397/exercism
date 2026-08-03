public class SquareRoot {
    public int squareRoot(int radicand) {
        int count = 0;
        int prev = 0;
        int current = 0;
        while(true) {
            prev = current;
            current = count*count;
            if (current == radicand) {
                return count;
            }
            if (current > radicand && prev < radicand) {
                return 0;
            }
            count++;
        }
    }
}
