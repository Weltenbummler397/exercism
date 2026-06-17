import java.util.ArrayList;
import java.util.List;

class Allergies {
    private final int score;

    Allergies(int score) {
        this.score = score;
    }

    boolean isAllergicTo(Allergen allergen) {
        // Prüft, ob das spezifische Bit des Allergens im Score gesetzt ist
        return (this.score & allergen.getScore()) != 0;
    }

    public List<Allergen> getList() {
        List<Allergen> allergicList = new ArrayList<>();
        
        // Geht alle Allergene der Reihe nach durch (EGGS, PEANUTS...)
        for (Allergen allergen : Allergen.values()) {
            if (isAllergicTo(allergen)) {
                allergicList.add(allergen);
            }
        }
        
        return allergicList;
    }
}
