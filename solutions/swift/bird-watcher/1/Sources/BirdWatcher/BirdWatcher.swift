func totalBirdCount(_ birdsPerDay: [Int]) -> Int {
  var sum = 0
  for i in birdsPerDay {
    sum += i
  }
  return sum
}

func birdsInWeek(_ birdsPerDay: [Int], weekNumber: Int) -> Int {
  var x = (weekNumber-1)*7
  var sum = 0
  for i in x..<x+7 {
    sum += birdsPerDay[i]
  }
  return sum
}

func fixBirdCountLog(_ birdsPerDay: [Int]) -> [Int] {
  var bPD = birdsPerDay
  for i in stride(from: 0, to: birdsPerDay.count, by: 2) {
    bPD[i] += 1
  }
  return bPD
}
