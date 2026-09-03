// TODO: define the 'LogLevel' enum, its `init`, and its `shortFormat` method
enum LogLevel: String  {
  case trace = "TRC"
    case debug = "DBG"
    case info = "INF"
    case warning = "WRN"
    case error = "ERR"
    case fatal = "FTL"
    case unknown

  init(_ i: String) {
    if let openingBracket = i.firstIndex(of: "["),
           let closingBracket = i.firstIndex(of: "]"),
           openingBracket < closingBracket {

            let start = i.index(after: openingBracket)
            let rawLevel = String(i[start..<closingBracket])
            self = LogLevel(rawValue: rawLevel) ?? .unknown
        } else {
            self = .unknown
        }
    }

  func shortFormat(message: String) -> String {
    return switch self {
      case .trace: "0:\(message)"
      case .debug: "1:\(message)"
      case .info: "4:\(message)"
      case .warning: "5:\(message)"
      case .error: "6:\(message)"
      case .fatal: "7:\(message)"
      case .unknown: "42:\(message)"
    }
  }
}