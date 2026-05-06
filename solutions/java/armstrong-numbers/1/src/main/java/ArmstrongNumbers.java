class ArmstrongNumbers {

    boolean isArmstrongNumber(int numberToCheck) {
        String s = String.valueOf(Math.abs(numberToCheck));
        int len = s.length();
        int sum = 0;
        for (int i = 0; i < len; i++) {
            int digit = s.charAt(i) - '0';
            sum += Math.pow(digit, len); 
        }
        return sum == numberToCheck;
    }
}
