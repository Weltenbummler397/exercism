class RotationalCipher {
    int key;
    RotationalCipher(int shiftKey) {
        key = shiftKey;
    }

    String rotate(String data) {
        StringBuilder ergebnis = new StringBuilder();
        for (Character d : data.toCharArray()) {
            if (Character.isLowerCase(d)) {
                char neu = (char) (((d - 'a' + key) % 26) + 'a');
                ergebnis.append(neu);
            } else if (Character.isUpperCase(d)) {
                char neu = (char) (((d - 'A' + key) % 26) + 'A');
                ergebnis.append(neu);
            } else {
                ergebnis.append(d);
            }
        }
        return ergebnis.toString();
    }
}