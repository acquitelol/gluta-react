// Old code
let concertDay = "Saturday"
let concertTicket = 100

if concertDay == "Saturday" {
    if concertTicket < 100 {
        print("Buy the ticket...")
    } else {
        print("Wait for next concert with cheaper ticket...")
    }
} else {
    print("Wait for concert on the weekday...")
}

// New code
func calcTicket(day: String, price: Int) -> String {
    if day != "Saturday" { return "Wait for concert on the weekday..." }
    if price > 100 { return "Wait for next concert with cheaper ticket..." }
    return "Buy the ticket..."
} 

print(calcTicket(day: "Sunday", price: 100))