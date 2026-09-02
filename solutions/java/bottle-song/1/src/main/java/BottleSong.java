class BottleSong {
    private static final String[] WORDS = {
        "no", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine", "ten"
    };

    String recite(int startBottles, int takeDown) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < takeDown; i++) {
            int count = startBottles - i;
            if (i > 0) {
                sb.append("\n");
            }
            sb.append(verse(count));
        }
        return sb.toString();
    }

    private String verse(int count) {
        StringBuilder sb = new StringBuilder();
        String capitalized = capitalize(WORDS[count]);
        String bottlesWord = bottles(count);
        sb.append(capitalized).append(" ").append(bottlesWord).append(" hanging on the wall,\n");
        sb.append(capitalized).append(" ").append(bottlesWord).append(" hanging on the wall,\n");
        sb.append("And if one green bottle should accidentally fall,\n");
        sb.append("There'll be ").append(WORDS[count - 1]).append(" ")
          .append(bottles(count - 1)).append(" hanging on the wall.\n");
        return sb.toString();
    }

    private String bottles(int count) {
        return count == 1 ? "green bottle" : "green bottles";
    }

    private String capitalize(String word) {
        return Character.toUpperCase(word.charAt(0)) + word.substring(1);
    }
}