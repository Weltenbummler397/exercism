// TODO: define the 'remainingMinutesInOven' function
func remainingMinutesInOven(elapsedMinutes elapsed: Int, expectedMinutesInOven expected: Int = 40) -> Int {
  return expected-elapsed
}
// TODO: define the 'preparationTimeInMinutes' function
func preparationTimeInMinutes(layers: String...) -> Int {
  var sum = 0
  for layer in layers {
    sum += 2
  }
  return sum
}
// TODO: define the 'quantities' function
func quantities(layers: String...) -> (noodles: Int, sauce: Double) {
  var noodels = 0
  var sauce = 0.0
  for layer in layers {
    if layer == "sauce" {
      sauce += 0.2
    } else if layer == "noodles" {
      noodels += 3
    }
  }
  return (noodels, sauce)
}
// TODO: define the 'toOz' function
func toOz(_ input: inout (noodles: Int, sauce: Double)) {
  input.sauce *= 33.814
}
// TODO: define the 'redWine' function
func redWine(layers: String...) -> Bool {
    func countMozzarella() -> Int {
        return layers.filter { $0 == "mozzarella" }.count
    }
    func countRicotta() -> Int {
        return layers.filter { $0 == "ricotta" }.count
    }
    func countBéchamel() -> Int {
        return layers.filter { $0 == "béchamel" }.count
    }
    func countSauce() -> Int {
        return layers.filter { $0 == "sauce" }.count
    }
    func countMeat() -> Int {
        return layers.filter { $0 == "meat" }.count
    }
    let whiteWineLayers = countMozzarella() + countRicotta() + countBéchamel()
    let redWineLayers = countSauce() + countMeat()
    return redWineLayers >= whiteWineLayers
}
