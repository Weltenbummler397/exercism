public class CarsAssemble {

    public double productionRatePerHour(int speed) {
        if (speed >= 1 && speed <= 4) {
            return speed*221;
        } else if (speed >= 5 && speed <= 8) {
            return speed*221*0.9;
        } else if (speed == 9) {
            return speed*221*0.8;
        } else if (speed == 10) {
            return speed*221*0.77;
        } else {
            return 0;
        }
    }

    public int workingItemsPerMinute(int speed) {
        if (speed >= 1 && speed <= 4) {
            return (int) (speed*221)/60;
        } else if (speed >= 5 && speed <= 8) {
            return (int) (speed*221*0.9)/60;
        } else if (speed == 9) {
            return (int) (speed*221*0.8)/60;
        } else if (speed == 10) {
            return (int) (speed*221*0.77)/60;
        } else {
            return 0;
        }
    }
}
