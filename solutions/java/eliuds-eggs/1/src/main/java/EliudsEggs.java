public class EliudsEggs {
    public int eggCount(int number) {
        String[] chars = Integer.toBinaryString(number).split("");
        int count = 0;

        for (String ch : chars) {
            if (ch.contains("1")) {
                count++;
            }
        }
        return count;
    }
}
