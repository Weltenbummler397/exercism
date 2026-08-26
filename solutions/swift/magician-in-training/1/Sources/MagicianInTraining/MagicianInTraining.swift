func getCard(at index: Int, from stack: [Int]) -> Int {
  if index >= 0 && index < stack.count {
    return stack[index]
  }
  return 0
}

func setCard(at index: Int, in stack: [Int], to newCard: Int) -> [Int] {
  if index >= 0 && index < stack.count {
    var s = stack
    s[index] = newCard
    return s
  }
  return stack
}

func insert(_ newCard: Int, atTopOf stack: [Int]) -> [Int] {
  var s = stack
  s.append(newCard)
  return s
}

func removeCard(at index: Int, from stack: [Int]) -> [Int] {
  if index >= 0 && index < stack.count {
    var s = stack
    s.remove(at: index)
    return s
  }
  return stack
}

func insert(_ newCard: Int, at index: Int, from stack: [Int]) -> [Int] {
  if index >= 0 && index <= stack.count {
    var s = stack
    s.insert(newCard, at: index)
    return s
  }
  return stack
}

func checkSizeOfStack(_ stack: [Int], _ size: Int) -> Bool {
  return stack.count == size
}
