class MicroBlog {
    public String truncate(String input) {
        if (input.codePointCount(0, input.length()) <= 5) {
            return input;
        }
        
        int endOffset = input.offsetByCodePoints(0, 5);

        return input.substring(0, endOffset);
    }
}
