typealias ChangeClosure = @Sendable ((String, String, String)) -> (String, String, String)

//let flip: ChangeClosure = TODO: Please define the flip closure
let flip: ChangeClosure = { (wires: (String, String, String)) -> (String, String, String) in
            let (a,b,c) = wires 
            return (b, a, c)
           }
//
//
//let rotate: ChangeClosure = TODO: Please define the rotate closure
let rotate: ChangeClosure = { (wires: (String, String, String)) -> (String, String, String) in
              let (a,b,c) = wires   
              return (b, c, a) 
               } 

func makeShuffle(
  flipper: @escaping ((String, String, String)) -> (String, String, String),
  rotator: @escaping ((String, String, String)) -> (String, String, String)
) -> ([UInt8], (String, String, String)) -> (String, String, String) {
  return { (id: [UInt8], wires: (String, String, String)) -> (String, String, String) in
        var currentWires = wires
        for bit in id.reversed() {
            if bit == 0 {
                currentWires = flipper(currentWires)
            } else if bit == 1 {
                currentWires = rotator(currentWires)
            }
        }
        return currentWires
    }
}
