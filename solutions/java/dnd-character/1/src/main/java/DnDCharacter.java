import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Random;

class DnDCharacter {
    
    // Wir brauchen ein Random-Objekt für die Würfel
    private final Random random = new Random();
    
    // Variablen, um die Werte für diesen Charakter zu speichern
    private final int strength;
    private final int dexterity;
    private final int constitution;
    private final int intelligence;
    private final int wisdom;
    private final int charisma;
    private final int hitpoints;

    // Der Konstruktor: Wenn ein neuer Charakter erstellt wird,
    // werden alle Werte automatisch einmal gewürfelt und berechnet.
    public DnDCharacter() {
        this.strength = ability(rollDice());
        this.dexterity = ability(rollDice());
        this.constitution = ability(rollDice());
        this.intelligence = ability(rollDice());
        this.wisdom = ability(rollDice());
        this.charisma = ability(rollDice());
        
        // Hitpoints hängen von der Konstitution ab
        this.hitpoints = 10 + modifier(this.constitution);
    }

    // Berechnet die Summe der 3 höchsten Würfe aus einer Liste von 4 Würfen
    int ability(List<Integer> scores) {
        // Kopie erstellen, damit wir die Originalliste nicht verändern
        List<Integer> sortedScores = new ArrayList<>(scores);
        // Sortiert die Zahlen aufsteigend (z.B. 2, 4, 5, 6)
        Collections.sort(sortedScores);
        
        // Die drei höchsten Werte zusammenrechnen (Index 1, 2 und 3)
        return sortedScores.get(1) + sortedScores.get(2) + sortedScores.get(3);
    }

    // Würfelt 4 separate Zahlen zwischen 1 und 6 und gibt sie als Liste zurück
    List<Integer> rollDice() {
        List<Integer> dice = new ArrayList<>();
        for (int i = 0; i < 4; i++) {
            dice.add(random.nextInt(6) + 1);
        }
        return dice;
    }

    // Berechnet den Modifikator (Wert minus 10, durch 2, abgerundet)
    int modifier(int input) {
        return (int) Math.floor((input - 10) / 2.0);
    }

    // Getter-Methoden, um die Werte abzufragen
    int getStrength() {
        return this.strength;
    }

    int getDexterity() {
        return this.dexterity;
    }

    int getConstitution() {
        return this.constitution;
    }

    int getIntelligence() {
        return this.intelligence;
    }

    int getWisdom() {
        return this.wisdom;
    }

    int getCharisma() {
        return this.charisma;
    }

    int getHitpoints() {
        return this.hitpoints;
    }
}
