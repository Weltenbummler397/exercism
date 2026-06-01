import java.util.Collections;
import java.util.List;
import java.util.ArrayList;

class HighScores {
    List<Integer> scores;
    
    public HighScores(List<Integer> highScores) {
        scores = new ArrayList<>(highScores);
    }

    List<Integer> scores() {
        return scores;
    }

    Integer latest() {
        return scores.get(scores.size()-1);
    }

    Integer personalBest() {
        return Collections.max(scores);
    }

    List<Integer> personalTopThree() {
        List<Integer> cloneList = new ArrayList<>(scores);
        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < 3 && !cloneList.isEmpty(); i++) { 
            Integer max = Collections.max(cloneList);
            result.add(max);
            cloneList.remove(max);
        }
        return result;
    }

}
