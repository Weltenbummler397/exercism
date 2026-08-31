// TODO: Define the Size struct
struct Size {
  var width = 80
  var height = 60

  mutating func resize(newWidth x : Int, newHeight y : Int) {
    width = x
    height = y
  }
}
// TODO: Define the Position struct
struct Position {
  var x = 0
  var y = 0

  mutating func moveTo(newX a: Int, newY b: Int) {
    x=a
    y=b
  }
}
// TODO: Define the Window class
class Window {
  var title = "New Window"
  let screenSize = Size(width: 800, height: 600)
  var size = Size()
  var position = Position()
  var contents: String? = nil

  func resize(to newSize: Size) {
    let safeWidth = min(max(1, newSize.width), screenSize.width - position.x)
    let safeHeight = min(max(1, newSize.height), screenSize.height - position.y)
    self.size.resize(newWidth: safeWidth, newHeight: safeHeight)
  }
  func move(to pos: Position) -> () {
    let maxX = screenSize.width - size.width
    let safeX = min(max(0, pos.x), maxX)
    let maxY = screenSize.height - size.height
    let safeY = min(max(0, pos.y), maxY)
    self.position.moveTo(newX: safeX, newY: safeY)
  }

  func update (title t: String) -> () {
    title = t
  }
  func update (text t: String?) -> () {
    contents = t
  }
  func display() -> String {
    let contentText = contents ?? "[This window intentionally left blank]"
    return "\(title)\nPosition: (\(position.x), \(position.y)), Size: (\(size.width) x \(size.height))\n\(contentText)\n"
  }

  init(title: String = "New Window", contents: String? = nil, size: Size = Size(), position: Position = Position()) {
    self.title = title
    self.contents = contents
    self.size = size
    self.position = position
    }
}