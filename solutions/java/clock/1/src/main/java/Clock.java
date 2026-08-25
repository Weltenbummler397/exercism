class Clock {
    public int hours;
    public int minutes;
    
    Clock(int hours, int minutes) {
        this.hours = hours;
        this.minutes = minutes;
        while (this.minutes>=60) {
            this.minutes-=60;
            this.hours++;
        }
        this.hours %= 24;
        while (this.minutes<0) {
            this.minutes +=60;
            this.hours -=1;
        }
        while (this.hours<0) {
            this.hours += 24;
        }
    }

    void add(int minutes) {
        this.minutes+=minutes;
        while (this.minutes>=60) {
            this.minutes-=60;
            this.hours++;
        }
        this.hours %= 24;
        while (this.minutes<0) {
            this.minutes +=60;
            this.hours -=1;
        }
        while (this.hours<0) {
            this.hours += 24;
        }
    }

    @Override
    public String toString(){
        return String.format("%02d:%02d", hours, minutes);
    }

    @Override
    public boolean equals(Object obj) {
        if (obj instanceof Clock other) { 
            return this.hours == other.hours && this.minutes == other.minutes;
        }
            return false;
        }

}