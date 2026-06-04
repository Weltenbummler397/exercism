import java.util.List;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Arrays;

class HandshakeCalculator {

    List<Signal> calculateHandshake(int number) {
        List<Signal> result = new ArrayList<>();
        String binaryStr = Integer.toBinaryString(number);
        String[] digits = binaryStr.split("");
        Collections.reverse(Arrays.asList(digits));
        for (int i = 0; i < digits.length; i++) {
            if (digits[i].equals("1")) {
                switch (i) {
                    case 0: 
                        result.add(Signal.WINK);
                        break;
                    case 1: 
                        result.add(Signal.DOUBLE_BLINK);
                        break;
                    case 2: 
                        result.add(Signal.CLOSE_YOUR_EYES);
                        break;
                    case 3: 
                        result.add(Signal.JUMP);
                        break;
                    case 4: 
                        Collections.reverse(result);
                        break;
                }
            }
        }
        return result;
    }

}
