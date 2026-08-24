import java.util.ArrayList;
import java.util.List;

class PrimeFactorsCalculator {

    List<Long> calculatePrimeFactorsOf(long number) {
        List<Long> result = new ArrayList<>();
        long divisor = 2L;
        while (number > 1L) {
            if (number % divisor == 0) {
                result.add(divisor);
                number /= divisor; // Zahl verkleinern
            } else {
                divisor++; // Wenn es nicht teilt, versuchen wir die nächste Zahl
            }
        }
        return result;
    }

}