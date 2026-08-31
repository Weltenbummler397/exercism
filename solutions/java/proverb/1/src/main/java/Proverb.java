class Proverb {
    private String result;
    
    Proverb(String[] words) {
        if (words.length == 0) {
        result = "";
        } else {
            StringBuilder s = new StringBuilder();
            int anzahl = words.length;
            for (int i = 0; i<anzahl-1; i++) {
                s.append("For want of a ")
                    .append(words[i])
                    .append(" the ")
                    .append(words[i+1])
                    .append(" was lost.\n");
            }
            s.append("And all for the want of a ")
                .append(words[0])
                .append(".");
            result = s.toString();
        }
    }

    String recite() {
        return result;
    }

}
