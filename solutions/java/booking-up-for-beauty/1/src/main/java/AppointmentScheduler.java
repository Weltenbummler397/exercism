import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

class AppointmentScheduler {
    public LocalDateTime schedule(String appointmentDateDescription) {
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("MM/dd/yyyy HH:mm:ss");
        return LocalDateTime.parse(appointmentDateDescription, formatter);
    }

    public boolean hasPassed(LocalDateTime appointmentDate) {
        LocalDateTime now = LocalDateTime.now();
        return now.isAfter(appointmentDate);
    }

    public boolean isAfternoonAppointment(LocalDateTime appointmentDate) {
        int hour = appointmentDate.getHour();
        return hour >= 12 && hour < 18;
    }

    public String getDescription(LocalDateTime appointmentDate) {
        DateTimeFormatter timeFormatter = DateTimeFormatter.ofPattern("h:mm a");
        DateTimeFormatter weekDayFormatter = DateTimeFormatter.ofPattern("EEEE"); // z.B. Monday
        DateTimeFormatter monthFormatter = DateTimeFormatter.ofPattern("MMMM");   // z.B. July
    
        String weekDay = appointmentDate.format(weekDayFormatter);
        int day = appointmentDate.getDayOfMonth();
        String month = appointmentDate.format(monthFormatter);
        int year = appointmentDate.getYear();
        String time = appointmentDate.format(timeFormatter);
    
        return String.format("You have an appointment on %s, %s %d, %d, at %s.", weekDay, month, day, year, time);
    }

    public LocalDate getAnniversaryDate() {
        LocalDateTime now = LocalDateTime.now();
        int year = now.getYear();
        return LocalDate.of(year, 9 , 15);
    }
}
