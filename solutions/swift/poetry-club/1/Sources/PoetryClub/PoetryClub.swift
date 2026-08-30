import Foundation

func splitOnNewlines(_ poem: String) -> [String] {
  return poem.components(separatedBy: "\n")
}

func frontDoorPassword(_ phrase: String) -> String {
  let phrase = splitOnNewlines(phrase)
  var result: [String] = []
  for p in phrase {
    if let char = p.first {
      result.append(String(char)) 
    } else {
      result.append("_")
    }
  }
  return result.joined(separator: "")
}

func backDoorPassword(_ phrase: String) -> String {
  let phrase = splitOnNewlines(phrase)
  var result: [String] = []
  for p in phrase {
    let trimmedLine = p.trimmingCharacters(in: .whitespacesAndNewlines)
    if let char = trimmedLine.last {
      result.append(String(char)) 
    } else {
      result.append("_")
    }
  }
  return result.joined(separator: "") + ", please"
}

func secretRoomPassword(_ phrase: String) -> String {
  let lines = splitOnNewlines(phrase)
  var count = 0
  var result: [String] = []
  for line in lines {
        if count < line.count {
            let ind = line.index(line.startIndex, offsetBy: count)
            result.append(String(line[ind]))
        } else {
            result.append("_")
        }
        count += 1
    }
  var x = result.joined(separator: "") + "!"
  return x.uppercased
}
