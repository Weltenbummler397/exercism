import java.util.*;

class NucleotideCounter {

    Map<Character, Integer> counts = new HashMap<>();
    NucleotideCounter(String sequence) {
        String[] chars = sequence.trim().split("");
        int A = 0;
        int C = 0;
        int G = 0;
        int T = 0;

        for(String ch : chars) {
            switch (ch) {
                case "A":
                    A++;
                    break;
                case "C":
                    C++;
                    break;
                case "G":
                    G++;
                    break;
                case "T":
                    T++;
                    break;
                case "":
                    break;
                default:
                    throw new IllegalArgumentException("The String must contain only A, C, G or T");
            }
        }    
        counts.put('A', A);
        counts.put('C', C);
        counts.put('G', G);
        counts.put('T', T);
    }

    Map<Character, Integer> nucleotideCounts() {
        return counts;
    }

}