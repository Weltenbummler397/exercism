public class SalaryCalculator {
    public double salaryMultiplier(int daysSkipped) {
        boolean expr = daysSkipped >= 5;
        return expr ? 0.85 : 1.0;
    }

    public int bonusMultiplier(int productsSold) {
        boolean expr = productsSold >= 20;
        return expr ? 13 : 10;
    }

    public double bonusForProductsSold(int productsSold) {
        return bonusMultiplier(productsSold) * productsSold;
    }

    public double finalSalary(int daysSkipped, int productsSold) {
        double x = 1000.00 * salaryMultiplier(daysSkipped) + bonusForProductsSold(productsSold);
        return x > 2000 ? 2000 : x;
    } 
}
