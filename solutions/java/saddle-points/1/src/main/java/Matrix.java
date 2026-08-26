import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class Matrix {
    private Set<MatrixCoordinate> coords = new HashSet<>();

    Matrix(List<List<Integer>> values) {
        if (values.isEmpty() || values.get(0).isEmpty()) {
            return;
        }

        List<Integer> rowMax = new ArrayList<>(); // Sattelpunkt: Zeilen-Maximum
        List<Integer> columnMin = new ArrayList<>(); // Sattelpunkt: Spalten-Minimum

        // 1. Spalten-Minimum-Liste mit MAX_VALUE initialisieren
        int numColumns = values.get(0).size();
        for (int i = 0; i < numColumns; i++) {
            columnMin.add(Integer.MAX_VALUE);
        }

        // 2. Zeilen-Maxima und Spalten-Minima berechnen
        for (List<Integer> row : values) {
            rowMax.add(Collections.max(row)); // Sucht das Maximum der Zeile
            
            for (int colIdx = 0; colIdx < row.size(); colIdx++) {
                int currentVal = row.get(colIdx);
                // Wenn der aktuelle Wert kleiner ist als das bisherige Minimum dieser Spalte
                if (currentVal < columnMin.get(colIdx)) {
                    columnMin.set(colIdx, currentVal); // Spalten-Minimum aktualisieren
                }
            }
        }

        // 3. Schnittpunkte finden (Sattelpunkte)
        for (int rowIdx = 0; rowIdx < values.size(); rowIdx++) {
            for (int colIdx = 0; colIdx < numColumns; colIdx++) {
                int currentVal = values.get(rowIdx).get(colIdx);
                
                // Ein Sattelpunkt ist >= dem Zeilenmaximum und <= dem Spaltenminimum
                if (currentVal == rowMax.get(rowIdx) && currentVal == columnMin.get(colIdx)) {
                    coords.add(new MatrixCoordinate(rowIdx + 1, colIdx + 1));
                }
            }
        }
    }


    Set<MatrixCoordinate> getSaddlePoints() {
        return coords;
    }
}
