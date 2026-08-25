func dailyRateFrom(hourlyRate: Int) -> Double {
  return 8.0 * Double(hourlyRate)
}

func monthlyRateFrom(hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (dailyRateFrom(hourlyRate:hourlyRate)*22.0*((100.0-discount)/100)).rounded()
}

func workdaysIn(budget: Double, hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (budget/(dailyRateFrom(hourlyRate:hourlyRate) * (100-discount)/100)).rounded(.down)
}
