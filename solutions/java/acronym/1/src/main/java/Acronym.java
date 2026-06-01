class Acronym {
    String acronym;
    Acronym(String phrase) {
        acronym = phrase.trim().replaceAll("[^a-zA-Z \\-]", "");
    }

    String get() {
        String[] words = acronym.split("[ -]+");
        String result = "";
        for(String word : words) {
            result += Character.toUpperCase(word.charAt(0));
        }
        return result;
    }

}
