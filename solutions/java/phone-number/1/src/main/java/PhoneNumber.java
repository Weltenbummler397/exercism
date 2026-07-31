class PhoneNumber {
    String num;
    
    PhoneNumber(String numberString) {
        if (numberString.matches(".*[a-zA-Z].*")) {
            throw new IllegalArgumentException("letters not permitted");
        }
        if (numberString.matches(".*[^0-9a-zA-Z\\s.()+-].*")) {
            throw new IllegalArgumentException("punctuations not permitted");
        }
        String temp = numberString.trim().replaceAll("[^0-9]", "");
        if (temp.length() < 10) {
            throw new IllegalArgumentException("must not be fewer than 10 digits");
        } else if (temp.length() > 11){
            throw new IllegalArgumentException("must not be greater than 11 digits");
        }else if (temp.length() == 11 && temp.charAt(0) != '1') {
            throw new IllegalArgumentException("11 digits must start with 1");
        } else if (temp.length() == 11) {
            temp = temp.substring(1);
        }
        if (temp.charAt(0) == '0') {
            throw new IllegalArgumentException("area code cannot start with zero");
        } else if (temp.charAt(0) == '1') {
            throw new IllegalArgumentException("area code cannot start with one");
        } else if (temp.charAt(3) == '0') {
            throw new IllegalArgumentException("exchange code cannot start with zero");
        } else if (temp.charAt(3) == '1') {
            throw new IllegalArgumentException("exchange code cannot start with one");
        }
        num = temp;
    }

    String getNumber() {
        return num;
    }

}