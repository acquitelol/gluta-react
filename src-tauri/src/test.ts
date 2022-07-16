const calcTicket = (day: string, price: number) => {
    if (day !== "Saturday") return "Wait for concert on the weekday...";
    if (price > 100) return "Wait for next concert with cheaper ticket...";
    return "Buy the ticket..."
  }
  
  console.log(calcTicket("Saturday", 100))
  