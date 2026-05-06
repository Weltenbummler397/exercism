import java.util.ArrayList;
import java.util.List;

class ProteinTranslator {

    List<String> translate(String rnaSequence) {
        List<String> result = new ArrayList<>();
        if (rnaSequence == null || rnaSequence.isEmpty()) {
            return result;
        }
        String[] parts = rnaSequence.split("(?<=\\G.{3})");
        for (String part : parts) {
            switch (part) {
                case "AUG" -> result.add("Methionine");
                case "UUU", "UUC" -> result.add("Phenylalanine");
                case "UUA", "UUG" -> result.add("Leucine");
                case "UCU", "UCC", "UCA", "UCG" -> result.add("Serine");
                case "UAU", "UAC" -> result.add("Tyrosine");
                case "UGU", "UGC" -> result.add("Cysteine");
                case "UGG" -> result.add("Tryptophan");
                case "UAA", "UAG", "UGA" -> { return result; }
                default -> throw new IllegalArgumentException("Invalid codon");
            }
        }
        return result;
    }
}
