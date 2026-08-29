func getName(_ item: (name: String, amount: Int)) -> String {
  return item.name
}

func createToy(name: String, amount: Int) -> (name: String, amount: Int) {
  return (name, amount)
}

func updateQuantity(_ items: [(name: String, amount: Int)], toy: String, amount: Int) ->  [(name: String, amount: Int)] {
  var mI = items
  for i in 0..<mI.count {
        if mI[i].name == toy {
            mI[i].amount = amount
        }
    }
  return mI
}

func addCategory(_ items: [(name: String, amount: Int)], category: String) -> [(name: String, amount: Int, category: String)] {
  var a: [(name: String, amount: Int, category: String)] = []   
  for (name, amount) in items {
    a.append((name, amount, category))   
  }
  
  return a
}
