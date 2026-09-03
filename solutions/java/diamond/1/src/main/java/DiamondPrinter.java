import java.util.*;

class DiamondPrinter {

    List<String> printToList(char a) {
        int n = (int) (a - 'A');
        int g = 2*n+1;
        List<String> diamanten_zeile = new ArrayList<>();
        for (int y = 0; y<g; y++) {
            StringBuilder aktuell = new StringBuilder();

            for (int x = 0; x<g; x++) {
                int abs_x = Math.abs(n-x);
                int abs_y = Math.abs(n-y);

                if (abs_x+abs_y == n) {
                    char buchstabe = (char) ('A'+(n-abs_y));
                    aktuell.append(buchstabe);
                } else {
                    aktuell.append(" ");
                }
            }
            diamanten_zeile.add(aktuell.toString());
        }
    return diamanten_zeile;
    }
}
