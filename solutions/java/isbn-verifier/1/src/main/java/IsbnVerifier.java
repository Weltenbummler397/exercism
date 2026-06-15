class IsbnVerifier {

    boolean isValid(String stringToVerify) {
        String cleanString = stringToVerify.replace("-", "");
        
        if (cleanString.length() != 10) {
            return false;
        }

        String[] textDigits = cleanString.split("");
        int[] digits = new int[10];

        for (int i = 0; i < 10; i++) {
            String current = textDigits[i];
            
            if (i == 9 && "X".equalsIgnoreCase(current)) {
                digits[i] = 10;
            } else {
                if (!current.matches("[0-9]")) {
                    return false; 
                }
                digits[i] = Integer.parseInt(current);
            }
        }
        
        int sum = 0;
        for (int i = 10; i > 0; i--) {
            sum += digits[10 - i] * i;
        }
        
        return (sum % 11) == 0;
    }
}
