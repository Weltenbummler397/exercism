import Foundation

func sliceSize(diameter: Double?, slices: Int?) -> Double? {
  guard let d = diameter, let s = slices else{ return nil}
  if d < 0 {
    return nil
  } else if s < 1 {
    return nil
  } else {
    return ((Double.pi * pow(d,2))/4)/Double(s)
  }
}

func biggestSlice(
  diameterA: String, slicesA: String,
  diameterB: String, slicesB: String
) -> String {
  let dA = Double(diameterA)
  let sA = Int(slicesA)
  let dB = Double(diameterB)
  let sB = Int(slicesB)
  let pi1 = sliceSize(diameter: dA, slices: sA)
  let pi2 = sliceSize(diameter: dB, slices: sB)
  if (pi1 ?? -1.0) < (pi2 ?? -1.0) {
    return "Slice B is bigger"
  } else if (pi2 ?? -1.0) < (pi1 ?? -1.0){
    return "Slice A is bigger"
  } else {
    return "Neither slice is bigger"
  }
}
