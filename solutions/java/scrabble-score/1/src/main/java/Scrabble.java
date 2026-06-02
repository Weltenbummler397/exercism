class Scrabble {
    String[] letters;
    Scrabble(String word) {
        this.letters = word.toUpperCase().trim().split("");
    }

    int getScore() {
        int sum = 0;
        for (String letter : letters) {
            switch (letter) {
                case "A":
                case "E":
                case "I":
                case "O":
                case "U":
                case "L":
                case "N":
                case "R":
                case "S":
                case "T":
                    sum += 1;
                    break;
                case "D":
                case "G":
                    sum += 2;
                    break;
                case "B":
                case "C":
                case "M":
                case "P":
                    sum += 3;
                    break;
                case "F":
                case "H":
                case "V":
                case "W":
                case "Y":
                    sum += 4;
                    break;
                case "K":
                    sum += 5;
                    break;
                case "J":
                case "X":
                    sum += 8;
                    break;
                case "Q":
                case "Z":
                    sum += 10;
                    break;
            }
        }
        return sum;
    }

}
