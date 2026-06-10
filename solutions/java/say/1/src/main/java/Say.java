public class Say {

    public String say(long number) {
    if (number < 0 || number > 999_999_999_999L) {
        throw new IllegalArgumentException("Number out of range (0 to 999,999,999,999)");
    }
    
    if (number == 0) {
        return "zero";
    }

    String[] wordsOnes = {
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen"
    };

    String[] wordsTens = {
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
    };

    String[] groups = {"", "thousand", "million", "billion"};
    
    StringBuilder result = new StringBuilder();
    int groupIdx = 0;

    while (number > 0) {
        int chunk = (int) (number % 1000);

        if (chunk > 0) {
            StringBuilder chunkWords = new StringBuilder();
            int hundreds = chunk / 100;
            int remainder = chunk % 100;

            if (hundreds > 0) {
                chunkWords.append(wordsOnes[hundreds]).append(" hundred");
            }

            if (remainder > 0) {
                if (chunkWords.length() > 0) {
                    chunkWords.append(" ");
                }
                
                if (remainder < 20) {
                    chunkWords.append(wordsOnes[remainder]);
                } else {
                    int tens = remainder / 10;
                    int ones = remainder % 10;
                    chunkWords.append(wordsTens[tens]);
                    if (ones > 0) {
                        chunkWords.append("-").append(wordsOnes[ones]);
                    }
                }
            }

            if (!groups[groupIdx].isEmpty()) {
                chunkWords.append(" ").append(groups[groupIdx]);
            }

            // Prepend the current group's string to the main result
            if (result.length() > 0) {
                result.insert(0, " ");
            }
            result.insert(0, chunkWords.toString());
        }

        number /= 1000;
        groupIdx++;
    }

    return result.toString();
}
}
