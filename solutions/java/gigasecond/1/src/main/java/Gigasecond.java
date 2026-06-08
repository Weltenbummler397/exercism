import java.time.LocalDate;
import java.time.LocalDateTime;

public class Gigasecond {
    private final LocalDateTime gigaSecondMoment;
    
    public Gigasecond(LocalDate moment) {
        this.gigaSecondMoment = moment.atStartOfDay().plusSeconds(1_000_000_000);
    }

    public Gigasecond(LocalDateTime moment) {
        this.gigaSecondMoment = moment.plusSeconds(1_000_000_000);
    }

    public LocalDateTime getDateTime() {
        return gigaSecondMoment;
    }
}
