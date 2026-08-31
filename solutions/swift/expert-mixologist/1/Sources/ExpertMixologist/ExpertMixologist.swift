func timeToPrepare(drinks: [String]) -> Double {
  var sum = 0.0
  for drink in drinks {
    switch drink {
      case "beer", "soda", "water": sum += 0.5
      case "shot": sum += 1.0
      case "mixed drink": sum += 1.5
      case "fancy drink": sum += 2.5
      case "frozen drink": sum += 3.0
      default: sum += 0.0
    }
  }
  return sum
}

func makeWedges(needed: Int, limes: [String]) -> Int {
  var sum = 0
  var count = 0
  while sum < needed && count < limes.count {
    switch limes[count] {
      case "small": sum += 6
      case "medium": sum += 8
      case "large": sum += 10
      default: continue
    }
    count += 1
  }
  return count
}

func finishShift(minutesLeft: Int, remainingOrders: [[String]]) -> [[String]] {
  var min = Double(minutesLeft)
  var count = 0;
  while min > 0.0 && count < remainingOrders.count {
        min -= timeToPrepare(drinks: remainingOrders[count])
        count += 1
    }
  return Array(remainingOrders[count...]) 
}

func orderTracker(orders: [(drink: String, time: String)]) -> (beer: (first: String, last: String, total: Int)?, soda: (first: String, last: String, total: Int)?) {
    let beerOrders = orders.filter { $0.drink == "beer" }
    let sodaOrders = orders.filter { $0.drink == "soda" }

    let beerResult: (first: String, last: String, total: Int)?
    if beerOrders.isEmpty {
        beerResult = nil
    } else {
        beerResult = (
            first: beerOrders[0].time,
            last: beerOrders[beerOrders.count - 1].time,
            total: beerOrders.count
        )
    }
    
    let sodaResult: (first: String, last: String, total: Int)?
    if sodaOrders.isEmpty {
        sodaResult = nil
    } else {
        sodaResult = (
            first: sodaOrders[0].time,
            last: sodaOrders[sodaOrders.count - 1].time,
            total: sodaOrders.count
        )
    }

    return (beer: beerResult, soda: sodaResult)
}

