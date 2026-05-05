class RnaTranscription {

    String transcribe(String dnaStrand) {
        String result = dnaStrand;
        result = result.replace("G", "#");
        result = result.replace("C", "G");
        result = result.replace("#", "C");
        result = result.replace("T", "#");
        result = result.replace("A", "U");
        result = result.replace("#", "A");
        return result;
    }

}
