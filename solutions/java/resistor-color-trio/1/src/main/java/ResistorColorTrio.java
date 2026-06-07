class ResistorColorTrio {
    String label(String[] colors) {
        String result = "";
        for (int i = 0; i < 3; i++) {
            switch (colors[i]) {
                case "black":
                    result += "0";
                    break;
                case "brown":
                    result += "1";
                    break;
                case "red":
                    result += "2";
                    break;
                case "orange":
                    result += "3";
                    break;
                case "yellow":
                    result += "4";
                    break;
                case "green":
                    result += "5";
                    break;
                case "blue":
                    result += "6";
                    break;
                case "violet":
                    result += "7";
                    break;
                case "grey":
                    result += "8";
                    break;
                case "white":
                    result += "9";
                    break;
                
            } 
        }
        String basis = result.substring(0, 2);
        int anzahlNullen = Character.getNumericValue(result.charAt(2));
        StringBuilder finalerWert = new StringBuilder(basis);
        for (int i = 0; i < anzahlNullen;i++) {
            finalerWert.append("0");
        }

        long ohmWert = Long.parseLong(finalerWert.toString());

        if (ohmWert >= 1_000_000_000) {
            return (ohmWert / 1_000_000_000) + " gigaohms";
        } else if (ohmWert >= 1_000_000) {
            return (ohmWert / 1_000_000) + " megaohms";
        } else if (ohmWert >= 1_000) {
            return (ohmWert / 1_000) + " kiloohms";
        } else {
            return ohmWert + " ohms";
        }
    }
}
