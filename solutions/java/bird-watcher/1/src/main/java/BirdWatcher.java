import java.util.stream.IntStream;

class BirdWatcher {
    private final int[] birdsPerDay;

    public BirdWatcher(int[] birdsPerDay) {
        this.birdsPerDay = birdsPerDay.clone();
    }

    public int[] getLastWeek() {
        return new int[] { 0, 2, 5, 3, 7, 8, 4 };
    }

    public int getToday() {
        return birdsPerDay[birdsPerDay.length -1];
    }

    public void incrementTodaysCount() {
        birdsPerDay[birdsPerDay.length -1] += 1;
    }

    public boolean hasDayWithoutBirds() {
        return IntStream.of(birdsPerDay).anyMatch(x -> x == 0);
    }

    public int getCountForFirstDays(int numberOfDays) {
        int result = 0;
        if (numberOfDays > 7) {
            numberOfDays = 7;
        }
        for (int i =0; i < numberOfDays; i++) {
            result += birdsPerDay[i];
        }
        return result; 
    }

    public int getBusyDays() {
        int count = 0;
        for (int i =0; i < 7; i++) {
            if (birdsPerDay[i] >= 5) {
                count ++;
            }
        }
        return count; 
    }
}
