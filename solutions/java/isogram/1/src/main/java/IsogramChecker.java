import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
class IsogramChecker {

    boolean isIsogram(String phrase) {
        ArrayList<String> letters = new ArrayList<>(Arrays.asList(phrase.trim().replace("-", "").replace(" ", "").toUpperCase().split("")));
        ArrayList<String> list = new ArrayList<>();
        for (String letter : letters) {
            if (list.contains(letter)) {
                return false;
            } else {
                list.add(letter);
            }
        }
        return true;
    }

}
