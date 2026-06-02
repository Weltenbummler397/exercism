public class Hamming {

    String[] left;
    String[] right;    
    public Hamming(String leftStrand, String rightStrand) {
        leftStrand = leftStrand.trim();
        rightStrand = rightStrand.trim();

        if (leftStrand.length() != rightStrand.length()) {
            throw new IllegalArgumentException("strands must be of equal length");
        }

        left = leftStrand.toUpperCase().split("");
        right = rightStrand.toUpperCase().split("");
    }

    public int getHammingDistance() {
        int len = left.length;
        int result = 0;
        
        for (int i = 0; i < len; i++) {
            if (!left[i].equals(right[i])) {
                result++;
            }
        }
        return result;
    }
}
