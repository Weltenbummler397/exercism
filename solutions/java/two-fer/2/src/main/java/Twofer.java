public class Twofer {
    public String twofer(String name) {
        String none = "One for you, one for me.";
        String some = "One for " + name + ", one for me.";
        return name == null? none : some;
    }
}
