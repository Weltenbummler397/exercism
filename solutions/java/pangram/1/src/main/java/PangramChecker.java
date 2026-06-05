public class PangramChecker {

    public boolean isPangram(String input) {
        String word = input.toUpperCase();
        for (char ch = 'A'; ch <= 'Z'; ch++) {
            if (!word.contains(String.valueOf(ch))) {
                return false;
            }
        }    
        return true;
    }

}
